use poem_openapi::{ApiResponse, OpenApi};

use super::ApiCategory;

#[derive(ApiResponse)]
enum HealthResponse {
    #[oai(status = 200)]
    Ok,
}

pub struct HealthApi;

#[OpenApi(prefix_path = "/v1/health-check", tag = "ApiCategory::Health")]
impl HealthApi {
    #[oai(path = "/", method = "get")]
    async fn ping(&self) -> HealthResponse {
        tracing::event!(target: "backend", tracing::Level::DEBUG, "Accessing health-check endpoint");
        HealthResponse::Ok
    }
}

#[tokio::test]
async fn health_check_works() {
    let app = crate::get_test_app();
    let cli = poem::test::TestClient::new(app);
    let resp = cli.get("/v1/health-check").send().await;
    resp.assert_status_is_ok();
    resp.assert_text("").await;
}
