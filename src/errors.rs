// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! # Metrics Errors
//!
//! Defines error types used throughout the metrics crate.
//!
//! This module provides a comprehensive set of error types that can occur during
//! metrics collection and export operations. Each error type includes a descriptive
//! message to help with troubleshooting.

use thiserror::Error;

/// # MetricsError
///
/// Error types that can occur during metrics operations.
///
/// This enum defines the various error conditions that can occur when setting up metrics
/// collection or during export operations.
///
/// ## Variants
///
/// * `InternalError` - An unspecified internal error occurred
/// * `InvalidFeaturesError` - The requested exporter requires features that were not enabled
/// * `ConversionError` - Failed to convert between data types
/// * `ExporterProviderError` - Failed to create the specified exporter provider
#[derive(Error, Debug, PartialEq, Eq)]
pub enum MetricsError {
    #[error("internal error")]
    InternalError,

    #[error("this exporter requires specific features")]
    InvalidFeaturesError,

    #[error("conversion error")]
    ConversionError,

    #[error("failure to create the exporter provide")]
    ExporterProviderError,
}
