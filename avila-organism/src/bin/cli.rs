//! Ávila email CLI

use avila_terminal::Colorize;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("\n{}", "Ávila Mail CLI".cyan().bold());
    println!("{}", "=".repeat(50).cyan());

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "send" => println!("{}", "Sending email...".green()),
        "list" => println!("{}", "Listing emails...".green()),
        "read" => println!("{}", "Reading email...".green()),
        "help" | "--help" | "-h" => print_help(),
        _ => {
            println!("{}", format!("Unknown command: {}", args[1]).red());
            print_help();
        }
    }
}

fn print_help() {
    println!("\n{}", "COMMANDS:".yellow().bold());
    println!("  {} [to] [subject]    Send email", "send".green());
    println!("  {}                     List emails", "list".green());
    println!("  {} [id]                Read email", "read".green());
    println!("  {}                     Show help", "help".green());
    println!();
}
