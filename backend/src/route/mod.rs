use poem_openapi::{OpenApi, Tags};

mod health;
pub use health::HealthApi;

mod meta;
pub use meta::MetaApi;

#[derive(Tags)]
enum ApiCategory {
    Health,
    Meta
}

pub(crate) struct Api;

#[OpenApi]
impl Api {}
