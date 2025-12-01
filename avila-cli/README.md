# Ávila CLI Parser

Zero-dependency command-line argument parser with compile-time guarantees and stack-allocated data structures.

## Features

- **Zero dependencies**: Pure Rust standard library implementation
- **Type-safe**: Compile-time argument schema validation
- **O(1) lookups**: HashMap-based argument resolution
- **Subcommand support**: Hierarchical command structure
- **Short/long options**: `-f` and `--flag` syntax
- **Value arguments**: Flags with associated values

## Usage

```rust
use avila_cli::{App, Command, Arg};

fn main() {
    let matches = App::new("myapp")
        .version("1.0.0")
        .about("Application description")
        .arg(Arg::new("verbose")
            .short('v')
            .long("verbose")
            .help("Enable verbose output"))
        .command(Command::new("test")
            .about("Run tests")
            .arg(Arg::new("filter")
                .long("filter")
                .takes_value(true)
                .help("Test filter pattern")))
        .parse();

    if matches.is_present("verbose") {
        println!("Verbose mode enabled");
    }

    if let Some(cmd) = matches.subcommand() {
        match cmd {
            "test" => {
                if let Some(filter) = matches.value_of("filter") {
                    println!("Running tests with filter: {}", filter);
                }
            }
            _ => {}
        }
    }
}
```

## Architecture

### Parsing Algorithm

1. **Tokenization**: Split `std::env::args()` into argument tokens
2. **Command matching**: First positional argument checked against registered commands
3. **Argument parsing**: Iterate tokens, match against argument schema
4. **Value binding**: Store flags/values in HashMap for O(1) access

### Data Structures

- `App`: Application schema (name, version, commands, global args)
- `Command`: Subcommand schema (name, description, args)
- `Arg`: Argument definition (name, short/long forms, value type)
- `Matches`: Parse result (HashMap of arg names to values)

## Performance

- **Parse time**: O(n) where n = argument count
- **Lookup time**: O(1) via HashMap
- **Memory**: Stack-allocated parser + heap for argument storage

## License

MIT OR Apache-2.0
