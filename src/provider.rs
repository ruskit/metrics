// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! # Metrics Provider
//!
//! Provides the main entry point for initializing metrics collection.
//!
//! This module contains the primary function for setting up metrics collection
//! based on application configuration. It handles feature detection and
//! initializes the appropriate exporter based on configuration.

use crate::{errors::MetricsError, exporters};
use opentelemetry_sdk::metrics::SdkMeterProvider;
use tracing::info;

/// Initialize metrics collection based on application configuration.
///
/// This function examines the application's metrics configuration and sets up
/// the appropriate metrics exporter if metrics are enabled. For Prometheus exporter,
/// it also returns the Prometheus Registry that can be used to expose metrics via HTTP.
///
/// # Type Parameters
///
/// * `T` - A type implementing `DynamicConfigs` for application-specific configuration
///
/// # Parameters
///
/// * `cfg` - The application configuration containing metrics settings
///
/// # Returns
///
/// * `Ok(Some(Arc<Registry>))` - If Prometheus metrics are enabled, returns the Prometheus registry
/// * `Ok(None)` - If metrics are enabled but not using Prometheus, or if metrics are disabled
/// * `Err(MetricsError)` - If an error occurred during metrics initialization
///
/// # Examples
///
/// ```
/// use configs::{Configs, Empty};
/// use metrics::provider;
///
/// fn setup_metrics() -> Result<(), Box<dyn std::error::Error>> {
///     let configs = Configs::<Empty>::default();
///     let registry = provider::init(&configs)?;
///     // Use registry if Prometheus exporter is enabled
///     Ok(())
/// }
/// ```
pub fn install() -> Result<SdkMeterProvider, MetricsError> {
    info!("metrics::install configure metrics...");

    #[cfg(feature = "otlp")]
    {
        let meter = exporters::otlp_grpc::install()?;
        Ok(meter)
    }

    #[cfg(feature = "stdout")]
    {
        let meter = exporters::stdout::install()?;
        Ok(meter)
    }

    #[cfg(not(any(feature = "stdout", feature = "otlp")))]
    return exporters::noop::install();
}
