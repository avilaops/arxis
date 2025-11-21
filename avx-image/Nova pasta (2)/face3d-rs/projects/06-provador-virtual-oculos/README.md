# 👓 Projeto 6: Provador Virtual de Óculos para Comunidades Carentes

## 🎯 Objetivo
Sistema de realidade aumentada (AR) que permite testar armações de óculos virtualmente, eliminando necessidade de deslocamento a óticas e democratizando acesso.

## 💡 O Problema

### Estatísticas Brasil
- 👁️ **35 milhões** de brasileiros precisam óculos (IBGE)
- 💰 **6 milhões** não têm condições de comprar
- 🏥 **SUS**: Fila de 6-12 meses para óculos gratuitos
- 🚌 **Comunidades remotas**: Sem óticas próximas

### Barreiras de Acesso
1. **Deslocamento**: Custo transporte para experimentar
2. **Tempo**: Filas em óticas sociais
3. **Variedade**: Poucas opções nas óticas públicas
4. **Constrangimento**: Experimentar muitos modelos

## 💡 Como Funciona

### Pipeline AR

#### 1. Setup (Uma vez)
```rust
use face3d_rs::ar_glasses::*;

// Escanear rosto do usuário (smartphone)
let face_model = FaceTracker::from_camera()?
    .detect_face()?
    .build_3d_model()?;

// Medir distâncias críticas
let measurements = FaceMeasurements {
    pupillary_distance: face_model.measure_pd()?,  // Distância pupilas
    temple_width: face_model.measure_temple()?,    // Largura têmpora
    nose_bridge: face_model.measure_bridge()?,     // Ponte nasal
    face_width: face_model.measure_width()?,       // Largura rosto
};

// Recomendar tamanhos
let recommended_size = GlassesSizeRecommender::suggest(&measurements)?;
```

#### 2. Provador Virtual (Tempo Real)
```rust
// Tracking em tempo real (60 FPS)
let mut ar_session = ARSession::new(camera)?;

loop {
    // 1. Detectar face e landmarks
    let face_pose = ar_session.track_face()?;
    
    // 2. Posicionar armação 3D
    let glasses_transform = calculate_glasses_position(
        &face_pose,
        &measurements,
        &selected_frame,
    )?;
    
    // 3. Renderizar com iluminação realista
    let rendered = ar_session.render_ar(
        glasses_model: &selected_frame.model_3d,
        transform: glasses_transform,
        lighting: face_pose.ambient_light,
    )?;
    
    // 4. Exibir na tela
    display.show(rendered)?;
    
    // Capturar foto se usuário quiser
    if user_pressed_capture() {
        save_photo("oculos_modelo_X.jpg", rendered)?;
    }
}
```

#### 3. Pedido + Entrega
```rust
// Após escolher modelo
struct Order {
    frame_model: String,
    lens_prescription: Prescription,  // Receita médica
    delivery_address: Address,
    payment: PaymentMethod,
}

impl Order {
    fn calculate_total(&self) -> Money {
        let frame_cost = Money::new(50, BRL);  // Custo real
        let lens_cost = self.lens_prescription.complexity_cost();
        
        // Subsídio para baixa renda
        let discount = if self.is_low_income()? {
            Money::new(50, BRL)  // R$ 50 de desconto
        } else {
            Money::zero()
        };
        
        frame_cost + lens_cost - discount
    }
}
```

### Output
- App mobile com AR glasses try-on
- Recomendações personalizadas
- Fotos para compartilhar (opinião amigos/família)
- Pedido online com receita médica
- Entrega em casa ou ponto de retirada

## 🏗️ Arquitetura Técnica

```
┌──────────────────┐
│  Mobile App      │ React Native + AR
│  (iOS/Android)   │ (ARKit/ARCore)
└────────┬─────────┘
         │
┌────────▼─────────┐
│  AVL Gateway     │ API REST
│                  │
└────────┬─────────┘
         │
┌────────▼─────────┐
│  face3d-rs       │ 3D face modeling
│  + Glasses DB    │ + size matching
└────────┬─────────┘
         │
┌────────▼─────────┐
│  AvilaDB         │ User profiles
│                  │ + Order history
└────────┬─────────┘
         │
┌────────▼─────────┐
│  Parceiros       │ Óticas Sociais
│  (Fulfillment)   │ + Laboratórios
└──────────────────┘
```

## 📊 Stack Tecnológica

