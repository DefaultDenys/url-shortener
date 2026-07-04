use std::sync::Arc;

use axum::body::Body;
use governor::middleware::NoOpMiddleware;
use tower_governor::{
    GovernorLayer,
    governor::GovernorConfigBuilder,
    key_extractor::SmartIpKeyExtractor,
};

type RateLimitLayer = GovernorLayer<SmartIpKeyExtractor, NoOpMiddleware, Body>;

fn build_layer(per_second: u64, burst_size: u32) -> RateLimitLayer {
    let mut config = GovernorConfigBuilder::default().key_extractor(SmartIpKeyExtractor);
    config.per_second(per_second);
    config.burst_size(burst_size);

    GovernorLayer::new(Arc::new(
        config
            .finish()
            .expect("valid rate limit configuration"),
    ))
}

/// ~20 requests/min sustained; burst of 10 for shorten spam protection.
pub fn shorten_layer() -> RateLimitLayer {
    build_layer(3, 10)
}

/// ~60 requests/min for stats polling.
pub fn stats_layer() -> RateLimitLayer {
    build_layer(1, 15)
}

/// ~120 requests/min for redirects.
pub fn redirect_layer() -> RateLimitLayer {
    build_layer(1, 30)
}