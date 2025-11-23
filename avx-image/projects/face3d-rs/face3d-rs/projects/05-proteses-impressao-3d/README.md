# 📱 Projeto 5: Próteses Faciais Customizadas (Impressão 3D)

## 🎯 Objetivo
Democratizar acesso a próteses faciais através de design 3D assistido por IA e impressão 3D de baixo custo.

## 💡 O Problema

### Estatísticas Brasil
- 🎗️ **200k+ pessoas** com deformidades faciais (INCA)
- 💰 **Próteses convencionais**: R$ 5k - R$ 30k
- ⏳ **Tempo fabricação**: 2-6 meses
- 🏥 **SUS**: Fila de 2+ anos, disponibilidade limitada

### Causas Comuns
1. **Câncer** (50%) - Remoção nariz, orelha, olho
2. **Acidentes** (30%) - Trânsito, trabalho, queimaduras
3. **Violência** (10%) - Agressões, mutilações
4. **Congênito** (10%) - Microtia, malformações

### Impacto Psicossocial
- 😔 Depressão e isolamento social
- 💼 Dificuldade de empregabilidade
- 💔 Autoestima destruída
- 🚪 Exclusão da vida pública

## 💡 Como Funciona

### Pipeline Completo

#### 1. Escaneamento (Input)
```rust
enum ScanMethod {
    Smartphone3D,      // Face ID (iPhone), ARCore (Android)
    StructuredLight,   // Intel RealSense, Kinect
    Photogrammetry,    // 20-50 fotos
    CTScan,           // Tomografia (hospitalar)
}
```

#### 2. Design Assistido por IA
```rust
use face3d_rs::prosthetics::*;

// 1. Carregar scan do paciente
let patient_scan = Scan3D::from_smartphone("scan.obj")?;

// 2. Detectar área faltante
let missing_region = patient_scan.detect_defect()?;

// 3. Reconstruir com BFM + simetria
let prosthesis = ProsthesisDesigner::new()
    .set_patient_scan(patient_scan)
    .set_missing_region(missing_region)
    .use_symmetry()  // Espelhar lado saudável
    .match_skin_tone(patient_scan.avg_color())
    .design()?;

// 4. Otimizar para impressão 3D
let printable = prosthesis
    .add_support_structures()
    .check_overhang_angles()
    .slice_for_printer(PrinterProfile::Creality_Ender3)?;

// 5. Exportar STL
printable.export_stl("protese_nariz.stl")?;
printable.generate_instructions("manual_instalacao.pdf")?;
```

#### 3. Impressão + Pós-Processamento
```bash
# Material: PLA Flexível (skin tone)
# Impressora: Ender 3 Pro (R$ 1.200)
# Tempo: 4-8 horas
# Custo material: R$ 20-50

# Pós-processamento:
1. Remover suportes
2. Lixar (600, 1000, 2000 grit)
3. Pintar (airbrush) - match tom de pele
4. Selagem (verniz médico)
5. Fixação (adesivo médico ou óculos)
```

### Output
- Arquivo STL pronto para impressão
- Manual de instalação ilustrado
- Vídeo tutorial
- Suporte online (telessaúde)

## 🏗️ Arquitetura Técnica

```
┌─────────────────┐
│  Mobile App     │ Escanear com celular
│  (React Native) │ (ARKit/ARCore)
└────────┬────────┘
         │ Upload
┌────────▼────────┐
│  AVL Cloud API  │
│  Design Service │ face3d-rs + ML
└────────┬────────┘
         │
┌────────▼────────┐
│  AvilaDB        │ Perfis pacientes
│  + AVL Storage  │ (arquivos STL)
└────────┬────────┘
         │ Download STL
┌────────▼────────┐
│  Maker Lab      │ Impressão local
│  ou Casa        │ (descentralizado)
└─────────────────┘
```

## 📊 Stack Tecnológica

### Mobile (Scanning)
- **React Native**: App cross-platform
- **ARKit** (iOS): Escaneamento 3D preciso
- **ARCore** (Android): Alternativa Android
- **Three.js**: Preview 3D no app

### Backend
- **face3d-rs**: Design da prótese
- **OpenSCAD**: CAD paramétrico
- **Blender Python API**: Pós-processamento
- **Axum**: API REST

### Infraestrutura
- **AVL Cloud**: Compute + Storage
- **AvilaDB**: Perfis + histórico
- **CDN**: Distribuição STLs

