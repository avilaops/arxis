# Advanced Features - Ávila CLI v1.0.0

## 🎭 Macro Helpers - Rapid Development

### Quick CLI Definition

```rust
use avila_cli::*;

fn main() {
    let app = cli! {
        "myapp" => {
            version: "1.0.0",
            about: "Ultra-fast CLI tool",
            args: [
                arg!("verbose", short: 'v'),
                arg!("debug", short: 'd'),
                arg!("output", required, short: 'o', help: "Output file"),
                arg!("format", default: "json", help: "Output format"),
            ]
        }
    };

    let matches = app.parse();
}
```

## 🌍 Environment Variable Fallback

### Automatic Prefix-Based

```rust
use avila_cli::*;

fn main() {
    let matches = App::new("myapp")
        .env_prefix("MYAPP")  // Enables MYAPP_PORT, MYAPP_HOST, etc.
        .arg(Arg::new("port")
            .takes_value(true)
            .default_value("8080"))
        .arg(Arg::new("host")
            .takes_value(true)
            .default_value("localhost"))
        .parse();

    // Priority: CLI > MYAPP_PORT env var > default
    let port = matches.value_of("port").unwrap();
    let host = matches.value_of("host").unwrap();

    println!("Server: {}:{}", host, port);
}
```

Run with:
```bash
# Use default
$ ./myapp
Server: localhost:8080

# Override via environment
$ MYAPP_PORT=3000 ./myapp
Server: localhost:3000

# CLI overrides everything
$ MYAPP_PORT=3000 ./myapp --port 9000
Server: localhost:9000
```

### Specific Environment Variables

```rust
use avila_cli::*;

fn main() {
    let matches = App::new("myapp")
        .arg(Arg::new("token")
            .takes_value(true)
            .env("API_TOKEN")  // Specific env var
            .required(true))
        .arg(Arg::new("database")
            .takes_value(true)
            .env("DATABASE_URL")
            .default_value("sqlite:memory"))
        .parse();

    let token = matches.value_of("token").unwrap();
    let db = matches.value_of("database").unwrap();

    println!("Token: {}", token);
    println!("Database: {}", db);
}
```

## 🔧 Config File Support

### Simple INI/TOML-like Format

Create `myapp.conf`:
```ini
# My app configuration
port = 8080
host = 0.0.0.0
debug = true
format: json
```

```rust
use avila_cli::*;

fn main() {
    let matches = App::new("myapp")
        .config_file("myapp.conf")  // Load config
        .arg(Arg::new("port").takes_value(true))
        .arg(Arg::new("host").takes_value(true))
        .arg(Arg::new("debug"))
        .arg(Arg::new("format").takes_value(true))
        .parse();

    // Priority: CLI > Env > Config > Default
    println!("Port: {}", matches.value_of("port").unwrap());
    println!("Host: {}", matches.value_of("host").unwrap());
    println!("Debug: {}", matches.is_present("debug"));
}
```

## 📊 Value Source Tracking

Know where each value came from:

```rust
use avila_cli::*;

fn main() {
    let matches = App::new("myapp")
        .env_prefix("MYAPP")
        .config_file("app.conf")
        .arg(Arg::new("port")
            .takes_value(true)
            .default_value("8080"))
        .parse();

    if let Some(source) = matches.value_source("port") {
        match source {
            ValueSource::CommandLine => println!("Port from CLI"),
            ValueSource::Environment => println!("Port from ENV"),
            ValueSource::ConfigFile => println!("Port from config"),
            ValueSource::Default => println!("Port using default"),
        }
    }
}
```

## 🔗 Argument Relations

### Conflicts

```rust
use avila_cli::*;

fn main() {
    let matches = App::new("myapp")
        .arg(Arg::new("json")
            .conflicts_with("yaml"))  // Can't use both
        .arg(Arg::new("yaml")
            .conflicts_with("json"))
        .parse();

    if matches.is_present("json") {
        println!("Using JSON format");
    } else if matches.is_present("yaml") {
        println!("Using YAML format");
    }
}
```

```bash
$ ./myapp --json --yaml
Error: --json conflicts with --yaml
```

### Requirements

```rust
use avila_cli::*;

fn main() {
    let matches = App::new("myapp")
        .arg(Arg::new("encrypt")
            .requires("key"))  // Must have --key too
        .arg(Arg::new("key")
            .takes_value(true))
        .parse();
}
```

