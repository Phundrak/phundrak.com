//! Backend server entry point.

#[cfg(not(tarpaulin_include))]
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    phundrak_dot_com_backend::run(None).await
}
