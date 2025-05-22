// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! # Metrics
//!
//! A flexible metrics collection and export library for Rust applications in the Ruskit ecosystem.
//!
//! This crate provides a simple yet powerful interface for collecting and exporting metrics
//! from Rust applications. It supports multiple export formats including OpenTelemetry OTLP,
//! Prometheus, and standard output.
//!
//! The library is built on top of OpenTelemetry's metrics SDK and integrates with the Ruskit
//! configuration system for seamless setup.
//!
//! ## Features
//!
//! - **Multiple Exporters**: Support for various metric export formats:
//!   - **OTLP**: Export metrics using OpenTelemetry Protocol over gRPC (requires `otlp` feature)
//!   - **Prometheus**: Expose metrics in Prometheus format via HTTP endpoint (requires `prometheus` feature)
//!   - **Stdout**: Write metrics to standard output for development (requires `stdout` feature)
//! - **Smart Temporality Selection**: Automatically selects optimal temporality strategy based on the metric type
//! - **Resource Attribution**: Automatically adds service name, namespace, environment and other attributes
//! - **Unified Interface**: Common API across all exporters
//! - **Ruskit Integration**: Seamless integration with Ruskit's configuration system
//! - **Comprehensive Error Handling**: Well-defined error types for better debugging
//!
//! ## Example
//!
//! ```rust
//! use configs::{Configs, Empty};
//! use metrics::provider;
//! use opentelemetry::metrics::MeterProvider;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize metrics with default configuration
//!     let meter_provider = provider::install()?;
//!     
//!     // Get a meter for your module or component
//!     let meter = meter_provider.meter("my_component");
//!     
//!     // Create instruments and record measurements
//!     let counter = meter.u64_counter("my_counter").init();
//!     counter.add(1, &[]);
//!     
//!     // Application runs and records metrics...
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Feature Flags
//!
//! - `otlp`: Enable OpenTelemetry Protocol (OTLP) exporter over gRPC
//! - `stdout`: Enable standard output exporter (useful for development)
//!
//! If no export features are enabled, a no-op implementation will be used.

pub mod errors;
pub mod exporters;
pub mod provider;
