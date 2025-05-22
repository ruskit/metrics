// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! # Stdout Metrics Exporter
//!
//! Provides a standard output exporter for metrics.
//!
//! This module is conditionally compiled when the "stdout" feature is enabled
//! and provides functionality to export metrics to standard output, which is
//! primarily useful for development and debugging purposes.
//!
//! ## Use Cases
//!
//! - Local development: Quickly verify that metrics are being recorded and exported.
//! - Debugging: Inspect metric output without running a collector or Prometheus server.
//!
//! ## Configuration
//!
//! Enable this exporter by building with the `stdout` feature flag:
//!
//! ```sh
//! cargo build --features stdout
//! ```
//!
//! # Example
//!
//! ```rust
//! use metrics::exporters::stdout;
//! let provider = stdout::install().unwrap();
//! ```
//!

use crate::errors::MetricsError;
use configs::app::AppConfigs;
use opentelemetry::{KeyValue, global};
use opentelemetry_sdk::{
    Resource,
    metrics::{PeriodicReader, SdkMeterProvider},
};
use tracing::info;

/// Creates and installs a standard output metrics exporter.
///
/// This function configures and installs a metrics exporter that writes metrics
/// to standard output. This is primarily useful for development and debugging.
///
/// # Returns
///
/// * `Ok(SdkMeterProvider)` - The configured meter provider
/// * `Err(MetricsError)` - If an error occurred during exporter setup
///
/// # Usage
///
/// This exporter is typically used during development to verify that metrics
/// are being recorded correctly before configuring a production-ready exporter
/// like OTLP or Prometheus.
pub fn install() -> Result<SdkMeterProvider, MetricsError> {
    let app_cfgs = AppConfigs::new();

    let exporter = opentelemetry_stdout::MetricExporter::default();
    let reader = PeriodicReader::builder(exporter).build();

    let provider = SdkMeterProvider::builder()
        .with_reader(reader)
        .with_resource(
            Resource::builder()
                .with_service_name(app_cfgs.name.clone())
                .with_attribute(KeyValue::new(
                    "service.namespace",
                    format!("{}", app_cfgs.namespace),
                ))
                .with_attribute(KeyValue::new("environment", format!("{}", app_cfgs.env)))
                .with_attribute(KeyValue::new("library.language", "rust"))
                .build(),
        )
        .build();

    global::set_meter_provider(provider.clone());

    info!("traces::install stdout metric installed");

    Ok(provider)
}
