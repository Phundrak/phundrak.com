#[derive(Debug, serde::Deserialize, Clone, Default)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub debug: bool,
    pub email: EmailSettings,
    pub frontend_url: String,
}

impl Settings {
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

#[derive(Debug, serde::Deserialize, Clone, Default)]
pub struct ApplicationSettings {
    pub name: String,
    pub version: String,
    pub port: u16,
    pub host: String,
    pub base_url: String,
    pub protocol: String,
}

#[derive(Debug, PartialEq, Eq, Default)]
pub enum Environment {
    #[default]
    Development,
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

#[derive(Debug, serde::Deserialize, Clone, Default)]
pub struct EmailSettings {
    pub host: String,
    pub user: String,
    pub password: String,
    pub from: String,
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
        assert_eq!(Environment::try_from("development").unwrap(), Environment::Development);
        assert_eq!(Environment::try_from("dev").unwrap(), Environment::Development);
        assert_eq!(Environment::try_from("Development").unwrap(), Environment::Development);
        assert_eq!(Environment::try_from("DEV").unwrap(), Environment::Development);
    }

    #[test]
    fn environment_from_str_production() {
        assert_eq!(Environment::try_from("production").unwrap(), Environment::Production);
        assert_eq!(Environment::try_from("prod").unwrap(), Environment::Production);
        assert_eq!(Environment::try_from("Production").unwrap(), Environment::Production);
        assert_eq!(Environment::try_from("PROD").unwrap(), Environment::Production);
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
}