```bash
$ ./myapp --encrypt
Error: --encrypt requires --key

$ ./myapp --encrypt --key mysecret
✓ OK
```

### Hidden Arguments

```rust
use avila_cli::*;

fn main() {
    let matches = App::new("myapp")
        .arg(Arg::new("debug")
            .hidden(true))  // Not shown in --help
        .arg(Arg::new("verbose"))
        .parse();
}
```

## 🎨 Colored Output Control

```rust
use avila_cli::*;

fn main() {
    // Disable colors
    let matches = App::new("myapp")
        .colored_help(false)
        .arg(Arg::new("verbose").required(true))
        .parse();

    // Respects NO_COLOR environment variable automatically
}
```

## 🐚 Shell Completion Generation

### Generate at Build Time

```rust
use avila_cli::*;
use std::fs;

fn main() {
    let app = App::new("myapp")
        .arg(Arg::new("file").takes_value(true))
        .arg(Arg::new("verbose"));

    // Generate all completion scripts
    fs::write("myapp.bash", app.generate_completion(Shell::Bash)).unwrap();
    fs::write("myapp.zsh", app.generate_completion(Shell::Zsh)).unwrap();
    fs::write("myapp.fish", app.generate_completion(Shell::Fish)).unwrap();
    fs::write("myapp.ps1", app.generate_completion(Shell::PowerShell)).unwrap();

    println!("Completions generated!");
}
```

### Runtime Generation

```rust
use avila_cli::*;

fn main() {
    let app = App::new("myapp")
        .arg(Arg::new("completions")
            .takes_value(true)
            .possible_values(&["bash", "zsh", "fish", "powershell"]));

    let matches = app.clone().parse();

    if let Some(shell) = matches.value_of("completions") {
        let script = match shell {
            "bash" => app.generate_completion(Shell::Bash),
            "zsh" => app.generate_completion(Shell::Zsh),
            "fish" => app.generate_completion(Shell::Fish),
            "powershell" => app.generate_completion(Shell::PowerShell),
            _ => unreachable!(),
        };
        println!("{}", script);
        return;
    }

    // Normal app logic...
}
```

Install completions:
```bash
# Bash
$ ./myapp --completions bash > /usr/share/bash-completion/completions/myapp

# Zsh
$ ./myapp --completions zsh > /usr/share/zsh/site-functions/_myapp

# Fish
$ ./myapp --completions fish > ~/.config/fish/completions/myapp.fish

# PowerShell
$ ./myapp --completions powershell > myapp.ps1
```

## ⚡ Advanced Validation

### Custom Validators

```rust
use avila_cli::*;

fn main() {
    let matches = App::new("myapp")
        .arg(Arg::new("port")
            .takes_value(true)
            .validator(|v| {
                v.parse::<u16>()
                    .map(|p| {
                        if p < 1024 {
                            Err("Port must be >= 1024".to_string())
                        } else {
                            Ok(())
                        }
                    })
                    .unwrap_or_else(|_| Err("Invalid port number".to_string()))
            }))
        .arg(Arg::new("email")
            .takes_value(true)
            .validator(|v| {
                if v.contains('@') && v.contains('.') {
                    Ok(())
                } else {
                    Err("Invalid email format".to_string())
                }
            }))
        .parse();
}
```

### Chained Validation

```rust
use avila_cli::*;
use std::path::Path;

fn validate_file_exists(path: &str) -> Result<(), String> {
    if Path::new(path).exists() {
        Ok(())
    } else {
        Err(format!("File '{}' does not exist", path))
    }
}

fn validate_file_writable(path: &str) -> Result<(), String> {
    // Check if parent directory is writable
    if let Some(parent) = Path::new(path).parent() {
        if parent.exists() {
            Ok(())
        } else {
            Err(format!("Directory '{}' does not exist", parent.display()))
        }
    } else {
        Ok(())
    }
}

fn main() {
    let matches = App::new("myapp")
        .arg(Arg::new("input")
            .takes_value(true)
            .validator(validate_file_exists))
        .arg(Arg::new("output")
            .takes_value(true)
            .validator(validate_file_writable))
        .parse();
}
```

## 🔐 Complete Example - Production Ready

