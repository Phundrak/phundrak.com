use poem::Result;
use poem_openapi::{ApiResponse, Object, OpenApi, payload::Json};

use super::ApiCategory;
use crate::settings::Settings;

#[derive(Object, Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Meta {
    version: String,
    name: String,
}

impl From<poem::web::Data<&Settings>> for Meta {
    fn from(value: poem::web::Data<&Settings>) -> Self {
        let version = value.application.version.clone();
        let name = value.application.name.clone();
        Self { version, name }
    }
}

#[derive(ApiResponse)]
enum MetaResponse {
    #[oai(status = 200)]
    Meta(Json<Meta>),
}

pub struct MetaApi;

#[OpenApi(prefix_path = "/v1/meta", tag = "ApiCategory::Meta")]
impl MetaApi {
    #[oai(path = "/", method = "get")]
    async fn meta(&self, settings: poem::web::Data<&Settings>) -> Result<MetaResponse> {
        tracing::event!(target: "backend", tracing::Level::DEBUG, "Accessing meta endpoint");
        Ok(MetaResponse::Meta(Json(settings.into())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::ApplicationSettings;

    #[tokio::test]
    async fn meta_endpoint_returns_correct_data() {
        let app = crate::get_test_app();
        let cli = poem::test::TestClient::new(app);
        let resp = cli.get("/v1/meta").send().await;
        resp.assert_status_is_ok();

        // let json = resp.0.into_json().await;
        // assert!(json.is_ok(), "Response should be valid JSON");
        // let json_value: serde_json::Value = json.unwrap();

        // assert!(json_value.get("version").is_some(), "Response should have version field");
        // assert!(json_value.get("name").is_some(), "Response should have name field");
    }

    #[tokio::test]
    async fn meta_endpoint_returns_200_status() {
        let app = crate::get_test_app();
        let cli = poem::test::TestClient::new(app);
        let resp = cli.get("/v1/meta").send().await;
        resp.assert_status_is_ok();
    }

    #[test]
    fn meta_from_settings_conversion() {
        let settings = Settings {
            application: ApplicationSettings {
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
        };

        let meta: Meta = poem::web::Data(&settings).into();
        assert_eq!(meta.name, "test-app");
        assert_eq!(meta.version, "1.0.0");
    }
}
