# Data Extraction Agent - Configuration

## Environment Variables

Create a `.env` file in the project root:

```bash
# AvilaDB Connection
AVILADB_CONNECTION_STRING=http://localhost:8000
AVILADB_DATABASE=market_intelligence

# Google Places API (optional)
GOOGLE_PLACES_API_KEY=your_api_key_here

# Logging
RUST_LOG=info  # or debug, warn, error

# Rate Limiting
SCRAPER_RATE_LIMIT=10
SCRAPER_MAX_RETRIES=3
```

## Configuration File

Edit `config/default.toml` or create `config/local.toml` for local overrides:

```toml
[scraper]
rate_limit_per_second = 10
max_concurrent_requests = 5
request_timeout_seconds = 30
max_retries = 3

[aviladb]
connection_string = "http://localhost:8000"
database = "market_intelligence"

[anti_detection]
randomize_delays = true
min_delay_ms = 500
max_delay_ms = 2000
rotate_user_agents = true
```

## Proxy Configuration

To use proxies, add them to your config:

```toml
[proxy]
enabled = true
rotation = "round_robin"
proxies = [
    "http://proxy1.example.com:8080",
    "http://proxy2.example.com:8080",
    "socks5://proxy3.example.com:1080",
]
```

## Source-Specific Settings

### LinkedIn
```toml
[sources.linkedin]
enabled = true
rate_limit = 5  # Lower rate for LinkedIn
```

### ITJobs.pt
```toml
[sources.itjobs]
enabled = true
base_url = "https://www.itjobs.pt"
```

### Google Maps
```toml
[sources.google_maps]
enabled = true
api_key = "your_key"  # Or use GOOGLE_PLACES_API_KEY env var
```

## Monitoring & Alerts

```toml
[monitoring]
enabled = true
alert_on_low_success_rate = 0.8  # 80%
alert_on_high_response_time = 5000  # 5000ms
```
