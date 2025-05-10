# Metrics

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/metrics.svg)](https://crates.io/crates/metrics)
[![Documentation](https://docs.rs/metrics/badge.svg)](https://docs.rs/metrics)

A flexible metrics collection and export library for Rust applications in the Ruskit ecosystem.

## Overview

The `metrics` crate provides a simple yet powerful interface for collecting and exporting metrics from Rust applications. It supports multiple export formats including OpenTelemetry OTLP, Prometheus, and standard output.

Built on top of the OpenTelemetry SDK, this library integrates with the Ruskit configuration system for seamless setup and consistent metrics reporting across different environments.

## Features

- **Multiple Exporters**: Support for different metrics export formats:
  - **OTLP**: Export metrics using OpenTelemetry Protocol over gRPC
  - **Prometheus**: Expose metrics in Prometheus format via HTTP endpoint
  - **Stdout**: Write metrics to standard output for development
- **Feature-based Configuration**: Use Cargo features to include only what you need
- **Integration with Ruskit Config**: Seamless configuration through the Ruskit ecosystem
- **Resource Attribution**: Automatic addition of service name, environment, and other attributes
- **Temporality Control**: Intelligent selection of delta or cumulative temporality based on instrument type

## Installation

Add `metrics` to your `Cargo.toml`:

```toml
[dependencies]
metrics = { git = "https://github.com/ruskit/metrics.git", tag = "v0.0.1" }
configs = { git = "https://github.com/ruskit/configs.git", tag = "v0.0.1" }

# Enable the exporters you need
[features]
default = ["stdout"]
prometheus = ["metrics/prometheus"]
otlp = ["metrics/otlp"]
stdout = ["metrics/stdout"]
```

## Usage

### Basic Setup

```rust
use configs::{Configs, Empty, MetricExporterKind};
use metrics::provider;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a default configuration
    let mut configs = Configs::<Empty>::default();
    
    // Enable metrics collection with the stdout exporter
    configs.metric.enable = true;
    configs.metric.exporter = MetricExporterKind::Stdout;
    
    // Initialize metrics (returns Prometheus registry if using Prometheus exporter)
    let registry = provider::init(&configs)?;
    
    // Now you can use OpenTelemetry API to record metrics
    let meter = opentelemetry::global::meter("my_app");
    let counter = meter
        .u64_counter("requests")
        .with_description("Number of requests processed")
        .init();
    
    // Record a metric
    counter.add(1, &[]);
    
    Ok(())
}
```

### Using with Prometheus

```rust
use configs::{Configs, Empty, MetricExporterKind};
use metrics::provider;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a default configuration
    let mut configs = Configs::<Empty>::default();
    
    // Enable metrics collection with the Prometheus exporter
    configs.metric.enable = true;
    configs.metric.exporter = MetricExporterKind::Prometheus;
    
    // Initialize metrics and get the Prometheus registry
    let registry = provider::init(&configs)?;
    
    // Create an HTTP endpoint to expose metrics
    if let Some(registry) = registry {
        let metrics_route = warp::path("metrics").map(move || {
            let encoder = prometheus::TextEncoder::new();
            let mut buffer = Vec::new();
            encoder.encode(&registry.gather(), &mut buffer).unwrap();
            String::from_utf8(buffer).unwrap()
        });
        
        // Start the HTTP server
        tokio::spawn(warp::serve(metrics_route).run(([0, 0, 0, 0], 9100)));
    }
    
    // Rest of your application...
    Ok(())
}
```

### Using with OTLP

```rust
use configs::{Configs, Empty, MetricExporterKind};
use metrics::provider;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a default configuration
    let mut configs = Configs::<Empty>::default();
    
    // Enable metrics collection with the OTLP exporter
    configs.metric.enable = true;
    configs.metric.exporter = MetricExporterKind::OtlpGrpc;
    configs.metric.host = "http://localhost:4317".to_string();
    configs.metric.header_access_key = "api-key".to_string();
    configs.metric.access_key = "YOUR_API_KEY".to_string();
    
    // Initialize metrics
    provider::init(&configs)?;
    
    // Now you can use OpenTelemetry API to record metrics
    // ...
    
    Ok(())
}
```

## Feature Flags

- `prometheus` - Enables the Prometheus exporter
- `otlp` - Enables the OpenTelemetry Protocol (OTLP) exporter
- `stdout` - Enables the standard output exporter (enabled by default)

## Configuration Options

The metrics system uses the following configuration options from the `configs` crate:

| Config Field | Description | Default |
|--------------|-------------|---------|
| `metric.enable` | Whether metrics collection is enabled | `false` |
| `metric.exporter` | Which exporter to use (Stdout, OtlpGrpc, Prometheus) | `Stdout` |
| `metric.host` | The host address for the OTLP exporter | `""` |
| `metric.header_access_key` | Header name for access key authentication | `""` |
| `metric.access_key` | Access key value for authentication | `""` |
| `metric.service_type` | Service type identifier for metrics | `""` |
| `metric.export_timeout` | Timeout for metric export operations in seconds | `30` |
| `metric.export_interval` | Interval between metric exports in seconds | `60` |
| `metric.export_rate_base` | Base rate for export sampling | `0.8` |

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Building with Specific Features

```bash
cargo build --features prometheus
cargo build --features otlp
cargo build --features stdout
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Ruskit Ecosystem

This crate is part of the [Ruskit](https://github.com/ruskit) ecosystem, which provides a modular toolkit for building robust Rust applications with built-in support for:

- Configuration management (`configs` crate)
- Structured logging (`logging` crate)
- Distributed tracing (`traces` crate)
- Metrics collection (this crate)
- Secrets management (`secrets_manager` crate)