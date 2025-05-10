// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! # Prometheus Metrics Exporter
//!
//! Provides a Prometheus exporter for metrics.
//!
//! This module is conditionally compiled when the "prometheus" feature is enabled
//! and provides functionality to expose metrics in Prometheus format that can be
//! scraped by a Prometheus server via HTTP.

use crate::errors::MetricsError;
use configs::{Configs, DynamicConfigs};
use opentelemetry::KeyValue;
use opentelemetry_sdk::{
    metrics::{MeterProviderBuilder, SdkMeterProvider},
    Resource,
};
use prometheus::Registry;
use std::sync::Arc;
use tracing::error;

/// Creates and installs a Prometheus metrics exporter.
///
/// This function configures and installs a Prometheus metrics exporter based on the
/// application configuration. It returns both the meter provider and the Prometheus
/// registry, which can be used to expose the metrics via an HTTP endpoint.
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
/// * `Ok((SdkMeterProvider, Arc<Registry>))` - The configured meter provider and Prometheus registry
/// * `Err(MetricsError)` - If an error occurred during exporter setup
///
/// # Usage
///
/// The returned Prometheus registry can be used with a web framework like Actix or Warp
/// to expose metrics at an HTTP endpoint (typically /metrics) that Prometheus can scrape.
pub fn install<T>(cfg: &Configs<T>) -> Result<(SdkMeterProvider, Arc<Registry>), MetricsError>
where
    T: DynamicConfigs,
{
    let registry = Registry::new();

    let exporter = match opentelemetry_prometheus::exporter()
        .with_registry(registry.clone())
        .build()
    {
        Ok(e) => Ok(e),
        Err(err) => {
            error!(error = err.to_string(), "failure to create prom exporter");
            Err(MetricsError::ExporterProviderError)
        }
    }?;

    let provider = MeterProviderBuilder::default()
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
        .with_reader(exporter)
        .build();

    Ok((provider, Arc::new(registry)))
}
