//! Application startup and server configuration.
//!
//! This module handles:
//! - Building the application with routes and middleware
//! - Setting up the OpenAPI service and Swagger UI
//! - Configuring CORS
//! - Starting the HTTP server

use poem::middleware::{AddDataEndpoint, Cors, CorsEndpoint};
use poem::{EndpointExt, Route};
use poem_openapi::OpenApiService;

use crate::{route::Api, settings::Settings};

type Server = poem::Server<poem::listener::TcpListener<String>, std::convert::Infallible>;
/// The configured application with CORS and settings data.
pub type App = AddDataEndpoint<CorsEndpoint<Route>, Settings>;

/// Application builder that holds the server configuration before running.
pub struct Application {
    server: Server,
    app: poem::Route,
    host: String,
    port: u16,
    settings: Settings,
}

/// A fully configured application ready to run.
pub struct RunnableApplication {
    server: Server,
    app: App,
}

impl RunnableApplication {
    /// Runs the application server.
    ///
    /// # Errors
    ///
    /// Returns a `std::io::Error` if the server fails to start or encounters
    /// an I/O error during runtime (e.g., port already in use, network issues).
    pub async fn run(self) -> Result<(), std::io::Error> {
        self.server.run(self.app).await
    }
}

impl From<RunnableApplication> for App {
    fn from(value: RunnableApplication) -> Self {
        value.app
    }
}

impl From<Application> for RunnableApplication {
    fn from(value: Application) -> Self {
        let app = value.app.with(Cors::new()).data(value.settings);
        let server = value.server;
        Self { server, app }
    }
}

impl Application {
    fn setup_app(settings: &Settings) -> poem::Route {
        let api_service = OpenApiService::new(
            Api::from(settings).apis(),
            settings.application.clone().name,
            settings.application.clone().version,
        )
        .url_prefix("/api");
        let ui = api_service.swagger_ui();
        poem::Route::new()
            .nest("/api", api_service.clone())
            .nest("/specs", api_service.spec_endpoint_yaml())
            .nest("/", ui)
    }

    fn setup_server(
        settings: &Settings,
        tcp_listener: Option<poem::listener::TcpListener<String>>,
    ) -> Server {
        let tcp_listener = tcp_listener.unwrap_or_else(|| {
            let address = format!(
                "{}:{}",
                settings.application.host, settings.application.port
            );
            poem::listener::TcpListener::bind(address)
        });
        poem::Server::new(tcp_listener)
    }

    /// Builds a new application with the given settings and optional TCP listener.
    ///
    /// If no listener is provided, one will be created based on the settings.
    #[must_use]
    pub fn build(
        settings: Settings,
        tcp_listener: Option<poem::listener::TcpListener<String>>,
    ) -> Self {
        let port = settings.application.port;
        let host = settings.application.clone().host;
        let app = Self::setup_app(&settings);
        let server = Self::setup_server(&settings, tcp_listener);
        Self {
            server,
            app,
            host,
            port,
            settings,
        }
    }

    /// Converts the application into a runnable application.
    #[must_use]
    pub fn make_app(self) -> RunnableApplication {
        self.into()
    }

    /// Returns the host address the application is configured to bind to.
    #[must_use]
    pub fn host(&self) -> String {
        self.host.clone()
    }

    /// Returns the port the application is configured to bind to.
    #[must_use]
    pub const fn port(&self) -> u16 {
        self.port
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_settings() -> Settings {
        Settings {
            application: crate::settings::ApplicationSettings {
                name: "test-app".to_string(),
                version: "1.0.0".to_string(),
                port: 8080,
                host: "127.0.0.1".to_string(),
                base_url: "http://localhost:8080".to_string(),
                protocol: "http".to_string(),
            },
            debug: false,
            email: crate::settings::EmailSettings::default(),
            frontend_url: "http://localhost:3000".to_string(),
        }
    }

    #[test]
    fn application_build_and_host() {
        let settings = create_test_settings();
        let app = Application::build(settings.clone(), None);
        assert_eq!(app.host(), settings.application.host);
    }

    #[test]
    fn application_build_and_port() {
        let settings = create_test_settings();
        let app = Application::build(settings, None);
        assert_eq!(app.port(), 8080);
    }

    #[test]
    fn application_host_returns_correct_value() {
        let settings = create_test_settings();
        let app = Application::build(settings, None);
        assert_eq!(app.host(), "127.0.0.1");
    }

    #[test]
    fn application_port_returns_correct_value() {
        let settings = create_test_settings();
        let app = Application::build(settings, None);
        assert_eq!(app.port(), 8080);
    }

    #[test]
    fn application_with_custom_listener() {
        let settings = create_test_settings();
        let tcp_listener =
            std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
        let port = tcp_listener.local_addr().unwrap().port();
        let listener = poem::listener::TcpListener::bind(format!("127.0.0.1:{port}"));

        let app = Application::build(settings, Some(listener));
        assert_eq!(app.host(), "127.0.0.1");
        assert_eq!(app.port(), 8080);
    }
}
