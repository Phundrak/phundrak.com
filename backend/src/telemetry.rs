//! Logging and tracing configuration.
//!
//! This module provides utilities for setting up structured logging using the tracing crate.
//! Supports both pretty-printed logs for development and JSON logs for production.

use tracing_subscriber::layer::SubscriberExt;

/// Creates a tracing subscriber configured for the given debug mode.
///
/// In debug mode, logs are pretty-printed to stdout.
/// In production mode, logs are output as JSON.
#[must_use]
pub fn get_subscriber(debug: bool) -> impl tracing::Subscriber + Send + Sync {
    let env_filter = if debug { "debug" } else { "info" }.to_string();
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(env_filter));
    let stdout_log = tracing_subscriber::fmt::layer().pretty();
    let subscriber = tracing_subscriber::Registry::default()
        .with(env_filter)
        .with(stdout_log);
    let json_log = if debug {
        None
    } else {
        Some(tracing_subscriber::fmt::layer().json())
    };
    subscriber.with(json_log)
}

/// Initializes the global tracing subscriber.
///
/// # Panics
///
/// Panics if:
/// - A global subscriber has already been set
/// - The subscriber cannot be set as the global default
pub fn init_subscriber(subscriber: impl tracing::Subscriber + Send + Sync) {
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_subscriber_debug_mode() {
        let subscriber = get_subscriber(true);
        // If we can create the subscriber without panicking, the test passes
        // We can't easily inspect the subscriber's internals, but we can verify it's created
        let _ = subscriber;
    }

    #[test]
    fn get_subscriber_production_mode() {
        let subscriber = get_subscriber(false);
        // If we can create the subscriber without panicking, the test passes
        let _ = subscriber;
    }

    #[test]
    fn get_subscriber_creates_valid_subscriber() {
        // Test both debug and non-debug modes create valid subscribers
        let debug_subscriber = get_subscriber(true);
        let prod_subscriber = get_subscriber(false);

        // Basic smoke test - if these are created without panicking, they're valid
        let _ = debug_subscriber;
        let _ = prod_subscriber;
    }
}
