// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! # OTLP Metrics Exporter
//!
//! Provides an OpenTelemetry Protocol (OTLP) exporter for metrics.
//!
//! This module is conditionally compiled when the "otlp" feature is enabled
//! and provides functionality to export metrics to an OpenTelemetry collector
//! using the OTLP protocol over gRPC.
//!
//! ## Use Cases
//!
//! - Production deployments: Export metrics to a centralized OpenTelemetry collector.
//! - Cloud-native environments: Integrate with observability pipelines using OTLP.
//!
//! ## Configuration
//!
//! Enable this exporter by building with the `otlp` feature flag:
//!
//! ```sh
//! cargo build --features otlp
//! ```
//!
//! ## Authentication
//!
//! The exporter uses the header access key and access key from configuration for
//! authentication with the OpenTelemetry collector.

use crate::errors::MetricsError;
use configs::{app::AppConfigs, otlp::OTLPConfigs};
use opentelemetry::{KeyValue, global};
use opentelemetry_otlp::{
    Compression, MetricExporter, Protocol, WithExportConfig, WithTonicConfig,
};
use opentelemetry_sdk::{
    Resource,
    metrics::{PeriodicReader, SdkMeterProvider},
};
use tracing::{error, info};

/// Creates and installs an OTLP metrics exporter.
///
/// This function configures and installs an OpenTelemetry Protocol (OTLP) metrics
/// exporter based on the application configuration. The exporter sends metrics
/// to an OpenTelemetry collector via gRPC with proper authentication headers.
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
/// * `Ok(SdkMeterProvider)` - The configured meter provider
/// * `Err(MetricsError)` - If an error occurred during exporter setup
///
/// # Configuration
///
/// The OTLP exporter is configured using the application's OTLP settings, including endpoint, timeout, and authentication headers. See the `OTLPConfigs` struct for details.
///
/// # Example
///
/// ```rust
/// use metrics::exporters::otlp_grpc;
/// let provider = otlp_grpc::install().unwrap();
/// ```
///
pub fn install() -> Result<SdkMeterProvider, MetricsError> {
    let app_cfgs = AppConfigs::new();
    let otlp_cfgs = OTLPConfigs::new();

    let exporter = match MetricExporter::builder()
        .with_tonic()
        .with_protocol(Protocol::Grpc)
        .with_timeout(otlp_cfgs.exporter_timeout)
        .with_endpoint(&otlp_cfgs.endpoint)
        .with_compression(Compression::Gzip)
        .build()
    {
        Ok(p) => Ok(p),
        Err(err) => {
            error!(
                error = err.to_string(),
                "failure to create exporter provider"
            );
            Err(MetricsError::ExporterProviderError)
        }
    }?;

    let reader = PeriodicReader::builder(exporter)
        .with_interval(otlp_cfgs.exporter_interval)
        .build();

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

    info!("traces::install otlp metric installed");

    Ok(provider)
}
