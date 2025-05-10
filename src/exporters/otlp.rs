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

use crate::errors::MetricsError;
use configs::{Configs, DynamicConfigs};
use opentelemetry::KeyValue;
use opentelemetry_otlp::{MetricExporter, Protocol, WithExportConfig, WithTonicConfig};
use opentelemetry_sdk::{
    metrics::{PeriodicReader, SdkMeterProvider, Temporality},
    Resource,
};
use std::time::Duration;
use tonic::metadata::{Ascii, MetadataKey, MetadataMap};
use tracing::error;

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
/// # Authentication
///
/// The exporter uses the header access key and access key from configuration for
/// authentication with the OpenTelemetry collector.
pub fn install<T>(cfg: &Configs<T>) -> Result<SdkMeterProvider, MetricsError>
where
    T: DynamicConfigs,
{
    let key: MetadataKey<Ascii> = match cfg.trace.header_access_key.clone().parse() {
        Ok(key) => key,
        Err(_) => {
            error!("failure to convert cfg.trace.header_key");
            MetadataKey::<Ascii>::from_bytes("api-key".as_bytes()).unwrap()
        }
    };

    let value = match cfg.trace.access_key.parse() {
        Ok(value) => Ok(value),
        Err(_) => {
            error!("failure to convert cfg.trace.header_value");
            Err(MetricsError::ConversionError)
        }
    }?;

    let mut map = MetadataMap::with_capacity(2);
    map.insert(key, value);

    let exporter = match MetricExporter::builder()
        .with_tonic()
        .with_temporality(Temporality::Delta)
        .with_protocol(Protocol::Grpc)
        .with_timeout(Duration::from_secs(cfg.metric.export_timeout))
        .with_endpoint(cfg.metric.host.clone())
        .with_metadata(map)
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
        .with_interval(Duration::from_secs(cfg.metric.export_interval))
        .build();

    let provider = SdkMeterProvider::builder()
        .with_reader(reader)
        .with_resource(
            Resource::builder()
                .with_service_name(cfg.app.name.clone())
                .with_attribute(KeyValue::new(
                    "service.type",
                    cfg.trace.service_type.clone(),
                ))
                .with_attribute(KeyValue::new("environment", format!("{}", cfg.app.env)))
                .with_attribute(KeyValue::new("library.language", "rust"))
                .build(),
        )
        .build();

    Ok(provider)
}
