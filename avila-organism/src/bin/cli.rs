//! CLI de email Ávila

use avila_terminal::Colorize;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("\n{}", "📬 Ávila Mail CLI".cyan().bold());
    println!("{}", "─".repeat(50).cyan());

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "send" => println!("{}", "📤 Enviando email...".green()),
        "list" => println!("{}", "📫 Listando emails...".green()),
        "read" => println!("{}", "📖 Lendo email...".green()),
        "help" | "--help" | "-h" => print_help(),
        _ => {
            println!("{}", format!("Comando desconhecido: {}", args[1]).red());
            print_help();
        }
    }
}

fn print_help() {
    println!("\n{}", "COMANDOS:".yellow().bold());
    println!("  {} [to] [subject]    Enviar email", "send".green());
    println!("  {}                     Listar emails", "list".green());
    println!("  {} [id]                Ler email", "read".green());
    println!("  {}                     Mostrar ajuda", "help".green());
    println!();
}
