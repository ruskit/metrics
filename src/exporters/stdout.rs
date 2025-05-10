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

use crate::errors::MetricsError;
use opentelemetry_sdk::metrics::{MeterProviderBuilder, PeriodicReader, SdkMeterProvider};

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
    let exporter = opentelemetry_stdout::MetricExporter::default();

    let reader = PeriodicReader::builder(exporter).build();

    Ok(MeterProviderBuilder::default().with_reader(reader).build())
}
