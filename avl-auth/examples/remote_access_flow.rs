//! Exemplo: Fluxo de Autenticação para Acesso Remoto
//!
//! Demonstra fluxo completo:
//! 1. Registro de dispositivos
//! 2. Solicitação de acesso
//! 3. Aprovação/negação
//! 4. Gestão de sessão
//! 5. Audit trail

use avl_auth::device_trust::*;
use std::collections::HashMap;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔐 AVL Auth - Remote Access Demo\n");

    let manager = DeviceAuthManager::new();

    // 1. Registrar dispositivo do técnico de suporte
    println!("1️⃣  Registrando dispositivo de suporte");
    let support_device = DeviceInfo {
        device_id: "support-laptop-001".into(),
        name: "Laptop Suporte TI".into(),
        device_type: DeviceType::Desktop,
        os: "Windows".into(),
        os_version: "11 Pro".into(),
        fingerprint: "hw:abc123:tpm:xyz789".into(),
        user_agent: Some("AvilaRemote/1.0".into()),
        ip_address: Some("200.1.2.3".into()),
        geo_location: Some(GeoLocation {
            latitude: -23.5505,
            longitude: -46.6333,
            city: Some("São Paulo".into()),
            country: "BR".into(),
        }),
        metadata: {
            let mut m = HashMap::new();
            m.insert("department".into(), "IT Support".into());
            m.insert("tpm_enabled".into(), "true".into());
            m
        },
    };

    let support_dev = manager
        .register_device("tech-user@company.com".into(), support_device.clone())
        .await?;

    println!("   ✅ Dispositivo registrado: {}", support_dev.info.name);
    println!("      Trust Level: {:?}", support_dev.trust_level);
    println!();

    // 2. Registrar dispositivo do usuário final
    println!("2️⃣  Registrando dispositivo do usuário");
    let user_device = DeviceInfo {
        device_id: "user-desktop-042".into(),
        name: "Desktop Usuário - Marketing".into(),
        device_type: DeviceType::Desktop,
        os: "Windows".into(),
        os_version: "11 Home".into(),
        fingerprint: "hw:def456:tpm:abc123".into(),
        user_agent: None,
        ip_address: Some("192.168.1.105".into()),
        geo_location: Some(GeoLocation {
            latitude: -23.5505,
            longitude: -46.6333,
            city: Some("São Paulo".into()),
            country: "BR".into(),
        }),
        metadata: {
            let mut m = HashMap::new();
            m.insert("department".into(), "Marketing".into());
            m
        },
    };

    let user_dev = manager
        .register_device("user@company.com".into(), user_device)
        .await?;

    println!("   ✅ Dispositivo registrado: {}", user_dev.info.name);
    println!();

    // 3. Solicitar acesso remoto
    println!("3️⃣  Técnico solicita acesso remoto");
    let request = manager
        .request_remote_access(
            support_device,
            user_dev.info.device_id.clone(),
            "tech-user@company.com".into(),
            "user@company.com".into(),
            AccessType::Support,
            Duration::from_secs(1800), // 30 minutos
            Some("Suporte técnico - instalação de software".into()),
        )
        .await?;

    println!("   📋 Solicitação criada: {}", request.request_id);
    println!("      Tipo: {:?}", request.access_type);
    println!("      Duração: {} minutos", request.requested_duration.as_secs() / 60);
    if let Some(reason) = &request.reason {
        println!("      Motivo: {}", reason);
    }
    println!();

    // 4. Usuário aprova solicitação
    println!("4️⃣  Usuário aprova acesso");
    let session = manager
        .approve_request(request.request_id.clone(), "user@company.com".into())
        .await?;

    println!("   ✅ Sessão iniciada: {}", session.session_id);
    println!("      Iniciada: {:?}", session.started_at);
    println!("      Expira: {:?}", session.expires_at);
    println!();

    // 5. Registrar eventos da sessão
    println!("5️⃣  Registrando eventos da sessão");
    
    manager
        .log_session_event(
            session.session_id.clone(),
            "connection_established".into(),
            "Conexão WebRTC estabelecida".into(),
            {
                let mut m = HashMap::new();
                m.insert("protocol".into(), "WebRTC".into());
                m.insert("codec".into(), "VP9".into());
                m
            },
        )
        .await?;

    println!("   📝 Evento: Conexão estabelecida");

    manager
        .log_session_event(
            session.session_id.clone(),
            "screen_shared".into(),
            "Compartilhamento de tela iniciado".into(),
            HashMap::new(),
        )
        .await?;

    println!("   📝 Evento: Tela compartilhada");

    manager
        .log_session_event(
            session.session_id.clone(),
            "file_transferred".into(),
            "Arquivo transferido".into(),
            {
                let mut m = HashMap::new();
                m.insert("filename".into(), "installer.exe".into());
                m.insert("size_bytes".into(), "15728640".into());
                m
            },
        )
        .await?;

    println!("   📝 Evento: Arquivo transferido");
    println!();

    // 6. Validar sessão
    println!("6️⃣  Validando sessão");
    let is_valid = manager.validate_session(session.session_id.clone()).await?;
    println!("   {} Sessão válida: {}", if is_valid { "✅" } else { "❌" }, is_valid);
    println!();

    // 7. Encerrar sessão
    println!("7️⃣  Encerrando sessão");
    manager.terminate_session(session.session_id.clone()).await?;
    println!("   ✅ Sessão encerrada");
    println!();

    // 8. Demonstrar cálculo de trust level
    println!("8️⃣  Evolução de Trust Level");
    
    let mut evolving_device = support_dev.clone();
    
    println!("   Dispositivo novo:");
    println!("      Acessos: {}, Trust: {:?}", 
        evolving_device.access_count, 
        manager.calculate_trust_level(&evolving_device));

    evolving_device.access_count = 15;
    println!("   Após 15 acessos:");
    println!("      Acessos: {}, Trust: {:?}", 
        evolving_device.access_count, 
        manager.calculate_trust_level(&evolving_device));

    evolving_device.access_count = 60;
    println!("   Após 60 acessos:");
    println!("      Acessos: {}, Trust: {:?}", 
        evolving_device.access_count, 
        manager.calculate_trust_level(&evolving_device));

    evolving_device.access_count = 150;
    evolving_device.certificate = Some("X.509 Certificate".into());
    println!("   Com certificado e 150 acessos:");
    println!("      Acessos: {}, Trust: {:?}", 
        evolving_device.access_count, 
        manager.calculate_trust_level(&evolving_device));
    println!();

    println!("🎉 Fluxo de autenticação remota completo!");
    println!("\n💡 Features demonstradas:");
    println!("   • Registro de dispositivos");
    println!("   • Device fingerprinting");
    println!("   • Solicitação com contexto");
    println!("   • Aprovação explícita");
    println!("   • Sessões time-limited");
    println!("   • Audit trail completo");
    println!("   • Trust level dinâmico");

    Ok(())
}
