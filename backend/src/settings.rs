//! Application configuration settings.
//!
//! This module provides configuration structures that can be loaded from:
//! - YAML configuration files (base.yaml and environment-specific files)
//! - Environment variables (prefixed with APP__)
//!
//! Settings include application details, email server configuration, and environment settings.

/// Application configuration settings.
///
/// Loads configuration from YAML files and environment variables.
#[derive(Debug, serde::Deserialize, Clone, Default)]
pub struct Settings {
    /// Application-specific settings (name, version, host, port, etc.)
    pub application: ApplicationSettings,
    /// Debug mode flag
    pub debug: bool,
    /// Email server configuration for contact form
    pub email: EmailSettings,
    /// Frontend URL for CORS configuration
    pub frontend_url: String,
}

impl Settings {
    /// Creates a new `Settings` instance by loading configuration from files and environment variables.
    ///
    /// # Errors
    ///
    /// Returns a `config::ConfigError` if:
    /// - Configuration files cannot be read or parsed
    /// - Required configuration values are missing
    /// - Configuration values cannot be deserialized into the expected types
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - The current directory cannot be determined
    /// - The `APP_ENVIRONMENT` variable contains an invalid value (not "dev", "development", "prod", or "production")
    pub fn new() -> Result<Self, config::ConfigError> {
        let base_path = std::env::current_dir().expect("Failed to determine the current directory");
        let settings_directory = base_path.join("settings");
        let environment: Environment = std::env::var("APP_ENVIRONMENT")
            .unwrap_or_else(|_| "dev".into())
            .try_into()
            .expect("Failed to parse APP_ENVIRONMENT");
        let environment_filename = format!("{environment}.yaml");
        // Lower = takes precedence
        let settings = config::Config::builder()
            .add_source(config::File::from(settings_directory.join("base.yaml")))
            .add_source(config::File::from(
                settings_directory.join(environment_filename),
            ))
            .add_source(
                config::Environment::with_prefix("APP")
                    .prefix_separator("__")
                    .separator("__"),
            )
            .build()?;
        settings.try_deserialize()
    }
}

/// Application-specific configuration settings.
#[derive(Debug, serde::Deserialize, Clone, Default)]
pub struct ApplicationSettings {
    /// Application name
    pub name: String,
    /// Application version
    pub version: String,
    /// Port to bind to
    pub port: u16,
    /// Host address to bind to
    pub host: String,
    /// Base URL of the application
    pub base_url: String,
    /// Protocol (http or https)
    pub protocol: String,
}

/// Application environment.
#[derive(Debug, PartialEq, Eq, Default)]
pub enum Environment {
    /// Development environment
    #[default]
    Development,
    /// Production environment
    Production,
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let self_str = match self {
            Self::Development => "development",
            Self::Production => "production",
        };
        write!(f, "{self_str}")
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&str> for Environment {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "development" | "dev" => Ok(Self::Development),
            "production" | "prod" => Ok(Self::Production),
            other => Err(format!(
                "{other} is not a supported environment. Use either `development` or `production`"
            )),
        }
    }
}

/// Email server configuration for the contact form.
#[derive(Debug, serde::Deserialize, Clone, Default)]
pub struct EmailSettings {
    /// SMTP server hostname
    pub host: String,
    /// SMTP server port
    pub port: u16,
    /// SMTP authentication username
    pub user: String,
    /// Email address to send from
    pub from: String,
    /// SMTP authentication password
    pub password: String,
    /// Email address to send contact form submissions to
    pub recipient: String,
    /// STARTTLS configuration
    pub starttls: Starttls,
    /// Whether to use implicit TLS (SMTPS)
    pub tls: bool,
}

/// STARTTLS configuration for SMTP connections.
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub enum Starttls {
    /// Never use STARTTLS (unencrypted connection)
    #[default]
    Never,
    /// Use STARTTLS if available (opportunistic encryption)
    Opportunistic,
    /// Always use STARTTLS (required encryption)
    Always,
}

impl TryFrom<&str> for Starttls {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "off" | "no" | "never" => Ok(Self::Never),
            "opportunistic" => Ok(Self::Opportunistic),
            "yes" | "always" => Ok(Self::Always),
            other => Err(format!(
                "{other} is not a supported option. Use either `yes`, `no`, or `opportunistic`"
            )),
        }
    }
}

impl TryFrom<String> for Starttls {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl From<bool> for Starttls {
    fn from(value: bool) -> Self {
        if value { Self::Always } else { Self::Never }
    }
}

impl std::fmt::Display for Starttls {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let self_str = match self {
            Self::Never => "never",
            Self::Opportunistic => "opportunistic",
            Self::Always => "always",
        };
        write!(f, "{self_str}")
    }
}