```rust
use avila_cli::*;
use std::process;

fn main() {
    let app = App::new("myserver")
        .version("1.0.0")
        .author("Your Name")
        .about("High-performance web server")

        // Config file support
        .config_file(".myserver.conf")
        .env_prefix("MYSERVER")

        // Server options
        .arg(Arg::new("port")
            .short('p')
            .takes_value(true)
            .default_value("8080")
            .env("PORT")
            .validator(|v| {
                v.parse::<u16>()
                    .map(|_| ())
                    .map_err(|_| "Invalid port".to_string())
            })
            .help("Server port"))

        .arg(Arg::new("host")
            .short('h')
            .takes_value(true)
            .default_value("127.0.0.1")
            .env("HOST")
            .help("Bind address"))

        // Output format (mutual exclusion)
        .arg(Arg::new("json").conflicts_with("yaml"))
        .arg(Arg::new("yaml").conflicts_with("json"))

        // TLS options (encryption requires both cert and key)
        .arg(Arg::new("tls")
            .requires("cert")
            .requires("key"))
        .arg(Arg::new("cert")
            .takes_value(true)
            .hidden(true))
        .arg(Arg::new("key")
            .takes_value(true)
            .hidden(true))

        // Logging
        .arg(Arg::new("verbose")
            .short('v')
            .help("Verbose output"))
        .arg(Arg::new("quiet")
            .short('q')
            .conflicts_with("verbose")
            .help("Suppress output"))

        // Argument group
        .group(ArgGroup::new("format")
            .args(&["json", "yaml"])
            .required(false)
            .multiple(false));

    let matches = app.parse();

    // Extract configuration
    let port = matches.value_of("port").unwrap();
    let host = matches.value_of("host").unwrap();

    // Show value sources
    if matches.is_present("verbose") {
        if let Some(source) = matches.value_source("port") {
            println!("Port source: {:?}", source);
        }
        if let Some(source) = matches.value_source("host") {
            println!("Host source: {:?}", source);
        }
    }

    // Start server
    println!("🚀 Server starting on {}:{}", host, port);

    if matches.is_present("tls") {
        println!("🔒 TLS enabled");
    }

    let format = if matches.is_present("json") {
        "JSON"
    } else if matches.is_present("yaml") {
        "YAML"
    } else {
        "Plain"
    };
    println!("📊 Output format: {}", format);

    // Your server logic here...
}
```

## 🎯 Best Practices

### 1. Always Provide Defaults

```rust
.arg(Arg::new("port")
    .takes_value(true)
    .default_value("8080"))  // Good!
```

### 2. Use Environment Variables for Secrets

```rust
.arg(Arg::new("api_key")
    .takes_value(true)
    .env("API_KEY")  // Never hardcode secrets
    .required(true))
```

### 3. Validate Early

```rust
.arg(Arg::new("file")
    .takes_value(true)
    .validator(|f| {
        if Path::new(f).exists() {
            Ok(())
        } else {
            Err("File not found".to_string())
        }
    }))
```

### 4. Use Config Files for Complex Apps

```rust
.config_file(".myapp.conf")  // User can override via file
.env_prefix("MYAPP")          // Or via environment
```

### 5. Generate Completions

```rust
// In build.rs
use avila_cli::*;
use std::fs;

fn main() {
    let app = /* your app definition */;
    fs::write("completions/myapp.bash", app.generate_completion(Shell::Bash)).ok();
    fs::write("completions/myapp.zsh", app.generate_completion(Shell::Zsh)).ok();
    fs::write("completions/myapp.fish", app.generate_completion(Shell::Fish)).ok();
}
```

## 🏆 Performance Tips

1. **Pre-allocate**: The parser uses `HashMap::with_capacity()` internally
2. **Zero-copy**: Values are borrowed, not cloned
3. **Lazy validation**: Only validates arguments that are present
4. **Fast path**: Common cases (--help, --version) exit early
5. **No allocations**: Uses stack for most operations

## 📚 More Examples

See `/examples` directory for complete working examples:
- `examples/basic.rs` - Simple CLI
- `examples/advanced.rs` - All features
- `examples/server.rs` - Production server
- `examples/completions.rs` - Shell completion generation
- `examples/config.rs` - Config file usage

---

**Zero dependencies. Maximum features. Pure Rust.**
