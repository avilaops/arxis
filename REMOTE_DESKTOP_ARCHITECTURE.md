# 🖥️ Arquitetura de Desktop Remoto - AVL Platform

## Mapeamento de Componentes → Crates Arxis

### 1. **Agente (Host)** - Captura e streaming do desktop
**Crates responsáveis:**
- **`avx-image`** - Captura de tela e encoding de vídeo
  - Screen capture (Windows: DXGI, Linux: X11/Wayland, macOS: CoreGraphics)
  - Hardware encoding (H.264/H.265/AV1)
  - Frame difference detection
  - Cursor overlay

- **`avx-events`** - Captura de input remoto
  - Keyboard injection
  - Mouse control
  - Clipboard sync
  - File transfer events

- **`avx-telemetry`** - Métricas do agente
  - CPU/GPU usage
  - Network stats
  - Frame rate monitoring
  - Latency tracking

**Status:** ⚠️ Parcialmente implementado
**Prioridade:** 🔴 Alta (core do sistema)

---

### 2. **Cliente (Desktop/Web/Mobile)** - Interface do usuário
**Crates responsáveis:**
- **`landing-page-leptos`** - Cliente web (Leptos/WASM)
  - WebRTC receiver
  - Canvas rendering
  - Input capture (keyboard/mouse)
  - Touch gestures (mobile)

- **`avx-cli`** - Cliente CLI para automação
  - Headless connections
  - Scripting support
  - Session management

- **Future: `avx-desktop-client`** - Cliente nativo (Tauri/Dioxus)
  - Native performance
  - System tray integration
  - Multi-monitor support

**Status:** 🟡 Em desenvolvimento
**Prioridade:** 🔴 Alta

---

### 3. **Signaling / Control Plane** - Coordenação de sessões
**Crates responsáveis:**
- **`avx-gateway`** - API Gateway + Signaling
  - WebSocket signaling server
  - SDP/ICE exchange
  - Session negotiation
  - Device registration
  - Authentication integration

- **`aviladb`** - Persistência
  - Device inventory (hosts disponíveis)
  - Session state
  - User permissions
  - Audit logs

- **`avl-auth`** - Autenticação e autorização
  - OAuth2/OIDC
  - MFA (TOTP, WebAuthn)
  - Session tokens
  - Role-based access control

**Status:** ✅ Implementado (base)
**Prioridade:** 🟢 Médio (refinamento)

---

### 4. **NAT Traversal (STUN/TURN)** - Conectividade P2P
**Crates responsáveis:**
- **`avl-loadbalancer`** - STUN/TURN server
  - STUN server para NAT detection
  - TURN relay para fallback
  - ICE candidate gathering
  - Bandwidth management

- **`avx-http`** - HTTP/3 + QUIC transport
  - Low-latency transport
  - Multiplexing
  - Connection migration

**Status:** 🟡 Parcial (precisa STUN/TURN)
**Prioridade:** 🔴 Alta (essencial para NAT)

---

### 5. **Media/Streaming Layer** - Transporte de vídeo
**Crates responsáveis:**
- **`avx-gateway`** - WebRTC SFU (Selective Forwarding Unit)
  - WebRTC peer connections
  - RTP/RTCP handling
  - Congestion control (GCC)
  - Adaptive bitrate

- **`avila-compress`** - Compressão de frames
  - LZ4 para deltas pequenos
  - Parallel compression
  - Columnar encoding para bulk data

- **`avx-image`** - Video codecs
  - H.264/H.265 encoding/decoding
  - VP8/VP9 support
  - AV1 (future)

**Status:** 🟡 Em desenvolvimento
**Prioridade:** 🔴 Alta (core do streaming)

---

### 6. **Relay / Escalonamento (SFU/MCU)** - Multi-viewer
**Crates responsáveis:**
- **`avl-loadbalancer`** - SFU/MCU
  - Selective forwarding para múltiplos clientes
  - Load balancing entre relays
  - Geographic distribution
  - Session recording

- **`avl-queue`** - Message queue para eventos
  - Pub/sub para input events
  - Session coordination
  - Horizontal scaling

**Status:** 🟡 Base implementada
**Prioridade:** 🟡 Médio (para escala)

---

### 7. **Segurança & Auth** - Criptografia e auditoria
**Crates responsáveis:**
- **`avl-auth`** - Autenticação
  - JWT tokens
  - OAuth2 flows
  - MFA enforcement
  - Session management

- **`avl-secrets`** - Key management
  - Credential storage
  - Certificate management
  - Key rotation
  - Vault integration

- **`avx-telemetry`** - Audit logging
  - Security events
  - Access logs
  - Compliance tracking
  - Anomaly detection

**Status:** ✅ Base sólida
**Prioridade:** 🟢 Médio (hardening)

---

### 8. **Infra e Observabilidade** - DevOps
**Crates responsáveis:**
- **`avl-observability`** - Métricas e logs
  - Prometheus metrics
  - OpenTelemetry traces
  - Log aggregation
  - Alerting rules

- **`avx-telemetry`** - Application metrics
  - Latency tracking
  - Throughput monitoring
  - Error rates
  - Custom dashboards

- **`avx-config`** - Configuration management
  - Environment configs
  - Feature flags
  - Dynamic updates

**Status:** ✅ Implementado
**Prioridade:** 🟢 Baixo (manutenção)

---

### 9. **UX / Permissões** - Controle de acesso
**Crates responsáveis:**
- **`avl-console`** - Admin dashboard
  - Device management
  - Permission matrix
  - Session viewer
  - Audit trail UI