### Impressão
- **Cura Engine**: Slicing automático
- **OctoPrint**: Gerenciamento impressoras

## 🚀 MVP - Funcionalidades

### Fase 1 (3 meses) - Protótipo
- [ ] App mobile (escanear com câmera)
- [ ] Design automático: nariz, orelha
- [ ] Exportar STL básico
- [ ] Testar com 10 pacientes

### Fase 2 (6 meses) - Produto
- [ ] IA de coloração (match tom de pele)
- [ ] Biblioteca de 20 próteses comuns
- [ ] Integração com maker labs
- [ ] Marketplace (designers voluntários)

### Fase 3 (12 meses) - Escala
- [ ] Parcerias hospitais SUS
- [ ] Rede de maker labs (100 cidades)
- [ ] Certificação ANVISA
- [ ] App de telemedicina integrado

## 💰 Modelo de Negócio

### Pricing (Modelo Social)

#### Pacientes
- **Gratuito**: Scan + Design + STL
- **Custo real**: Apenas material impressão (R$ 20-50)
- **Impressão**: Maker lab parceiro ou própria

#### Maker Labs (B2B)
- **Software gratuito**: Licença comunitária
- **Revenue share**: 10% se cobrar paciente
- **Materiais**: Marketplace AVL (comissão)

#### Hospitais (B2G)
- **Licença institucional**: R$ 10k/ano
- **Treinamento equipe**: Incluído
- **Suporte prioritário**: 24/7

### Custos vs Tradicional

| Item | Tradicional | Nossa Solução | Economia |
|------|-------------|---------------|----------|
| Prótese Nariz | R$ 8.000 | R$ 50 | **99%** |
| Prótese Orelha | R$ 12.000 | R$ 80 | **99%** |
| Prótese Olho | R$ 15.000 | R$ 100 | **99%** |
| Tempo | 3-6 meses | 1-2 semanas | **90%** |

## 🤝 Parcerias Estratégicas

### Saúde
1. **INCA** (Instituto Nacional do Câncer)
2. **Hospital de Câncer de Barretos**
3. **GRAACC** (Oncologia pediátrica)
4. **Rede hospitalar SUS**

### Maker Movement
1. **Fab Lab Livre SP** - Rede de labs
2. **Garoa Hacker Clube** - Comunidade makers
3. **Instituto de Tecnologia e Sociedade**
4. **Olabi** (RJ) - Makerspace social

### Materiais
1. **3D Fila** - Fornecedor PLA brasileiro
2. **GTMax3D** - Filamentos especiais
3. **Flashforge** - Impressoras acessíveis

### Academia
1. **USP** - Faculdade de Odontologia
2. **Unicamp** - Engenharia Biomédica
3. **ITA** - Impressão 3D

## 📈 Métricas de Impacto

### Objetivos Ano 1
- 🎭 **1.000 próteses** impressas
- 🏥 **10 hospitais** parceiros
- 🔧 **50 maker labs** ativos
- 💰 **R$ 8M economia** para pacientes

### KPIs
- Custo médio por prótese
- Tempo scan → prótese pronta
- Satisfação paciente (NPS)
- Taxa de reimpressão (ajustes)

## 🛡️ Regulamentação & Segurança

### ANVISA
- ✅ **Registro Classe II**: Dispositivo médico
- ✅ **Biocompatibilidade**: Materiais aprovados
- ✅ **Rastreabilidade**: Cada prótese tem ID único
- ✅ **Esterilização**: Protocolo definido

### Materiais Aprovados
1. **PLA Médico**: Biocompatível, biodegradável
2. **Silicone** (pós-processamento): Flexibilidade
3. **Resina Dental**: Próteses rígidas
4. **TPU**: Próteses flexíveis

### Segurança
- 🔒 Dados médicos criptografados (LGPD)
- 🩺 Aprovação médica obrigatória
- 📋 Termo de consentimento
- 🔬 Controle de qualidade (checklist)

## 📚 Tecnologia de Design

### Reconstrução por Simetria
```rust
impl ProsthesisDesigner {
    fn mirror_healthy_side(&self) -> Result<Mesh> {
        let scan = &self.patient_scan;
        
        // 1. Detectar plano de simetria facial
        let symmetry_plane = scan.compute_symmetry_plane()?;
        
        // 2. Isolar lado saudável
        let healthy_side = scan.extract_side(Side::Healthy)?;
        
        // 3. Espelhar
        let mirrored = healthy_side.mirror(symmetry_plane)?;
        
        // 4. Ajustar para região faltante
        let prosthesis = mirrored.crop_to_defect_region(&self.missing_region)?;
        
        // 5. Blend transição (suavizar bordas)
        prosthesis.blend_edges(blend_distance_mm: 5.0)?;
        
        Ok(prosthesis)
    }
}
```

