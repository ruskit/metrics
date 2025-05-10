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
//! - Support for multiple metric exporters:
//!   - **OTLP**: Export metrics using OpenTelemetry Protocol over gRPC (requires `otlp` feature)
//!   - **Prometheus**: Expose metrics in Prometheus format via HTTP endpoint (requires `prometheus` feature)
//!   - **Stdout**: Write metrics to standard output for development (requires `stdout` feature)
//! - Unified interface for all exporters
//! - Integration with Ruskit configuration system
//! - Error handling with specific error types
//!
//! ## Example
//!
//! ```rust
//! use configs::{Configs, Empty};
//! use metrics::provider;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Load application configuration
//!     let configs = Configs::<Empty>::default();
//!     
//!     // Initialize metrics (returns Prometheus registry if using Prometheus exporter)
//!     let registry = provider::init(&configs)?;
//!     
//!     // Now you can use OpenTelemetry API to record metrics
//!     // ...
//!     
//!     Ok(())
//! }
//! ```

pub mod errors;
pub mod exporters;
pub mod provider;
