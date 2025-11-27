//! Servidor de email Ávila

use avila_organism::{PlatformConfig, webmail, api, admin};
use avila_organ::server::EmailServer;
use avila_tissue::storage::EmailStorage;
use avila_terminal::Colorize;
use axum::Router;
use tokio::net::TcpListener;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = PlatformConfig::default();

    println!("\n{}", "╔══════════════════════════════════════════════╗".cyan());
    println!("{}", "║    🧬 ÁVILA ORGANISM - Email Platform 🧬    ║".cyan().bold());
    println!("{}", "╚══════════════════════════════════════════════╝".cyan());
    println!();
    println!("{}", "📡 Iniciando servidores...".green());
    println!("   • SMTP: {}", format!(":{}", config.smtp_port).yellow());
    println!("   • IMAP: {}", format!(":{}", config.imap_port).yellow());
    println!("   • HTTP: {}", format!(":{}", config.http_port).yellow());
    println!();
    println!("{}", "🧬 Hierarquia Biológica Ativada:".cyan());
    println!("   {} Partículas fundamentais (bits, bytes)", "⚛️  Nucleus".bold());
    println!("   {} Estruturas de dados (Option, Result, Vec)", "🔬 Atom".bold());
    println!("   {} Protocolos de rede (TCP, UDP, TLS)", "🧪 Molecule".bold());
    println!("   {} Protocolos de email (SMTP, IMAP, POP3)", "🦠 Cell".bold());
    println!("   {} Storage & indexação", "🧵 Tissue".bold());
    println!("   {} Servidor & cliente", "🫀 Organ".bold());
    println!("   {} Plataforma completa", "🧬 Organism".bold());
    println!();

    // Criar storage compartilhado
    let storage = EmailStorage::new();

    // Criar servidor de email
    let email_server = Arc::new(EmailServer::new(
        config.smtp_port,
        config.imap_port,
        storage,
    ));

    // Iniciar servidores SMTP e IMAP em background
    let email_server_task = Arc::clone(&email_server);
    tokio::spawn(async move {
        if let Err(e) = email_server_task.start().await {
            eprintln!("❌ Email server error: {}", e);
        }
    });

    // Combina todas as rotas HTTP
    let app = Router::new()
        .merge(webmail::routes())
        .merge(api::routes())
        .merge(admin::routes());

    // Inicia servidor HTTP
    let addr = format!("0.0.0.0:{}", config.http_port);
    let listener = TcpListener::bind(&addr).await?;

    println!("{}", format!("✅ Servidor HTTP em http://{}", addr).green().bold());
    println!("{}", "🚀 Plataforma de email 100% Ávila Platform - Zero dependências no core!".green());
    println!();

    axum::serve(listener, app).await?;

    Ok(())
}