### Mobile AR
- **React Native**: App cross-platform
- **ARKit** (iOS): Face tracking nativo
- **ARCore** (Android): AR do Google
- **Three.js/Babylon.js**: Renderização 3D
- **React Native AR**: Biblioteca AR

### Backend
- **face3d-rs**: Modelagem facial + medidas
- **Axum**: API REST
- **AvilaDB**: Perfis + pedidos

### Catálogo 3D
- **Blender**: Modelagem armações
- **glTF 2.0**: Formato 3D otimizado (AR)
- **CDN**: Distribuição modelos 3D

### Integrações
- **PagSeguro/Mercado Pago**: Pagamento
- **Correios API**: Rastreamento entrega
- **WhatsApp Business**: Atendimento

## 🚀 MVP - Funcionalidades

### Fase 1 (2 meses) - Protótipo
- [ ] App mobile (iOS/Android)
- [ ] AR try-on básico (10 modelos)
- [ ] Medição automática PD
- [ ] Recomendação de tamanho

### Fase 2 (4 meses) - Produto
- [ ] 100 modelos de armações
- [ ] Upload receita médica (foto)
- [ ] Checkout + pagamento
- [ ] Parceria 3 óticas sociais

### Fase 3 (12 meses) - Escala
- [ ] 500+ modelos
- [ ] Programa de subsídio baixa renda
- [ ] 50 óticas parceiras (nacional)
- [ ] Entrega expressa (7 dias)

## 💰 Modelo de Negócio

### Pricing Social

#### Para Usuários
- **App**: 100% gratuito
- **Óculos completo**: R$ 150-300
  - Armação: R$ 50-100
  - Lentes: R$ 100-200
- **Desconto baixa renda**: -R$ 50 (com CadÚnico)
- **SUS (gratuito)**: Parceria prefeituras

#### Para Óticas Parceiras (B2B)
- **Software gratuito**: Licença para AR try-on
- **Comissão**: 15% por pedido
- **Leads qualificados**: Usuários que já experimentaram virtualmente

#### Comparação Custos

| Item | Ótica Tradicional | Nossa Solução | Economia |
|------|-------------------|---------------|----------|
| Óculos completo | R$ 400-800 | R$ 150-300 | **60%** |
| Deslocamentos | R$ 20-50 | R$ 0 | **100%** |
| Tempo | 3h (ida + fila) | 15min (casa) | **95%** |

### Sustentabilidade
- **Revenue**: Comissão 15% × 10k pedidos/mês = R$ 225k/mês
- **Custos**: Dev R$ 60k + Infra R$ 10k = R$ 70k/mês
- **Margem**: R$ 155k/mês (69%)

## 🤝 Parcerias Estratégicas

### Óticas Sociais
1. **Óticas Social** - Rede nacional (50 lojas)
2. **Visão Solidária** - Programa social
3. **Ver Bem** - Ótica popular
4. **Lenscope** - Online (baixo custo)

### Governo
1. **Prefeituras**: Programa "Óculos Cidadão"
2. **Ministério da Saúde**: Integração SUS
3. **CadÚnico**: Validação baixa renda
4. **Correios**: Logística subsidiada

### Corporativo
1. **Essilor/Luxottica**: Fornecedor lentes
2. **Facebook/Meta**: SDK AR
3. **Google**: ARCore + Cloud Anchors

### ONGs
1. **Optical Social**: ONG que doa óculos
2. **Enxergar Melhor**: Mutirões vista
3. **Instituto Ver e Viver**

## 📈 Métricas de Impacto

### Objetivos Ano 1
- 👓 **50k óculos** entregues
- 🏘️ **500 comunidades** atendidas
- 💰 **R$ 10M economia** para famílias
- ⭐ **4.5+ rating** no app

### KPIs
- Downloads do app
- Conversion rate (try-on → pedido)
- NPS (satisfação)
- Tempo médio até entrega
- Taxa de devolução (ajuste)

## 🛡️ Privacidade & Segurança

### Dados Sensíveis
- 🔒 **Face data**: Processado localmente (não sai do celular)
- 📜 **Receita médica**: Criptografada (LGPD)
- 💳 **Pagamento**: PCI DSS compliant
- 🗑️ **Retenção**: Deletado após 90 dias (pedido completo)

### Segurança AR
- ✅ Sem coleta de fotos faciais (apenas medidas)
- ✅ Processamento local (ARKit/ARCore)
- ✅ Opt-in para analytics
- ✅ Transparência total (código open source)

## 📚 Tecnologia AR Detalhada