impl<'de> serde::Deserialize<'de> for Starttls {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StartlsVisitor;

        impl serde::de::Visitor<'_> for StartlsVisitor {
            type Value = Starttls;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string or boolean representing STARTTLS setting (e.g., 'yes', 'no', 'opportunistic', true, false)")
            }

            fn visit_str<E>(self, value: &str) -> Result<Starttls, E>
            where
                E: serde::de::Error,
            {
                Starttls::try_from(value).map_err(E::custom)
            }

            fn visit_string<E>(self, value: String) -> Result<Starttls, E>
            where
                E: serde::de::Error,
            {
                Starttls::try_from(value.as_str()).map_err(E::custom)
            }

            fn visit_bool<E>(self, value: bool) -> Result<Starttls, E>
            where
                E: serde::de::Error,
            {
                Ok(Starttls::from(value))
            }
        }

        deserializer.deserialize_any(StartlsVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn environment_display_development() {
        let env = Environment::Development;
        assert_eq!(env.to_string(), "development");
    }

    #[test]
    fn environment_display_production() {
        let env = Environment::Production;
        assert_eq!(env.to_string(), "production");
    }

    #[test]
    fn environment_from_str_development() {
        assert_eq!(
            Environment::try_from("development").unwrap(),
            Environment::Development
        );
        assert_eq!(
            Environment::try_from("dev").unwrap(),
            Environment::Development
        );
        assert_eq!(
            Environment::try_from("Development").unwrap(),
            Environment::Development
        );
        assert_eq!(
            Environment::try_from("DEV").unwrap(),
            Environment::Development
        );
    }

    #[test]
    fn environment_from_str_production() {
        assert_eq!(
            Environment::try_from("production").unwrap(),
            Environment::Production
        );
        assert_eq!(
            Environment::try_from("prod").unwrap(),
            Environment::Production
        );
        assert_eq!(
            Environment::try_from("Production").unwrap(),
            Environment::Production
        );
        assert_eq!(
            Environment::try_from("PROD").unwrap(),
            Environment::Production
        );
    }

    #[test]
    fn environment_from_str_invalid() {
        let result = Environment::try_from("invalid");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not a supported environment"));
    }

    #[test]
    fn environment_from_string_development() {
        assert_eq!(
            Environment::try_from("development".to_string()).unwrap(),
            Environment::Development
        );
    }

    #[test]
    fn environment_from_string_production() {
        assert_eq!(
            Environment::try_from("production".to_string()).unwrap(),
            Environment::Production
        );
    }

    #[test]
    fn environment_from_string_invalid() {
        let result = Environment::try_from("invalid".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn environment_default_is_development() {
        let env = Environment::default();
        assert_eq!(env, Environment::Development);
    }

    #[test]
    fn startls_deserialize_from_string_never() {
        let json = r#""never""#;
        let result: Starttls = serde_json::from_str(json).unwrap();
        assert_eq!(result, Starttls::Never);

        let json = r#""no""#;
        let result: Starttls = serde_json::from_str(json).unwrap();
        assert_eq!(result, Starttls::Never);

        let json = r#""off""#;
        let result: Starttls = serde_json::from_str(json).unwrap();
        assert_eq!(result, Starttls::Never);
    }

    #[test]
    fn startls_deserialize_from_string_always() {
        let json = r#""always""#;
        let result: Starttls = serde_json::from_str(json).unwrap();
        assert_eq!(result, Starttls::Always);

        let json = r#""yes""#;
        let result: Starttls = serde_json::from_str(json).unwrap();
        assert_eq!(result, Starttls::Always);
    }

    #[test]
    fn startls_deserialize_from_string_opportunistic() {
        let json = r#""opportunistic""#;
        let result: Starttls = serde_json::from_str(json).unwrap();
        assert_eq!(result, Starttls::Opportunistic);
    }

    #[test]
    fn startls_deserialize_from_bool() {
        let json = "true";
        let result: Starttls = serde_json::from_str(json).unwrap();
        assert_eq!(result, Starttls::Always);

        let json = "false";
        let result: Starttls = serde_json::from_str(json).unwrap();
        assert_eq!(result, Starttls::Never);
    }

    #[test]
    fn startls_deserialize_from_string_invalid() {
        let json = r#""invalid""#;
        let result: Result<Starttls, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn startls_default_is_never() {
        let startls = Starttls::default();
        assert_eq!(startls, Starttls::Never);
    }
}
