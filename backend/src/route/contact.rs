//! Contact form endpoint for handling user submissions and sending emails.
//!
//! This module provides functionality to:
//! - Validate contact form submissions
//! - Detect spam using honeypot fields
//! - Send emails via SMTP with various TLS configurations

use lettre::{
    Message, SmtpTransport, Transport, message::header::ContentType,
    transport::smtp::authentication::Credentials,
};
use poem_openapi::{ApiResponse, Object, OpenApi, payload::Json};
use validator::Validate;

use super::ApiCategory;
use crate::settings::{EmailSettings, Starttls};

impl TryFrom<&EmailSettings> for SmtpTransport {
    type Error = lettre::transport::smtp::Error;

    fn try_from(settings: &EmailSettings) -> Result<Self, Self::Error> {
        if settings.tls {
            // Implicit TLS (SMTPS) - typically port 465
            tracing::event!(target: "backend::contact", tracing::Level::DEBUG, "Using implicit TLS (SMTPS)");
            let creds = Credentials::new(settings.user.clone(), settings.password.clone());
            Ok(Self::relay(&settings.host)?
                .port(settings.port)
                .credentials(creds)
                .build())
        } else {
            // STARTTLS or no encryption
            match settings.starttls {
                Starttls::Never => {
                    // For local development without TLS
                    tracing::event!(target: "backend::contact", tracing::Level::DEBUG, "Using unencrypted connection");
                    let builder = Self::builder_dangerous(&settings.host).port(settings.port);
                    if settings.user.is_empty() {
                        Ok(builder.build())
                    } else {
                        let creds =
                            Credentials::new(settings.user.clone(), settings.password.clone());
                        Ok(builder.credentials(creds).build())
                    }
                }
                Starttls::Opportunistic | Starttls::Always => {
                    // STARTTLS - typically port 587
                    tracing::event!(target: "backend::contact", tracing::Level::DEBUG, "Using STARTTLS");
                    let creds = Credentials::new(settings.user.clone(), settings.password.clone());
                    Ok(Self::starttls_relay(&settings.host)?
                        .port(settings.port)
                        .credentials(creds)
                        .build())
                }
            }
        }
    }
}

#[derive(Debug, Object, Validate)]
struct ContactRequest {
    #[validate(length(
        min = 1,
        max = "100",
        message = "Name must be between 1 and 100 characters"
    ))]
    name: String,
    #[validate(email(message = "Invalid email address"))]
    email: String,
    #[validate(length(
        min = 10,
        max = 5000,
        message = "Message must be between 10 and 5000 characters"
    ))]
    message: String,
    /// Honeypot field - should always be empty
    #[oai(rename = "website")]
    honeypot: Option<String>,
}

#[derive(Debug, Object, serde::Deserialize)]
struct ContactResponse {
    success: bool,
    message: String,
}

impl From<ContactResponse> for Json<ContactResponse> {
    fn from(value: ContactResponse) -> Self {
        Self(value)
    }
}

#[derive(ApiResponse)]
enum ContactApiResponse {
    /// Success
    #[oai(status = 200)]
    Ok(Json<ContactResponse>),
    /// Bad Request - validation failed
    #[oai(status = 400)]
    BadRequest(Json<ContactResponse>),
    /// Too Many Requests - rate limit exceeded
    #[oai(status = 429)]
    #[allow(dead_code)]
    TooManyRequests,
    /// Internal Server Error
    #[oai(status = 500)]
    InternalServerError(Json<ContactResponse>),
}

/// API for handling contact form submissions and sending emails.
#[derive(Clone)]
pub struct ContactApi {
    settings: EmailSettings,
}

impl From<EmailSettings> for ContactApi {
    fn from(settings: EmailSettings) -> Self {
        Self { settings }
    }
}

#[OpenApi(tag = "ApiCategory::Contact")]
impl ContactApi {
    /// Submit a contact form
    ///
    /// Send a message through the contact form. Rate limited to prevent spam.
    #[oai(path = "/contact", method = "post")]
    async fn submit_contact(
        &self,
        body: Json<ContactRequest>,
        remote_addr: Option<poem::web::Data<&poem::web::RemoteAddr>>,
    ) -> ContactApiResponse {
        let body = body.0;
        if body.honeypot.is_some() {
            tracing::event!(target: "backend::contact", tracing::Level::INFO, "Honeypot triggered, rejecting request silently. IP: {}", remote_addr.map_or_else(|| "No remote address found".to_owned(), |ip| ip.0.to_string()));
            return ContactApiResponse::Ok(
                ContactResponse {
                    success: true,
                    message: "Message sent successfully, but not really, you bot".to_owned(),
                }
                .into(),
            );
        }
        if let Err(e) = body.validate() {
            return ContactApiResponse::BadRequest(
                ContactResponse {
                    success: false,
                    message: format!("Validation error: {e}"),
                }
                .into(),
            );
        }
        match self.send_email(&body).await {
            Ok(()) => {
                tracing::event!(target: "backend::contact", tracing::Level::INFO, "Message sent successfully from: {}", body.email);
                ContactApiResponse::Ok(
                    ContactResponse {
                        success: true,
                        message: "Message sent successfully".to_owned(),
                    }
                    .into(),
                )
            }
            Err(e) => {
                tracing::event!(target: "backend::contact", tracing::Level::ERROR, "Failed to send email: {}", e);
                ContactApiResponse::InternalServerError(
                    ContactResponse {
                        success: false,
                        message: "Failed to send message. Please try again later.".to_owned(),
                    }
                    .into(),
                )
            }
        }
    }

