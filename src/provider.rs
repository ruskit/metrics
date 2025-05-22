// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! # Metrics Provider
//!
//! Provides the main entry point for initializing metrics collection.
//!
//! This module contains the primary function for setting up metrics collection
//! based on application configuration. It handles feature detection and
//! initializes the appropriate exporter based on the available features.
//!
//! The provider automatically selects the appropriate exporter in the following priority:
//!
//! 1. OTLP exporter (when the `otlp` feature is enabled)
//! 2. Stdout exporter (when the `stdout` feature is enabled)
//! 3. No-op exporter (when neither of the above features are enabled)
//!
//! This design allows applications to switch between exporters by simply changing feature flags
//! without modifying application code.

use crate::{errors::MetricsError, exporters};
use opentelemetry_sdk::metrics::SdkMeterProvider;
use tracing::info;

/// Initialize and install the metrics provider based on available features.
///
/// This function sets up the appropriate metrics exporter based on the features enabled
/// during compilation. It automatically configures the following in order of precedence:
///
/// 1. OTLP exporter (when the `otlp` feature is enabled)
/// 2. Stdout exporter (when the `stdout` feature is enabled)
/// 3. No-op exporter (when neither of the above features are enabled)
///
/// The function also configures resource attributes for the metrics including service name,
/// namespace, environment, and library language.
///
/// # Returns
///
/// * `Ok(SdkMeterProvider)` - The configured meter provider that can be used to create meters
/// * `Err(MetricsError)` - If an error occurred during metrics initialization
///
/// # Examples
///
/// ```
/// use metrics::provider;
/// use opentelemetry::metrics::{MeterProvider, Counter};
///
/// fn setup_metrics() -> Result<(), Box<dyn std::error::Error>> {
///     // Install the metrics provider
///     let provider = provider::install()?;
///     
///     // Create a meter for this component
///     let meter = provider.meter("component_name");
///     
///     // Create and use instruments
///     let counter = meter.u64_counter("requests").init();
///     counter.add(1, &[]);
///     
///     Ok(())
/// }
/// ```
///
/// # Feature Selection
///
/// The exporter is selected based on enabled features:
///
/// ```rust,no_run
/// // With OTLP feature:
/// // cargo build --features otlp
///
/// // With stdout feature:
/// // cargo build --features stdout
///
/// // With no specific feature (uses no-op):
/// // cargo build
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
