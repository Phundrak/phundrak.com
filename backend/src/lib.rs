//! Backend API server for phundrak.com
//!
//! This is a REST API built with the Poem framework that provides:
//! - Health check endpoints
//! - Application metadata endpoints
//! - Contact form submission with email integration

#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![warn(missing_docs)]
#![allow(clippy::unused_async)]

/// Custom middleware implementations
pub mod middleware;
/// API route handlers and endpoints
pub mod route;
/// Application configuration settings
pub mod settings;
/// Application startup and server configuration
pub mod startup;
/// Logging and tracing setup
pub mod telemetry;

type MaybeListener = Option<poem::listener::TcpListener<String>>;

fn prepare(listener: MaybeListener) -> startup::Application {
    dotenvy::dotenv().ok();
    let settings = settings::Settings::new().expect("Failed to read settings");
    if !cfg!(test) {
        let subscriber = telemetry::get_subscriber(settings.debug);
        telemetry::init_subscriber(subscriber);
    }
    tracing::event!(
        target: "backend",
        tracing::Level::DEBUG,
        "Using these settings: {:?}",
        settings
    );
    let application = startup::Application::build(settings, listener);
    tracing::event!(
        target: "backend",
        tracing::Level::INFO,
        "Listening on http://{}:{}/",
        application.host(),
        application.port()
    );
    tracing::event!(
        target: "backend",
        tracing::Level::INFO,
        "Documentation available at http://{}:{}/",
        application.host(),
        application.port()
    );
    application
}

/// Runs the application with the specified TCP listener.
///
/// # Errors
///
/// Returns a `std::io::Error` if the server fails to start or encounters
/// an I/O error during runtime (e.g., port already in use, network issues).
#[cfg(not(tarpaulin_include))]
pub async fn run(listener: MaybeListener) -> Result<(), std::io::Error> {
    let application = prepare(listener);
    application.make_app().run().await
}

#[cfg(test)]
fn make_random_tcp_listener() -> poem::listener::TcpListener<String> {
    let tcp_listener =
        std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind a random TCP listener");
    let port = tcp_listener.local_addr().unwrap().port();
    poem::listener::TcpListener::bind(format!("127.0.0.1:{port}"))
}

#[cfg(test)]
fn get_test_app() -> startup::App {
    let tcp_listener = make_random_tcp_listener();
    prepare(Some(tcp_listener)).make_app().into()
}
