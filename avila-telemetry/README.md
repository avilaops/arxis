# avila-telemetry

## ⚙️ The Observatory of Arxis

**Time series analysis and observability** - Monitoring the engine's heartbeat

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-2021-CE422B.svg)](https://www.rust-lang.org/)
[![Part of Arxis](https://img.shields.io/badge/Arxis-Observatory-00d4ff)](https://github.com/avilaops/arxis)

**avila-telemetry** is the observatory within the citadel - monitoring the **AXIS** (engine) to ensure optimal performance.

Like a watchful sentinel, it detects anomalies, forecasts trends, and ensures NASA-grade data quality for scientific missions.

## Features

### Time Series Analysis
- **Operations**: Moving average, exponential smoothing, differencing
- **Statistics**: Mean, std dev, min, max, percentiles
- **Transformations**: Slicing, resampling, windowing

### Anomaly Detection
- **Statistical Methods**: Z-score (configurable threshold)
- **Robust Methods**: IQR (Interquartile Range) detection
- **Use Cases**: Glitch detection, instrumental artifacts, outliers

### Forecasting
- **ARIMA**: AutoRegressive Integrated Moving Average
- **Exponential Smoothing**: Simple, double, triple
- **Applications**: Trend prediction, observation planning

### Data Quality (NASA Standards)
- **Quality Metrics**: Accuracy, completeness, consistency, validity
- **Scoring**: Overall quality score (0-1), NASA threshold (≥0.95)
- **Observability**: Structured logging, alerts, performance tracking

## Usage

```rust
use avila_telemetry::{TimeSeries, AnomalyDetector};

// Time series operations
let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 4.0, 3.0, 2.0];
let ts = TimeSeries::new(data);
let ma = ts.moving_average(3)?;
let stats = ts.statistics();

// Anomaly detection
let detector = AnomalyDetector::new(3.0, 1.5); // 3-sigma, 1.5 IQR
let anomalies = detector.detect_zscore(&ts)?;
println!("Found {} anomalies", anomalies.len());

// Forecasting
use avila_telemetry::forecasting::ExponentialSmoothing;
let forecaster = ExponentialSmoothing::simple(0.3);
let forecast = forecaster.predict(&ts, 5)?; // 5 steps ahead
```

## Installation

```toml
[dependencies]
avila-telemetry = { git = "https://github.com/avilaops/arxis", branch = "main" }
chrono = "0.4"
```

## Tests

```bash
cargo test -p avila-telemetry
```

---

## ⚙️ Part of Arxis

**avila-telemetry** is the observatory of [**Arxis**](https://github.com/avilaops/arxis) - monitoring the engine.

**ARX** (fortress) + **AXIS** (engine) = **ARXIS**

Built with ❤️ by [Avila](https://avila.cloud)
```

## Installation

```toml
[dependencies]
avila-telemetry = { git = "https://github.com/avilaops/arxis", branch = "main" }
chrono = { version = "0.4", features = ["serde"] }
```

## Examples

```bash
cargo run --example basic_operations
cargo run --example anomaly_detection
cargo run --example forecasting
cargo run --example nasa_gcp_observability
```

## Tests

```bash
cargo test -p avila-telemetry
```

**22 tests passing** ✅

## License

MIT - See LICENSE for details
