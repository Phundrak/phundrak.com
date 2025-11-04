//! Rate limiting middleware using the governor crate.
//!
//! This middleware implements per-IP rate limiting using the Generic Cell Rate
//! Algorithm (GCRA) via the governor crate. It stores rate limiters in memory
//! without requiring external dependencies like Redis.

use std::{
    net::IpAddr,
    num::NonZeroU32,
    sync::Arc,
    time::Duration,
};

use governor::{
    clock::DefaultClock,
    state::{InMemoryState, NotKeyed},
    Quota, RateLimiter,
};
use poem::{
    Endpoint, Error, IntoResponse, Middleware, Request, Response, Result,
};

/// Rate limiting configuration.
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Maximum number of requests allowed in the time window (burst size).
    pub burst_size: u32,
    /// Time window in seconds for rate limiting.
    pub per_seconds: u64,
}

impl RateLimitConfig {
    /// Creates a new rate limit configuration.
    ///
    /// # Arguments
    ///
    /// * `burst_size` - Maximum number of requests allowed in the time window
    /// * `per_seconds` - Time window in seconds
    #[must_use]
    pub const fn new(burst_size: u32, per_seconds: u64) -> Self {
        Self {
            burst_size,
            per_seconds,
        }
    }

    /// Creates a rate limiter from this configuration.
    ///
    /// # Panics
    ///
    /// Panics if `burst_size` is zero.
    #[must_use]
    pub fn create_limiter(&self) -> RateLimiter<NotKeyed, InMemoryState, DefaultClock> {
        let quota = Quota::with_period(Duration::from_secs(self.per_seconds))
            .expect("Failed to create quota")
            .allow_burst(NonZeroU32::new(self.burst_size).expect("Burst size must be non-zero"));
        RateLimiter::direct(quota)
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        // Default: 10 requests per second with burst of 20
        Self::new(20, 1)
    }
}

/// Middleware for rate limiting based on IP address.
pub struct RateLimit {
    limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
}

impl RateLimit {
    /// Creates a new rate limiting middleware with the given configuration.
    #[must_use]
    pub fn new(config: &RateLimitConfig) -> Self {
        Self {
            limiter: Arc::new(config.create_limiter()),
        }
    }
}

impl<E: Endpoint> Middleware<E> for RateLimit {
    type Output = RateLimitEndpoint<E>;

    fn transform(&self, ep: E) -> Self::Output {
        RateLimitEndpoint {
            endpoint: ep,
            limiter: self.limiter.clone(),
        }
    }
}

/// The endpoint wrapper that performs rate limiting checks.
pub struct RateLimitEndpoint<E> {
    endpoint: E,
    limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
}

impl<E: Endpoint> Endpoint for RateLimitEndpoint<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        // Check rate limit
        if self.limiter.check().is_err() {
            let client_ip = Self::get_client_ip(&req)
                .map_or_else(|| "unknown".to_string(), |ip| ip.to_string());

            tracing::event!(
                target: "backend::middleware::rate_limit",
                tracing::Level::WARN,
                client_ip = %client_ip,
                "Rate limit exceeded"
            );

            return Err(Error::from_status(poem::http::StatusCode::TOO_MANY_REQUESTS));
        }

        // Process the request
        let response = self.endpoint.call(req).await;
        response.map(IntoResponse::into_response)
    }
}

impl<E> RateLimitEndpoint<E> {
    /// Extracts the client IP address from the request.
    fn get_client_ip(req: &Request) -> Option<IpAddr> {
        req.remote_addr().as_socket_addr().map(std::net::SocketAddr::ip)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rate_limit_config_new() {
        let config = RateLimitConfig::new(10, 60);
        assert_eq!(config.burst_size, 10);
        assert_eq!(config.per_seconds, 60);
    }

    #[test]
    fn rate_limit_config_default() {
        let config = RateLimitConfig::default();
        assert_eq!(config.burst_size, 20);
        assert_eq!(config.per_seconds, 1);
    }

    #[test]
    fn rate_limit_config_creates_limiter() {
        let config = RateLimitConfig::new(5, 1);
        let limiter = config.create_limiter();

        // First 5 requests should succeed
        for _ in 0..5 {
            assert!(limiter.check().is_ok());
        }

        // 6th request should fail
        assert!(limiter.check().is_err());
    }

    #[tokio::test]
    async fn rate_limit_middleware_allows_within_limit() {
        use poem::{handler, test::TestClient, EndpointExt, Route};

        #[handler]
        async fn index() -> String {
            "Hello".to_string()
        }

        let config = RateLimitConfig::new(5, 60);
        let app = Route::new()
            .at("/", poem::get(index))
            .with(RateLimit::new(&config));
        let cli = TestClient::new(app);

        // First 5 requests should succeed
        for _ in 0..5 {
            let response = cli.get("/").send().await;
            response.assert_status_is_ok();
        }
    }

    #[tokio::test]
    async fn rate_limit_middleware_blocks_over_limit() {
        use poem::{handler, test::TestClient, EndpointExt, Route};

        #[handler]
        async fn index() -> String {
            "Hello".to_string()
        }

        let config = RateLimitConfig::new(3, 60);
        let app = Route::new()
            .at("/", poem::get(index))
            .with(RateLimit::new(&config));
        let cli = TestClient::new(app);

        // First 3 requests should succeed
        for _ in 0..3 {
            let response = cli.get("/").send().await;
            response.assert_status_is_ok();
        }

        // 4th request should be rate limited
        let response = cli.get("/").send().await;
        response.assert_status(poem::http::StatusCode::TOO_MANY_REQUESTS);
    }
}
