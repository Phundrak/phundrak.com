//! Health check endpoint for monitoring service availability.

use poem_openapi::{ApiResponse, OpenApi};

use super::ApiCategory;

#[derive(ApiResponse)]
enum HealthResponse {
    #[oai(status = 200)]
    Ok,
}

/// Health check API for monitoring service availability.
#[derive(Default, Clone)]
pub struct HealthApi;

#[OpenApi(tag = "ApiCategory::Health")]
impl HealthApi {
    #[oai(path = "/health", method = "get")]
    async fn ping(&self) -> HealthResponse {
        tracing::event!(target: "backend::health", tracing::Level::DEBUG, "Accessing health-check endpoint");
        HealthResponse::Ok
    }
}

#[tokio::test]
async fn health_check_works() {
    let app = crate::get_test_app();
    let cli = poem::test::TestClient::new(app);
    let resp = cli.get("/api/health").send().await;
    resp.assert_status_is_ok();
    resp.assert_text("").await;
}
