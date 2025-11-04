//! Application metadata endpoint for retrieving version and name information.

use poem::Result;
use poem_openapi::{ApiResponse, Object, OpenApi, payload::Json};

use super::ApiCategory;
use crate::settings::ApplicationSettings;

#[derive(Object, Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Meta {
    version: String,
    name: String,
}

impl From<&MetaApi> for Meta {
    fn from(value: &MetaApi) -> Self {
        let version = value.version.clone();
        let name = value.name.clone();
        Self { version, name }
    }
}

#[derive(ApiResponse)]
enum MetaResponse {
    #[oai(status = 200)]
    Meta(Json<Meta>),
}

/// API for retrieving application metadata (name and version).
#[derive(Clone)]
pub struct MetaApi {
    name: String,
    version: String,
}

impl From<&ApplicationSettings> for MetaApi {
    fn from(value: &ApplicationSettings) -> Self {
        let name = value.name.clone();
        let version = value.version.clone();
        Self { name, version }
    }
}

#[OpenApi(tag = "ApiCategory::Meta")]
impl MetaApi {
    #[oai(path = "/meta", method = "get")]
    async fn meta(&self) -> Result<MetaResponse> {
        tracing::event!(target: "backend::meta", tracing::Level::DEBUG, "Accessing meta endpoint");
        Ok(MetaResponse::Meta(Json(self.into())))
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn meta_endpoint_returns_correct_data() {
        let app = crate::get_test_app();
        let cli = poem::test::TestClient::new(app);
        let resp = cli.get("/api/meta").send().await;
        resp.assert_status_is_ok();

        let json_value: serde_json::Value = resp.json().await.value().deserialize();

        assert!(
            json_value.get("version").is_some(),
            "Response should have version field"
        );
        assert!(
            json_value.get("name").is_some(),
            "Response should have name field"
        );
    }

    #[tokio::test]
    async fn meta_endpoint_returns_200_status() {
        let app = crate::get_test_app();
        let cli = poem::test::TestClient::new(app);
        let resp = cli.get("/api/meta").send().await;
        resp.assert_status_is_ok();
    }
}