    async fn send_email(&self, request: &ContactRequest) -> Result<(), Box<dyn std::error::Error>> {
        let email_body = format!(
            r"New contact form submission:

Name: {}
Email: {},

Message:
{}",
            request.name, request.email, request.message
        );
        tracing::event!(target: "email", tracing::Level::DEBUG, "Sending email content: {}", email_body);
        let email = Message::builder()
            .from(self.settings.from.parse()?)
            .reply_to(format!("{} <{}>", request.name, request.email).parse()?)
            .to(self.settings.recipient.parse()?)
            .subject(format!("Contact Form: {}", request.name))
            .header(ContentType::TEXT_PLAIN)
            .body(email_body)?;
        tracing::event!(target: "email", tracing::Level::DEBUG, "Email to be sent: {}", format!("{email:?}"));

        let mailer = SmtpTransport::try_from(&self.settings)?;
        mailer.send(&email)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for ContactRequest validation
    #[test]
    fn contact_request_valid() {
        let request = ContactRequest {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            message: "This is a test message that is long enough.".to_string(),
            honeypot: None,
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn contact_request_name_too_short() {
        let request = ContactRequest {
            name: String::new(),
            email: "john@example.com".to_string(),
            message: "This is a test message that is long enough.".to_string(),
            honeypot: None,
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn contact_request_name_too_long() {
        let request = ContactRequest {
            name: "a".repeat(101),
            email: "john@example.com".to_string(),
            message: "This is a test message that is long enough.".to_string(),
            honeypot: None,
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn contact_request_name_at_max_length() {
        let request = ContactRequest {
            name: "a".repeat(100),
            email: "john@example.com".to_string(),
            message: "This is a test message that is long enough.".to_string(),
            honeypot: None,
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn contact_request_invalid_email() {
        let request = ContactRequest {
            name: "John Doe".to_string(),
            email: "not-an-email".to_string(),
            message: "This is a test message that is long enough.".to_string(),
            honeypot: None,
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn contact_request_message_too_short() {
        let request = ContactRequest {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            message: "Short".to_string(),
            honeypot: None,
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn contact_request_message_too_long() {
        let request = ContactRequest {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            message: "a".repeat(5001),
            honeypot: None,
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn contact_request_message_at_min_length() {
        let request = ContactRequest {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            message: "a".repeat(10),
            honeypot: None,
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn contact_request_message_at_max_length() {
        let request = ContactRequest {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            message: "a".repeat(5000),
            honeypot: None,
        };
        assert!(request.validate().is_ok());
    }

    // Tests for SmtpTransport TryFrom implementation
    #[test]
    fn smtp_transport_implicit_tls() {
        let settings = EmailSettings {
            host: "smtp.example.com".to_string(),
            port: 465,
            user: "user@example.com".to_string(),
            password: "password".to_string(),
            from: "from@example.com".to_string(),
            recipient: "to@example.com".to_string(),
            tls: true,
            starttls: Starttls::Never,
        };

        let result = SmtpTransport::try_from(&settings);
        assert!(result.is_ok());
    }

    #[test]
    fn smtp_transport_starttls_always() {
        let settings = EmailSettings {
            host: "smtp.example.com".to_string(),
            port: 587,
            user: "user@example.com".to_string(),
            password: "password".to_string(),
            from: "from@example.com".to_string(),
            recipient: "to@example.com".to_string(),
            tls: false,
            starttls: Starttls::Always,
        };

        let result = SmtpTransport::try_from(&settings);
        assert!(result.is_ok());
    }

    #[test]
    fn smtp_transport_starttls_opportunistic() {
        let settings = EmailSettings {
            host: "smtp.example.com".to_string(),
            port: 587,
            user: "user@example.com".to_string(),
            password: "password".to_string(),
            from: "from@example.com".to_string(),
            recipient: "to@example.com".to_string(),
            tls: false,
            starttls: Starttls::Opportunistic,
        };

        let result = SmtpTransport::try_from(&settings);
        assert!(result.is_ok());
    }

    #[test]
    fn smtp_transport_no_encryption_with_credentials() {
        let settings = EmailSettings {
            host: "localhost".to_string(),
            port: 1025,
            user: "user@example.com".to_string(),
            password: "password".to_string(),
            from: "from@example.com".to_string(),
            recipient: "to@example.com".to_string(),
            tls: false,
            starttls: Starttls::Never,
        };

        let result = SmtpTransport::try_from(&settings);
        assert!(result.is_ok());
    }

    #[test]
    fn smtp_transport_no_encryption_no_credentials() {
        let settings = EmailSettings {
            host: "localhost".to_string(),
            port: 1025,
            user: String::new(),
            password: String::new(),
            from: "from@example.com".to_string(),
            recipient: "to@example.com".to_string(),
            tls: false,
            starttls: Starttls::Never,
        };

        let result = SmtpTransport::try_from(&settings);
        assert!(result.is_ok());
    }

    // Integration tests for contact API endpoint
    #[tokio::test]
    async fn contact_endpoint_honeypot_triggered() {
        let app = crate::get_test_app();
        let cli = poem::test::TestClient::new(app);

        let body = serde_json::json!({
            "name": "Bot Name",
            "email": "bot@example.com",
            "message": "This is a spam message from a bot.",
            "website": "http://spam.com"
        });

        let resp = cli.post("/api/contact").body_json(&body).send().await;
        resp.assert_status_is_ok();

        let json_text = resp.0.into_body().into_string().await.unwrap();
        let json: ContactResponse = serde_json::from_str(&json_text).unwrap();
        assert!(json.success);
        assert!(json.message.contains("not really"));
    }

    #[tokio::test]
    async fn contact_endpoint_validation_error_empty_name() {
        let app = crate::get_test_app();
        let cli = poem::test::TestClient::new(app);

        let body = serde_json::json!({
            "name": "",
            "email": "test@example.com",
            "message": "This is a valid message that is long enough."
        });

        let resp = cli.post("/api/contact").body_json(&body).send().await;
        resp.assert_status(poem::http::StatusCode::BAD_REQUEST);

        let json_text = resp.0.into_body().into_string().await.unwrap();
        let json: ContactResponse = serde_json::from_str(&json_text).unwrap();
        assert!(!json.success);
        assert!(json.message.contains("Validation error"));
    }

    #[tokio::test]
    async fn contact_endpoint_validation_error_invalid_email() {
        let app = crate::get_test_app();
        let cli = poem::test::TestClient::new(app);

        let body = serde_json::json!({
            "name": "Test User",
            "email": "not-an-email",
            "message": "This is a valid message that is long enough."
        });

        let resp = cli.post("/api/contact").body_json(&body).send().await;
        resp.assert_status(poem::http::StatusCode::BAD_REQUEST);

        let json_text = resp.0.into_body().into_string().await.unwrap();
        let json: ContactResponse = serde_json::from_str(&json_text).unwrap();
        assert!(!json.success);
        assert!(json.message.contains("Validation error"));
    }

    #[tokio::test]
    async fn contact_endpoint_validation_error_message_too_short() {
        let app = crate::get_test_app();
        let cli = poem::test::TestClient::new(app);

        let body = serde_json::json!({
            "name": "Test User",
            "email": "test@example.com",
            "message": "Short"
        });

        let resp = cli.post("/api/contact").body_json(&body).send().await;
        resp.assert_status(poem::http::StatusCode::BAD_REQUEST);

        let json_text = resp.0.into_body().into_string().await.unwrap();
        let json: ContactResponse = serde_json::from_str(&json_text).unwrap();
        assert!(!json.success);
        assert!(json.message.contains("Validation error"));
    }

    #[tokio::test]
    async fn contact_endpoint_validation_error_name_too_long() {
        let app = crate::get_test_app();
        let cli = poem::test::TestClient::new(app);

        let body = serde_json::json!({
            "name": "a".repeat(101),
            "email": "test@example.com",
            "message": "This is a valid message that is long enough."
        });

        let resp = cli.post("/api/contact").body_json(&body).send().await;
        resp.assert_status(poem::http::StatusCode::BAD_REQUEST);

        let json_text = resp.0.into_body().into_string().await.unwrap();
        let json: ContactResponse = serde_json::from_str(&json_text).unwrap();
        assert!(!json.success);
        assert!(json.message.contains("Validation error"));
    }

    #[tokio::test]
    async fn contact_endpoint_validation_error_message_too_long() {
        let app = crate::get_test_app();
        let cli = poem::test::TestClient::new(app);

        let body = serde_json::json!({
            "name": "Test User",
            "email": "test@example.com",
            "message": "a".repeat(5001)
        });

        let resp = cli.post("/api/contact").body_json(&body).send().await;
        resp.assert_status(poem::http::StatusCode::BAD_REQUEST);

        let json_text = resp.0.into_body().into_string().await.unwrap();
        let json: ContactResponse = serde_json::from_str(&json_text).unwrap();
        assert!(!json.success);
        assert!(json.message.contains("Validation error"));
    }
}
