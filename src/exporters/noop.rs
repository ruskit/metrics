// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! # No-op Metrics Exporter
//!
//! Provides a no-operation exporter for metrics when no other exporter is enabled.
//!
//! This module serves as a fallback when neither "stdout" nor "otlp" features are enabled.
//! It provides a minimal implementation that creates a default SdkMeterProvider without
//! any actual metrics collection or export functionality.

use crate::errors::MetricsError;
use opentelemetry_sdk::metrics::SdkMeterProvider;

/// Creates and installs a no-operation metrics provider.
///
/// This function is the fallback when no specific metrics exporter features are enabled.
/// It returns a default SdkMeterProvider that effectively discards all metrics.
///
/// # Returns
///
/// * `Ok(SdkMeterProvider)` - A default meter provider that doesn't export metrics
/// * `Err(MetricsError)` - This implementation should never return an error
pub fn install() -> Result<SdkMeterProvider, MetricsError> {
    Ok(SdkMeterProvider::default())
}
