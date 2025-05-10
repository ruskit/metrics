// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! # Metrics Export Selectors
//!
//! Provides temporality selectors for OpenTelemetry metrics exporters.
//!
//! This module defines the temporality selection strategy used by the metrics exporters.
//! Temporality refers to how successive data points relate to each other in time.

use opentelemetry_sdk::{
    error::OTelSdkResult,
    metrics::{
        data::ResourceMetrics, reader::MetricReader, InstrumentKind, MetricResult, Pipeline,
        Temporality,
    },
};
use std::sync::Weak;

/// # OTLPTemporalitySelector
///
/// Selects the appropriate temporality for different instrument kinds when using OTLP.
///
/// This struct implements the `MetricReader` trait to provide a temporality selection
/// strategy that optimizes for different instrument types:
///
/// - Uses Cumulative temporality for counters that can go up and down (UpDownCounter)
/// - Uses Delta temporality for other instrument types
///
/// ## Temporality Background
///
/// - **Cumulative temporality**: Successive data points repeat the starting timestamp.
///   For example, from start time T0, data points cover time ranges (T0, T1], (T0, T2], (T0, T3], etc.
///   Common in systems like Prometheus. Provides natural averaging when collection fails intermittently.
///
/// - **Delta temporality**: Successive data points advance the starting timestamp.
///   For example, from start time T0, data points cover time ranges (T0, T1], (T1, T2], (T2, T3], etc.
///   Common in systems like Statsd. Enables sampling and reduces process memory usage.
#[derive(Debug, Clone, Default)]
pub struct OTLPTemporalitySelector;

impl MetricReader for OTLPTemporalitySelector {
    /// Determines the temporality strategy for the given instrument kind.
    ///
    /// # Parameters
    ///
    /// * `kind` - The kind of instrument (counter, gauge, etc.)
    ///
    /// # Returns
    ///
    /// The selected temporality for the instrument kind
    fn temporality(&self, kind: InstrumentKind) -> Temporality {
        match kind {
            InstrumentKind::UpDownCounter | InstrumentKind::ObservableUpDownCounter => {
                Temporality::Cumulative
            }
            _ => Temporality::Delta,
        }
    }

    /// Registers a pipeline with this reader.
    ///
    /// This implementation is a no-op as this selector is not collecting metrics.
    fn register_pipeline(&self, _: Weak<Pipeline>) {}

    /// Collects metrics.
    ///
    /// This implementation is a placeholder and will panic if called.
    fn collect(&self, _: &mut ResourceMetrics) -> MetricResult<()> {
        todo!()
    }

    /// Forces a flush of metrics.
    ///
    /// This implementation is a placeholder and will panic if called.
    fn force_flush(&self) -> OTelSdkResult {
        todo!()
    }

    /// Shuts down the reader.
    ///
    /// This implementation is a placeholder and will panic if called.
    fn shutdown(&self) -> OTelSdkResult {
        todo!()
    }
}
