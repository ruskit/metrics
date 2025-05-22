# 📊 Metrics

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/metrics.svg)](https://crates.io/crates/metrics)
[![Documentation](https://docs.rs/metrics/badge.svg)](https://docs.rs/metrics)
[![Build Status](https://github.com/ruskit/metrics/actions/workflows/ci.yml/badge.svg)](https://github.com/ruskit/metrics/actions)
[![OpenTelemetry](https://img.shields.io/badge/OpenTelemetry-enabled-blue)](https://opentelemetry.io/)

A flexible and performant metrics collection and export library for Rust applications in the [Ruskit](https://github.com/ruskit) ecosystem.

<p align="center">
  <b>Connect your Rust applications to modern observability platforms.</b>
</p>

## 🌟 Overview

The `metrics` crate provides a simple yet powerful interface for collecting and exporting metrics from Rust applications. Built on top of the OpenTelemetry SDK, it supports multiple export formats and integrates seamlessly with the Ruskit configuration system.

### Why Metrics?

- **Measure what matters**: Gain insights into application performance and behavior
- **Make data-driven decisions**: Track KPIs and operational metrics
- **Detect issues early**: Monitor trends to identify potential problems
- **Understand user patterns**: Capture business-relevant metrics

## ✨ Features

- **🔌 Multiple Exporters**:
  - **[OTLP](https://opentelemetry.io/docs/specs/otlp/)**: Export metrics using OpenTelemetry Protocol over gRPC
  - **[Prometheus](https://prometheus.io/)**: Expose metrics in Prometheus format via HTTP endpoint
  - **Stdout**: Write metrics to standard output for development and debugging
- **🧩 Feature-based Configuration**: Use Cargo features to include only what you need
- **⚙️ Integration with Ruskit Config**: Seamless configuration through the ecosystem
- **🏷️ Resource Attribution**: Automatic addition of service name, environment, and other attributes
- **⏱️ Intelligent Temporality**: Optimal selection of delta or cumulative temporality based on instrument type
- **🔍 Comprehensive Documentation**: Well-documented API with examples

## 📦 Installation

Add `metrics` to your `Cargo.toml`:

```toml
[dependencies]
metrics = { version = "0.1", default-features = false }
opentelemetry = { version = "0.19", features = ["metrics"] }
configs = { version = "0.1" }

# Enable the exporters you need
[features]
default = ["stdout"]
prometheus = ["metrics/prometheus"]
otlp = ["metrics/otlp"]
stdout = ["metrics/stdout"]
```

For development directly from the repository:

```toml
[dependencies]
metrics = { git = "https://github.com/ruskit/metrics.git", tag = "v0.1.0" }
configs = { git = "https://github.com/ruskit/configs.git", tag = "v0.1.0" }
```

## 🚀 Quick Start

### Basic Setup

```rust
use opentelemetry::metrics::MeterProvider;
use metrics::provider;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize metrics provider
    let provider = provider::install()?;
    
    // Get a meter for your component
    let meter = provider.meter("my_component");
    
    // Create instruments
    let counter = meter
        .u64_counter("requests")
        .with_description("Number of requests processed")
        .init();
    
    // Record a measurement
    counter.add(1, &[]);
    
    // Your application code...
    
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

## 🔧 Advanced Usage

### Working with Different Metric Types

```rust
use opentelemetry::{KeyValue, metrics::MeterProvider};
use metrics::provider;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = provider::install()?;
    let meter = provider.meter("payment_service");
    
    // Counter - for values that only go up
    let counter = meter.u64_counter("processed_payments").init();
    counter.add(1, &[KeyValue::new("status", "success")]);
    
    // UpDownCounter - for values that can increase or decrease
    let updown = meter.i64_up_down_counter("active_connections").init();
    updown.add(1, &[]);  // Connection opened
    updown.add(-1, &[]); // Connection closed
    
    // Histogram - for recording distributions of values
    let histogram = meter.f64_histogram("response_time_seconds").init();
    histogram.record(0.25, &[KeyValue::new("endpoint", "/api/users")]);
    
    // Observable Gauge - for current value observations
    let gauge = meter.f64_observable_gauge("memory_usage_bytes").init();
    meter.register_callback(&[gauge.as_any()], move |observer| {
        // Get current memory usage
        let memory_usage = get_memory_usage();
        observer.observe_f64(&gauge, memory_usage, &[]);
    })?;
    
    Ok(())
}

fn get_memory_usage() -> f64 {
    // Implementation to get memory usage
    1024.0 * 1024.0 // Example: 1MB
}
```

### Using with Different Exporters

Select the appropriate exporter through feature flags:

```bash
# Build with OTLP exporter
cargo build --features otlp

# Build with stdout exporter (default)
cargo build --features stdout
```

## ⚙️ Feature Flags

| Feature | Description | Default |
|---------|-------------|---------|
| `otlp` | Enables the OpenTelemetry Protocol (OTLP) exporter over gRPC | No |
| `stdout` | Enables the standard output exporter for development | Yes |
| `prometheus` | *Coming soon:* Enables the Prometheus exporter | No |

## 🔧 Configuration Options

The metrics system can be configured through environment variables or the `configs` crate:

| Config Field | Environment Variable | Description | Default |
|--------------|---------------------|-------------|---------|
| `metric.enable` | `METRIC_ENABLE` | Whether metrics collection is enabled | `false` |
| `metric.exporter` | `METRIC_EXPORTER` | Exporter type (`Stdout`, `OtlpGrpc`, `Prometheus`) | `Stdout` |
| `metric.host` | `METRIC_HOST` | Host address for the OTLP exporter | `""` |
| `metric.header_access_key` | `METRIC_HEADER_ACCESS_KEY` | Header name for authentication | `""` |
| `metric.access_key` | `METRIC_ACCESS_KEY` | Access key value for authentication | `""` |
| `metric.service_type` | `METRIC_SERVICE_TYPE` | Service type identifier | `""` |
| `metric.export_timeout` | `METRIC_EXPORT_TIMEOUT` | Export timeout in seconds | `30` |
| `metric.export_interval` | `METRIC_EXPORT_INTERVAL` | Export interval in seconds | `60` |
| `metric.export_rate_base` | `METRIC_EXPORT_RATE_BASE` | Base rate for export sampling | `0.8` |

## 👨‍💻 Development

### Building & Testing

```bash
# Build with default features
cargo build

# Run tests
cargo test

# Build with specific exporters
cargo build --features otlp
cargo build --no-default-features --features stdout
```

### Local Development Setup

For local development with OTLP, you can use the OpenTelemetry Collector:

1. Install the [OpenTelemetry Collector](https://opentelemetry.io/docs/collector/getting-started/)
2. Configure your collector to receive OTLP and export to your desired backend:

```yaml
# collector-config.yaml
receivers:
  otlp:
    protocols:
      grpc:
        endpoint: localhost:4317

processors:
  batch:

exporters:
  logging:
    verbosity: detailed

service:
  pipelines:
    metrics:
      receivers: [otlp]
      processors: [batch]
      exporters: [logging]
```

3. Run the collector and your application with the OTLP exporter enabled.

## 📊 Integration Examples

- [Web Service with Axum](https://github.com/ruskit/examples/web-service)
- [CLI Application](https://github.com/ruskit/examples/cli-app)
- [Worker Service](https://github.com/ruskit/examples/worker)

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🌐 Ruskit Ecosystem

This crate is part of the [Ruskit](https://github.com/ruskit) ecosystem, which provides a modular toolkit for building robust Rust applications:

| Crate | Description |
|-------|-------------|
| [`configs`](https://github.com/ruskit/configs) | Configuration management with environment variables and files |
| [`logging`](https://github.com/ruskit/logging) | Structured logging with different output formats |
| [`traces`](https://github.com/ruskit/traces) | Distributed tracing with OpenTelemetry |
| **`metrics`** | *This crate:* Metrics collection and export |
| [`secrets_manager`](https://github.com/ruskit/secrets_manager) | Secure secrets management |

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request