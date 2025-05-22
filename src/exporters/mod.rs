// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! # Metrics Exporters
//!
//! Provides various exporters for OpenTelemetry metrics data.
//!
//! This module contains implementations for different metrics exporters:
//!
//! - **OTLP Exporter**: Sends metrics to an OpenTelemetry collector using the OpenTelemetry Protocol over gRPC
//! - **Stdout Exporter**: Writes metrics to standard output for development and debugging
//! - **No-op Exporter**: A fallback exporter that discards metrics when no other exporter is enabled
//!
//! Each exporter is conditionally compiled based on the corresponding feature flag, allowing
//! applications to include only the exporters they need. This reduces binary size and dependencies
//! when only specific exporters are required.
//!
//! The module also includes common components like temporality selectors which define how
//! successive metric data points relate to each other in time.
//!
//! ## Feature Flags
//!
//! - `otlp`: Enable the OTLP exporter (gRPC)
//! - `stdout`: Enable the stdout exporter
//!
//! If no export feature is enabled, the no-op exporter will be used as a fallback.

mod selectors;

#[cfg(feature = "otlp")]
pub mod otlp_grpc;

#[cfg(feature = "stdout")]
pub mod stdout;

pub mod noop;