### Coloração Automática
```rust
// Match tom de pele do paciente
fn color_matching(scan: &Scan3D, prosthesis: &Mesh) -> ColorMap {
    // Amostragem de cores da pele saudável
    let skin_samples = scan.sample_colors_near_defect(radius_mm: 20.0);
    
    // Calcular cor média (LAB color space)
    let avg_color = skin_samples.mean_lab();
    
    // Gerar gradiente natural
    ColorMap::generate_gradient(
        base_color: avg_color,
        variation: 0.05,  // 5% variação natural
    )
}
```

## 🎓 Equipe Necessária

### Tech (4 pessoas)
- 1 Tech Lead (3D/CAD)
- 1 Rust Developer (face3d-rs)
- 1 Mobile Developer (React Native)
- 1 3D Designer (Blender/OpenSCAD)

### Clínica (3 pessoas)
- 1 Médico (oncologia/cirurgia plástica)
- 1 Protesista (consultor)
- 1 Maker (impressão 3D)

### Custo: R$ 60k/mês

## 🗓️ Roadmap

### Q1 2026 - MVP
- App mobile (scan 3D)
- Design automático (nariz, orelha)
- Exportar STL
- Piloto: 10 pacientes (SP)

### Q2 2026 - Launch
- 20 tipos de próteses
- Coloração automática
- 5 maker labs parceiros
- 100 próteses impressas

### Q3 2026 - Growth
- Parcerias hospitais SUS
- Marketplace de designs
- 20 maker labs
- 500 próteses

### Q4 2026 - Scale
- Certificação ANVISA
- 50 maker labs (nacional)
- 2.000 próteses
- Revenue: R$ 200k/ano (B2G)

## 🌟 Casos de Uso Específicos

### 1. Prótese de Nariz
- **Causa**: Câncer de pele (mais comum)
- **Design**: Simetria facial
- **Material**: PLA + silicone
- **Fixação**: Adesivo médico ou óculos

### 2. Prótese de Orelha
- **Causa**: Microtia (congênito), acidentes
- **Design**: Espelhamento orelha saudável
- **Material**: TPU flexível
- **Fixação**: Óculos ou adesivo

### 3. Prótese Ocular (Cobertura)
- **Causa**: Enucleação (remoção olho)
- **Design**: Match com olho saudável
- **Material**: Resina pintada à mão
- **Fixação**: Óculos especiais

### 4. Prótese Maxilofacial (Parcial)
- **Causa**: Tumores, acidentes graves
- **Design**: CAD complexo
- **Material**: PLA + revestimento silicone
- **Fixação**: Parafusos ósseos (cirúrgico)

## 💬 Depoimentos (Simulados)

> *"Perdi meu nariz para câncer de pele. A prótese convencional custava R$ 10 mil e levaria 6 meses. Com esse projeto, em 2 semanas tive minha prótese por R$ 50. Voltei a sair de casa."*  
> — José, 58 anos, Agricultor (MG)

> *"Minha filha nasceu sem orelha direita (microtia). O SUS tinha fila de 3 anos. Fizemos a prótese em um maker lab local. Ela voltou a sorrir na escola."*  
> — Maria, mãe (BA)

## 📞 Como Participar

### Para Pacientes
- 📱 **App**: Download gratuito (iOS/Android)
- 🏥 **Hospitais**: Lista de parceiros
- 🔧 **Maker Labs**: Encontre o mais próximo

### Para Maker Labs
- 🤝 **Parceria**: makerlabs@avila.cloud
- 🆓 **Software gratuito**: Registro online
- 📚 **Treinamento**: Curso online

### Para Hospitais
- 🏥 **Implantação**: hospitais@avila.cloud
- 💰 **Gratuito SUS**: Projeto social

### Para Desenvolvedores
- 💻 **Open source**: GitHub
- 🎨 **Designs**: Contribua próteses

---

## 🚀 Call to Action

**Tecnologia 3D + Solidariedade = Vidas Transformadas**

Vamos democratizar acesso a próteses faciais no Brasil.

---

*Desenvolvido com empatia por Avila.inc*  
*"Um rosto, uma dignidade"*