### Medição Automática
```rust
impl FaceMeasurements {
    fn measure_pupillary_distance(landmarks: &FaceLandmarks) -> Result<f32> {
        let left_pupil = landmarks.left_eye_center;
        let right_pupil = landmarks.right_eye_center;
        
        // Distância euclidiana 3D
        let distance_mm = left_pupil.distance(&right_pupil) * SCALE_FACTOR;
        
        // Validação (range normal: 54-74mm)
        if distance_mm < 50.0 || distance_mm > 80.0 {
            return Err(Error::InvalidMeasurement);
        }
        
        Ok(distance_mm)
    }
    
    fn recommend_frame_size(&self) -> FrameSize {
        // Algoritmo baseado em optometria
        match self.face_width {
            w if w < 120.0 => FrameSize::Small,
            w if w < 135.0 => FrameSize::Medium,
            _ => FrameSize::Large,
        }
    }
}
```

### Posicionamento Realista
```rust
fn calculate_glasses_position(
    face_pose: &FacePose,
    measurements: &FaceMeasurements,
    frame: &GlassesModel,
) -> Transform3D {
    // 1. Posição vertical (ponte nasal)
    let nose_bridge_height = face_pose.landmarks.nose_bridge.y;
    
    // 2. Rotação (inclinar com rosto)
    let head_rotation = face_pose.rotation;
    
    // 3. Escala (ajustar ao tamanho do rosto)
    let scale = measurements.face_width / frame.default_width;
    
    Transform3D {
        position: Vector3::new(0.0, nose_bridge_height, -measurements.nose_bridge),
        rotation: head_rotation,
        scale: Vector3::splat(scale),
    }
}
```

## 🎓 Equipe Necessária

### Tech (5 pessoas)
- 1 Tech Lead (AR + 3D)
- 2 Mobile Developers (React Native + AR)
- 1 Rust Developer (face3d-rs)
- 1 3D Artist (modelagem armações)

### Operações (3 pessoas)
- 1 Product Manager
- 1 Parcerias (óticas)
- 1 Customer Success

**Custo**: R$ 60k/mês

## 🗓️ Roadmap

### Q1 2026 - MVP
- App funcional (iOS/Android)
- 10 modelos AR
- Medição automática
- Beta: 1.000 usuários (SP)

### Q2 2026 - Launch
- 100 modelos armações
- Checkout + pagamento
- 3 óticas parceiras
- Launch público nacional

### Q3 2026 - Growth
- 300 modelos
- Programa baixa renda
- 10 óticas parceiras
- 10k pedidos

### Q4 2026 - Scale
- 500 modelos
- 50 óticas parceiras
- Integração SUS (piloto)
- 50k pedidos, R$ 225k MRR

## 🌟 Diferenciais

### vs Warby Parker / Lenscope
- ✅ **Foco social**: Subsídio para baixa renda
- ✅ **Localizado**: Parcerias óticas locais
- ✅ **Preço**: 50% mais barato
- ✅ **Acessibilidade**: Interface simples

### vs Ótica Física
- ⚡ **Conveniência**: Testar em casa
- 💰 **Preço**: Sem intermediários
- 🎨 **Variedade**: 500+ modelos vs 50
- ⏰ **Tempo**: 15min vs 3h

## 💬 Depoimentos (Simulados)

> *"Moro em comunidade rural. A ótica mais próxima fica a 60km. Com o app, experimentei 20 modelos sem sair de casa. Recebi em 1 semana."*  
> — Ana, 34 anos, Professora (CE)

> *"Tenho 3 filhos e todos precisavam de óculos. Na ótica custaria R$ 1.200. Aqui paguei R$ 450. Salvou nosso orçamento!"*  
> — Carlos, Pai (PE)

## 📞 Como Participar

### Para Usuários
- 📱 **Download**: oculos.avila.cloud
- 🆓 **Sempre gratuito**: App
- 🏷️ **CadÚnico**: Ganhe R$ 50 de desconto

### Para Óticas
- 🤝 **Parceria**: Aumente suas vendas
- 📧 **Contato**: parceiros@avila.cloud
- 💰 **Sem custo fixo**: Apenas comissão

### Para Prefeituras
- 🏛️ **Programa Social**: "Óculos Cidadão"
- 📄 **Proposta**: governo@avila.cloud

---

## 🚀 Call to Action

**Ver bem é um direito, não um privilégio.**

Vamos levar acesso a óculos de qualidade para todo Brasil.

**#VerMelhorÉPossível**

---

*Desenvolvido com visão por Avila.inc* 👁️
