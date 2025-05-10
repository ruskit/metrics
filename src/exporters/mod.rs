// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! # Metrics Exporters
//!
//! Defines different exporters for metrics data.
//!
//! This module contains implementations for various metrics exporters,
//! including OTLP, Prometheus, and Stdout. Each exporter is conditionally
//! compiled based on the corresponding feature flag.

mod selectors;

#[cfg(feature = "otlp")]
pub mod otlp;

#[cfg(feature = "prometheus")]
pub mod prom;

#[cfg(feature = "stdout")]
pub mod stdout;
