//! Ávila CLI Parser
//!
//! Zero-dependency command-line argument parser with stack-allocated data structures.
//! Provides compile-time type safety and constant-time argument lookups via HashMap.

use std::collections::HashMap;
use std::env;

/// Command-line application parser
///
/// Stack-allocated structure that defines the command-line interface schema.
/// All fields use heap-allocated collections for dynamic argument counts,
/// but the parser itself is deterministic and type-safe.
pub struct App {
    name: String,
    version: String,
    about: String,
    commands: Vec<Command>,
    global_args: Vec<Arg>,
}

impl App {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: "0.2.0".to_string(),
            about: String::new(),
            commands: Vec::new(),
            global_args: Vec::new(),
        }
    }

    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    pub fn about(mut self, about: impl Into<String>) -> Self {
        self.about = about.into();
        self
    }

    pub fn command(mut self, cmd: Command) -> Self {
        self.commands.push(cmd);
        self
    }

    pub fn arg(mut self, arg: Arg) -> Self {
        self.global_args.push(arg);
        self
    }

    pub fn parse(self) -> Matches {
        let args: Vec<String> = env::args().skip(1).collect();
        self.parse_args(&args)
    }

    fn parse_args(self, args: &[String]) -> Matches {
        let mut matches = Matches {
            command: None,
            args: HashMap::new(),
            values: Vec::new(),
        };

        if args.is_empty() {
            return matches;
        }

        // Check for help/version
        if args[0] == "--help" || args[0] == "-h" {
            self.print_help();
            std::process::exit(0);
        }
        if args[0] == "--version" || args[0] == "-V" {
            println!("{} {}", self.name, self.version);
            std::process::exit(0);
        }

        // Parse command
        if let Some(cmd) = self.commands.iter().find(|c| c.name == args[0]) {
            matches.command = Some(args[0].clone());
            matches.parse_command_args(cmd, &args[1..]);
        } else {
            matches.parse_args_list(&self.global_args, args);
        }

        matches
    }

    fn print_help(&self) {
        println!("{}", self.name);
        if !self.about.is_empty() {
            println!("{}\n", self.about);
        }
        println!("Usage: {} [OPTIONS] [COMMAND]\n", self.name.to_lowercase());

        if !self.commands.is_empty() {
            println!("Commands:");
            for cmd in &self.commands {
                println!("  {:<12} {}", cmd.name, cmd.about);
            }
            println!();
        }

        println!("Options:");
        println!("  -h, --help     Print help");
        println!("  -V, --version  Print version");

        for arg in &self.global_args {
            let short = arg.short.as_ref().map(|s| format!("-{}, ", s)).unwrap_or_default();
            println!("  {}{:<12} {}", short, format!("--{}", arg.long), arg.help);
        }
    }
}

/// Subcommand definition
///
/// Represents a distinct command with its own argument schema.
/// Commands are parsed from the first positional argument.
pub struct Command {
    name: String,
    about: String,
    args: Vec<Arg>,
}

impl Command {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            about: String::new(),
            args: Vec::new(),
        }
    }

    pub fn about(mut self, about: impl Into<String>) -> Self {
        self.about = about.into();
        self
    }

    pub fn arg(mut self, arg: Arg) -> Self {
        self.args.push(arg);
        self
    }
}

/// Command-line argument specification
///
/// Defines a flag or option with optional short/long forms.
/// Can be boolean (flag) or value-taking (option).
pub struct Arg {
    name: String,
    long: String,
    short: Option<String>,
    help: String,
    takes_value: bool,
    required: bool,
}

impl Arg {
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            long: name.clone(),
            name,
            short: None,
            help: String::new(),
            takes_value: false,
            required: false,
        }
    }

    pub fn long(mut self, long: impl Into<String>) -> Self {
        self.long = long.into();
        self
    }

    pub fn short(mut self, short: char) -> Self {
        self.short = Some(short.to_string());
        self
    }

    pub fn help(mut self, help: impl Into<String>) -> Self {
        self.help = help.into();
        self
    }

    pub fn takes_value(mut self, takes: bool) -> Self {
        self.takes_value = takes;
        self
    }

    pub fn required(mut self, req: bool) -> Self {
        self.required = req;
        self
    }
}

/// Parse result containing matched arguments
///
/// Uses HashMap for O(1) argument lookups.
/// Stores the active subcommand and all parsed argument values.
pub struct Matches {
    command: Option<String>,
    args: HashMap<String, Option<String>>,
    values: Vec<String>,
}

impl Matches {
    pub fn subcommand(&self) -> Option<&str> {
        self.command.as_deref()
    }

    pub fn is_present(&self, name: &str) -> bool {
        self.args.contains_key(name)
    }

