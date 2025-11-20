# avx-cli

**Command-line tools for Avila Experience Fabric**

[![Crates.io](https://img.shields.io/crates/v/avx-cli.svg)](https://crates.io/crates/avx-cli)
[![Documentation](https://docs.rs/avx-cli/badge.svg)](https://docs.rs/avx-cli)
[![License](https://img.shields.io/crates/l/avx-cli.svg)](https://github.com/avilaops/arxis#license)

Command-line interface for managing, deploying, and monitoring AVX (Avila Experience) platform services. Provides developer-friendly tools for configuration, testing, and operations.

## Installation

### From crates.io

```bash
cargo install avx-cli
```

### From source

```bash
git clone https://github.com/avilaops/arxis
cd arxis/avx-cli
cargo install --path .
```

## Quick Start

```bash
# Show version and help
avx --version
avx --help

# Initialize new AVX project
avx init my-project

# Check configuration
avx config validate

# Start local development
avx dev

# Deploy to production
avx deploy production
```

## Commands

### Project Management

```bash
# Initialize new project
avx init <name> [--template <template>]

# Available templates:
#   - gateway: API gateway setup
#   - service: Microservice template
#   - quantum-render: Quantum renderer project
#   - full-stack: Complete AVX application

# Generate new component
avx generate service user-service
avx generate route /api/users
```

### Configuration

```bash
# Validate configuration files
avx config validate

# Show current configuration
avx config show [--env production]

# Set configuration value
avx config set server.port 8080

# List all config keys
avx config list
```

### Development

```bash
# Start development server with hot reload
avx dev

# Run with specific environment
avx dev --env staging

# Run tests
avx test [--watch]

# Lint code
avx lint

# Format code
avx fmt
```

### Deployment

```bash
# Deploy to environment
avx deploy <env>

# Deploy specific service
avx deploy production --service api-gateway

# Rollback deployment
avx rollback production

# Show deployment status
avx status [--env production]
```

### Monitoring

```bash
# Show logs
avx logs [--service gateway] [--follow]

# Show metrics
avx metrics [--service gateway]

# Health check
avx health [--all]

# Performance analysis
avx perf --trace request-id
```

### Gateway Management

```bash
# Add route to gateway
avx gateway add-route /api/users http://user-service:8000

# List routes
avx gateway routes

# Test route
avx gateway test /api/users

# Reload configuration
avx gateway reload
```

### Telemetry

```bash
# View real-time traces
avx trace --live

# Export traces
avx trace export --format json --output traces.json

# Analyze anomalies in time series
avx telemetry anomalies --metric response_time

# Generate performance report
avx telemetry report --from "1 hour ago"
```

## Configuration File

`avx-cli` looks for configuration in:

1. `./avx.toml` (project-specific)
2. `~/.avx/config.toml` (user-specific)
3. `/etc/avx/config.toml` (system-wide)

Example `avx.toml`:

```toml
[project]
name = "my-avx-app"
version = "0.1.0"

[dev]
port = 8080
hot_reload = true

[gateway]
routes = [
    { path = "/api/users", upstream = "http://localhost:8001" },
    { path = "/api/products", upstream = "http://localhost:8002" },
]

[telemetry]
enabled = true
level = "debug"
output = "console"

[deployment]
default_env = "staging"

[deployment.production]
host = "prod.avila.cloud"
region = "us-east-1"
```

## Environment Variables

Override configuration with environment variables:

```bash
export AVX_ENV=production
export AVX_PORT=3000
export AVX_LOG_LEVEL=info
export AVX_CONFIG_PATH=/custom/path/avx.toml
```

## Examples

### Initialize Full-Stack Project

```bash
avx init my-app --template full-stack
cd my-app
avx dev
```

### Deploy with Custom Config

```bash
avx deploy production \
    --config production.toml \
    --service api-gateway \
    --tag v1.2.3
```

### Monitor Logs in Real-Time

```bash
avx logs --service gateway --follow --level error
```

### Analyze Performance

```bash
# Find slow requests
avx perf slow-requests --threshold 1000ms --last 1h

# Show latency percentiles
avx perf latency --service gateway

# Trace specific request
avx trace ABC123-REQUEST-ID
```

## Integration with AVX Ecosystem

The CLI integrates with all AVX components:

- **avx-gateway**: Route management, health checks
- **avx-config**: Configuration validation and management
- **avx-telemetry**: Log viewing, metrics, tracing
- **avx-quantum-render**: Render job management

## Plugins

Extend functionality with plugins:

```bash
# Install plugin
avx plugin install avx-deploy-k8s

# List plugins
avx plugin list

# Use plugin
avx k8s deploy --namespace production
```

## Shell Completion

Generate shell completion scripts:

```bash
# Bash
avx completion bash > /etc/bash_completion.d/avx

# Zsh
avx completion zsh > /usr/local/share/zsh/site-functions/_avx

# Fish
avx completion fish > ~/.config/fish/completions/avx.fish

# PowerShell
avx completion powershell > $PROFILE
```

## Development

Build from source:

```bash
git clone https://github.com/avilaops/arxis
cd arxis/avx-cli
cargo build --release
./target/release/avx --version
```

## License

MIT OR Apache-2.0

See [LICENSE-MIT](../LICENSE-MIT) and [LICENSE-APACHE](../LICENSE-APACHE) for details.

## Links

- **Repository**: https://github.com/avilaops/arxis
- **Documentation**: https://docs.rs/avx-cli
- **Crates.io**: https://crates.io/crates/avx-cli
- **AVX Platform**: https://avila.cloud

## Support

- **Issues**: https://github.com/avilaops/arxis/issues
- **Email**: nicolas@avila.inc
- **Discussions**: https://github.com/avilaops/arxis/discussions