- **`avl-auth`** - Granular permissions
  - View-only mode
  - Control permissions
  - File transfer ACL
  - Time-limited sessions

- **`landing-page-leptos`** - User consent flow
  - Permission request UI
  - Accept/Deny workflow
  - Notification system

**Status:** 🟡 Parcial
**Prioridade:** 🟡 Médio (UX crítico)

---

## 📋 Roadmap de Implementação

### Phase 1: Core Streaming (4-6 semanas)
**Objetivo:** Conexão P2P básica funcionando

- [ ] **avx-image**: Screen capture + H.264 encoding
- [ ] **avx-gateway**: WebRTC signaling server
- [ ] **avl-loadbalancer**: STUN/TURN server
- [ ] **landing-page-leptos**: WebRTC receiver + canvas render
- [ ] **avx-events**: Input injection (keyboard/mouse)
- [ ] **aviladb**: Device inventory + session state

**Deliverable:** Usuário conecta via web e vê desktop remoto com controle básico

---

### Phase 2: Production Ready (3-4 semanas)
**Objetivo:** Sistema robusto e seguro

- [ ] **avl-auth**: MFA + OAuth2 integration
- [ ] **avl-secrets**: Certificate management
- [ ] **avx-telemetry**: Latency/quality metrics
- [ ] **avila-compress**: Frame delta compression
- [ ] **avx-gateway**: Adaptive bitrate
- [ ] **avl-console**: Admin dashboard básico

**Deliverable:** Produto MVP pronto para beta testing

---

### Phase 3: Scale & Features (4-6 semanas)
**Objetivo:** Multi-viewer e enterprise features

- [ ] **avl-loadbalancer**: SFU para múltiplos viewers
- [ ] **avl-queue**: Event streaming
- [ ] **avx-cli**: Headless automation
- [ ] **avx-image**: Multi-monitor support
- [ ] **avl-console**: Permission management UI
- [ ] **avl-observability**: Production monitoring

**Deliverable:** Sistema enterprise-grade com horizontal scaling

---

### Phase 4: Advanced (ongoing)
**Objetivo:** Diferenciação competitiva

- [ ] **avx-quantum-render**: AI upscaling para baixa latência
- [ ] **avx-gpu**: Hardware acceleration para encoding
- [ ] **avila-ml**: Anomaly detection para segurança
- [ ] **avx-desktop-client**: Native clients (Tauri)
- [ ] **avx-events**: File transfer + clipboard sync
- [ ] Recording/playback de sessões

**Deliverable:** Features premium que superam TeamViewer/AnyDesk

---

## 🎯 Priorização por Impacto

### 🔴 Critical Path (bloqueador para MVP)
1. **avx-image** - Screen capture + encoding
2. **avx-gateway** - WebRTC signaling
3. **avl-loadbalancer** - STUN/TURN
4. **landing-page-leptos** - Cliente web
5. **avx-events** - Input injection

### 🟡 Important (necessário para produção)
6. **avl-auth** - MFA + OAuth2
7. **aviladb** - Session persistence
8. **avx-telemetry** - Metrics
9. **avila-compress** - Bandwidth optimization
10. **avl-console** - Admin UI

### 🟢 Nice to Have (diferenciação)
11. **avl-queue** - Horizontal scaling
12. **avx-cli** - Automation
13. **avx-gpu** - Hardware accel
14. **avila-ml** - AI features

---

## 🏗️ Arquitetura de Conexão

```
┌─────────────────────────────────────────────────────────────┐
│                      Internet / NAT                          │
└─────────────────────────────────────────────────────────────┘
                            │
        ┌───────────────────┼───────────────────┐
        │                   │                   │
   ┌────▼─────┐      ┌──────▼──────┐     ┌────▼─────┐
   │   STUN   │      │  Signaling  │     │   TURN   │
   │  Server  │      │   Gateway   │     │  Relay   │
   └──────────┘      └──────────────┘     └──────────┘
 (avl-loadbalancer)    (avx-gateway)   (avl-loadbalancer)
        │                   │                   │
        └───────────────────┼───────────────────┘
                            │
                ┌───────────┴───────────┐
                │                       │
         ┌──────▼──────┐         ┌─────▼──────┐
         │   Cliente   │◄────────┤   Agente   │
         │ (Browser/   │  WebRTC │   (Host)   │
         │   Native)   │   P2P   │            │
         └─────────────┘         └────────────┘
      (landing-page-leptos)      (avx-image +
                                  avx-events)
```

---

## 🔧 Stack Tecnológico

### Backend (Rust)
- **Web framework:** Axum (avx-gateway)
- **WebRTC:** webrtc-rs ou mediasoup-rs
- **Database:** AvilaDB + PostgreSQL (aviladb)
- **Message queue:** Redis Streams (avl-queue)
- **Metrics:** Prometheus + OpenTelemetry

### Frontend (Web)
- **Framework:** Leptos (WASM)
- **WebRTC:** web-sys bindings
- **Canvas:** 2D rendering
- **Styling:** TailwindCSS

### Native Clients (Future)
- **Desktop:** Tauri + Leptos
- **Mobile:** React Native (WebRTC bridge)

---

## 🚀 Próximos Passos Imediatos

1. **Revisar avx-image** - Implementar screen capture básico
2. **Adicionar WebRTC signaling em avx-gateway**
3. **Implementar STUN server em avl-loadbalancer**
4. **Criar cliente web básico em landing-page-leptos**
5. **Testar conexão P2P ponta-a-ponta**

---

**Arquiteto:** GitHub Copilot
**Data:** 2025-11-23
**Status:** 📋 Plano aprovado para execução
