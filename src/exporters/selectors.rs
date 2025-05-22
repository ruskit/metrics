// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! # Metrics Export Selectors
//!
//! Provides temporality selectors for OpenTelemetry metrics exporters.
//!
//! This module defines the temporality selection strategy used by the metrics exporters.
//! Temporality refers to how successive data points relate to each other in time.
//!
//! ## Usage
//!
//! Temporality selectors are used internally by exporters to determine how metric data points are reported over time. Most users do not need to interact with this module directly unless implementing a custom exporter or modifying temporality behavior.

use opentelemetry_sdk::{
    error::OTelSdkResult,
    metrics::{
        InstrumentKind, MetricResult, Pipeline, Temporality, data::ResourceMetrics,
        reader::MetricReader,
    },
};
use std::sync::Weak;

/// # OTLPTemporalitySelector
///
/// Implements a temporality selection strategy for OTLP metrics exporters.
///
/// This selector is used by the OTLP exporter to determine whether to use cumulative or delta temporality for each instrument type. It is designed to optimize compatibility and efficiency for different metric backends.
///
/// - **Cumulative**: Used for UpDownCounter and ObservableUpDownCounter instruments, matching Prometheus-style reporting.
/// - **Delta**: Used for all other instrument types, matching Statsd-style reporting.
///
/// This struct is primarily used internally and is not intended for direct use by most applications.
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
