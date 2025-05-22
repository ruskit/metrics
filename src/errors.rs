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
//!
//! The error types are designed to provide meaningful context to help diagnose issues
//! with metrics configuration, initialization, or export operations. These errors
//! are returned by public functions in the crate to indicate specific failure modes.
//!
//! All errors implement the standard `Error` trait, making them compatible with
//! error handling patterns like `?` operator and conversion to `Box<dyn Error>`.

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
/// * `InternalError` - An unspecified internal error occurred in the metrics subsystem
/// * `InvalidFeaturesError` - The requested exporter requires features that were not enabled at compile time
/// * `ConversionError` - Failed to convert between OpenTelemetry and exporter-specific data types
/// * `ExporterProviderError` - Failed to create the specified exporter provider, typically due to
///    connection issues or invalid configuration
///
/// ## Example
///
/// ```
/// use metrics::errors::MetricsError;
/// use std::error::Error;
///
/// fn process_result(result: Result<(), MetricsError>) -> Result<(), Box<dyn Error>> {
///     match result {
///         Err(MetricsError::InvalidFeaturesError) => {
///             println!("The requested metrics exporter requires a feature flag that wasn't enabled");
///             // Handle specific error case...
///         }
///         Err(e) => return Err(e.into()),
///         Ok(()) => println!("Metrics initialized successfully"),
///     }
///     Ok(())
/// }
/// ```
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
