//! Avila CLI - AVL Platform CLI framework
//! Replacement for clap - 100% Rust std
//! Simple command-line argument parsing

use std::collections::HashMap;
use std::env;

/// CLI Application
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
            version: "0.1.0".to_string(),
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

/// Command
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

/// Argument
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

/// Parsed matches
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
}