    pub fn value_of(&self, name: &str) -> Option<&str> {
        self.args.get(name)?.as_deref()
    }

    pub fn values(&self) -> &[String] {
        &self.values
    }

    /// Get parsed value as specific type
    /// 
    /// # Example
    /// ```
    /// let port: u16 = matches.value_as("port").unwrap_or(8080);
    /// ```
    pub fn value_as<T>(&self, name: &str) -> Option<T>
    where
        T: std::str::FromStr,
    {
        self.value_of(name)?.parse().ok()
    }

    /// Check if any of the given argument names is present
    /// 
    /// # Example
    /// ```
    /// if matches.any_present(&["verbose", "debug"]) {
    ///     println!("Logging enabled");
    /// }
    /// ```
    pub fn any_present(&self, names: &[&str]) -> bool {
        names.iter().any(|name| self.is_present(name))
    }

    /// Check if all of the given argument names are present
    pub fn all_present(&self, names: &[&str]) -> bool {
        names.iter().all(|name| self.is_present(name))
    }

    /// Get value or return a default
    pub fn value_or<'a>(&'a self, name: &str, default: &'a str) -> &'a str {
        self.value_of(name).unwrap_or(default)
    }

    /// Get the number of positional arguments
    pub fn values_count(&self) -> usize {
        self.values.len()
    }

    fn parse_command_args(&mut self, cmd: &Command, args: &[String]) {
        self.parse_args_list(&cmd.args, args);
    }

    fn parse_args_list(&mut self, arg_defs: &[Arg], args: &[String]) {
        let mut i = 0;
        while i < args.len() {
            let arg = &args[i];

            if arg.starts_with("--") {
                let key = &arg[2..];
                if let Some(arg_def) = arg_defs.iter().find(|a| a.long == key) {
                    if arg_def.takes_value && i + 1 < args.len() {
                        self.args.insert(arg_def.name.clone(), Some(args[i + 1].clone()));
                        i += 2;
                    } else {
                        self.args.insert(arg_def.name.clone(), None);
                        i += 1;
                    }
                } else {
                    i += 1;
                }
            } else if arg.starts_with('-') && arg.len() == 2 {
                let short = &arg[1..];
                if let Some(arg_def) = arg_defs.iter().find(|a| a.short.as_deref() == Some(short)) {
                    if arg_def.takes_value && i + 1 < args.len() {
                        self.args.insert(arg_def.name.clone(), Some(args[i + 1].clone()));
                        i += 2;
                    } else {
                        self.args.insert(arg_def.name.clone(), None);
                        i += 1;
                    }
                } else {
                    i += 1;
                }
            } else {
                self.values.push(arg.clone());
                i += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arg_creation() {
        let arg = Arg::new("test")
            .long("test")
            .short('t')
            .help("Test argument")
            .takes_value(true);

        assert_eq!(arg.name, "test");
        assert_eq!(arg.long, "test");
        assert_eq!(arg.short, Some("t".to_string()));
    }

    #[test]
    fn test_command_creation() {
        let cmd = Command::new("test")
            .about("Test command")
            .arg(Arg::new("arg1"));

        assert_eq!(cmd.name, "test");
        assert_eq!(cmd.args.len(), 1);
    }

    #[test]
    fn test_value_as_parsing() {
        let mut matches = Matches {
            command: None,
            args: HashMap::new(),
            values: Vec::new(),
        };
        matches.args.insert("port".to_string(), Some("8080".to_string()));
        
        let port: u16 = matches.value_as("port").unwrap();
        assert_eq!(port, 8080);
    }

    #[test]
    fn test_any_present() {
        let mut matches = Matches {
            command: None,
            args: HashMap::new(),
            values: Vec::new(),
        };
        matches.args.insert("verbose".to_string(), None);
        
        assert!(matches.any_present(&["verbose", "debug"]));
        assert!(!matches.any_present(&["quiet", "silent"]));
    }

    #[test]
    fn test_all_present() {
        let mut matches = Matches {
            command: None,
            args: HashMap::new(),
            values: Vec::new(),
        };
        matches.args.insert("verbose".to_string(), None);
        matches.args.insert("debug".to_string(), None);
        
        assert!(matches.all_present(&["verbose", "debug"]));
        assert!(!matches.all_present(&["verbose", "debug", "trace"]));
    }

    #[test]
    fn test_value_or_default() {
        let matches = Matches {
            command: None,
            args: HashMap::new(),
            values: Vec::new(),
        };
        
        assert_eq!(matches.value_or("port", "8080"), "8080");
    }

    #[test]
    fn test_values_count() {
        let matches = Matches {
            command: None,
            args: HashMap::new(),
            values: vec!["file1".to_string(), "file2".to_string()],
        };
        
        assert_eq!(matches.values_count(), 2);
    }
}
