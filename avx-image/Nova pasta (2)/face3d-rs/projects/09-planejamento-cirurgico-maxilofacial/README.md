# 🧑‍⚕️ Projeto 9: Planejamento Cirúrgico Maxilofacial (SUS)

## 🎯 Objetivo
Sistema de simulação 3D para planejamento de cirurgias ortognáticas (correção mandíbula/maxilar), reduzindo tempo cirúrgico e melhorando resultados no SUS.

## 💡 O Problema

### Cirurgias Maxilofaciais no Brasil
- 🦷 **50k cirurgias/ano** (SUS + privado)
- ⏳ **Fila SUS**: 2-5 anos de espera
- 💰 **Custo privado**: R$ 20k-80k (inacessível)
- 🔧 **Taxa reoperação**: 15% (planejamento inadequado)

### Condições Tratadas
1. **Prognatismo**: Mandíbula projetada ("queixo para frente")
2. **Retrognatismo**: Mandíbula recuada
3. **Laterognatismo**: Assimetria facial
4. **Má oclusão severa**: Mordida errada
5. **Apneia do sono**: Vias aéreas obstruídas
6. **Trauma facial**: Acidentes graves

### Desafios Atuais
- 📏 Planejamento manual impreciso (cefalometria 2D)
- ⏱️ Cirurgias longas (6-8 horas)
- 🎯 Resultados imprevisíveis
- 💼 Limitação SUS (falta tecnologia 3D)

## 💡 Como Funciona

### Pipeline Cirúrgico

#### 1. Diagnóstico (Input)
```rust
use face3d_rs::maxillofacial::*;

// Dados do paciente
struct PatientData {
    ct_scan: DicomSeries,           // Tomografia mandíbula
    dental_cast: Mesh3D,            // Moldagem dentária
    photos: Vec<Image>,             // Fotos faciais (perfil + frontal)
    complaints: Vec<Complaint>,     // Queixas (estética, funcional)
}

impl PatientData {
    async fn load_and_process(patient_id: &str) -> Result<Self> {
        // Carregar DICOM (tomografia)
        let ct_scan = load_dicom_series(&format!("patient_{}/ct/", patient_id))?;
        
        // Segmentar ossos (mandíbula + maxila)
        let skull_model = segment_skull_from_ct(&ct_scan)?;
        
        // Carregar scan intraoral (dentes)
        let teeth_model = load_stl(&format!("patient_{}/teeth.stl", patient_id))?;
        
        // Fotos faciais
        let photos = load_photos(patient_id)?;
        
        Ok(PatientData { ct_scan, dental_cast: teeth_model, photos, ... })
    }
}
```

#### 2. Análise Cefalométrica 3D
```rust
fn cephalometric_analysis(skull: &SkullModel) -> CephalometricReport {
    // Landmarks anatômicos 3D
    let landmarks = skull.detect_landmarks()?;
    
    // Ângulos importantes
    let sna = landmarks.angle_sella_nasion_a_point();  // Posição maxila
    let snb = landmarks.angle_sella_nasion_b_point();  // Posição mandíbula
    let anb = sna - snb;  // Relação maxila-mandíbula
    
    // Classificação de Angle (oclusão)
    let occlusion_class = if anb > 4.0 {
        OcclusionClass::ClassII  // Retrognatismo
    } else if anb < 0.0 {
        OcclusionClass::ClassIII  // Prognatismo
    } else {
        OcclusionClass::ClassI  // Normal
    };
    
    // Plano oclusal
    let occlusal_plane = calculate_occlusal_plane(&skull)?;
    
    CephalometricReport {
        sna,
        snb,
        anb,
        occlusion_class,
        occlusal_plane,
        recommendations: generate_recommendations(anb, occlusion_class),
    }
}
```

#### 3. Planejamento Virtual (VSP)
```rust
async fn virtual_surgical_planning(
    patient: &PatientData,
    analysis: &CephalometricReport,
) -> SurgicalPlan {
    let mut planner = SurgicalPlanner::new(patient)?;
    
    // PASSO 1: Osteotomias (cortes ósseos)
    let osteotomies = match analysis.occlusion_class {
        OcclusionClass::ClassII => {
            // Avanço mandibular
            vec![
                Osteotomy::BSSO {  // Bilateral Sagittal Split Osteotomy
                    advancement_mm: 8.0,
                    rotation_degrees: 2.0,
                }
            ]
        }
        OcclusionClass::ClassIII => {
            // Avanço maxilar + recuo mandibular
            vec![
                Osteotomy::LeFortI {
                    advancement_mm: 5.0,
                    impaction_mm: 2.0,
                },
                Osteotomy::BSSO {
                    setback_mm: 4.0,
                    rotation_degrees: -1.0,
                }
            ]
        }
        _ => vec![],
    };
    
    // PASSO 2: Simular osteotomias
    let simulated_bones = planner.apply_osteotomies(&osteotomies)?;
    
    // PASSO 3: Reposicionar maxila/mandíbula
    let repositioned = planner.reposition_jaw_segments(
        simulated_bones,
        target_occlusion: OcclusionGoal::IdealOverjet { mm: 2.5 },
    )?;
    
    // PASSO 4: Prever tecidos moles (face)
    let predicted_face = predict_soft_tissue_changes(
        &patient.photos,
        &repositioned,
    )?;
    
    // PASSO 5: Gerar guias cirúrgicos 3D
    let surgical_guides = generate_cutting_guides(&osteotomies)?;
    
    SurgicalPlan {
        osteotomies,
        bone_movements: repositioned,
        predicted_face,
        surgical_guides,
        estimated_duration: Duration::from_hours(4),
        materials_needed: calculate_materials(&repositioned),
    }
}
```

#### 4. Fabricação de Guias (Impressão 3D)
```rust
fn generate_cutting_guides(osteotomies: &[Osteotomy]) -> Vec<SurgicalGuide> {
    osteotomies.iter().map(|ost| {
        match ost {
            Osteotomy::BSSO { advancement_mm, .. } => {
                // Guia para corte bilateral
                let guide = SurgicalGuide {
                    name: "BSSO Cutting Guide",
                    fit_region: FitRegion::MandibularRamus,
                    cutting_slots: vec![
                        CuttingSlot {
                            angle: 90.0,
                            depth: 30.0,
                            position: Vector3::new(20.0, -10.0, 5.0),
                        }
                    ],
                    positioning_pins: 3,
                };
                
                // Exportar STL para impressão
                guide.export_stl("bsso_guide_L.stl")?;
                guide
            }
            _ => { /* outros tipos */ }
        }
    }).collect()
}
```

#### 5. Pós-Operatório Virtual
```rust
fn postop_monitoring(
    preop: &PatientData,
    surgical_plan: &SurgicalPlan,
    postop_ct: &DicomSeries,
) -> OutcomeReport {
    // Comparar plano vs realidade
    let planned_position = &surgical_plan.bone_movements;
    let actual_position = segment_skull_from_ct(postop_ct)?;
    
    let deviation = calculate_deviation(planned_position, &actual_position)?;
    
    OutcomeReport {
        accuracy: if deviation.max_mm < 2.0 {
            Accuracy::Excellent
        } else if deviation.max_mm < 5.0 {
            Accuracy::Good
        } else {
            Accuracy::NeedsRevision
        },
        max_deviation_mm: deviation.max_mm,
        avg_deviation_mm: deviation.avg_mm,
        patient_satisfaction: None,  // Survey pós 6 meses
    }
}
```

### Output
- **Relatório cirúrgico completo** (PDF)
- **Modelos 3D antes/depois** (visualização)
- **Guias cirúrgicos** (STL para impressão)
- **Placas/parafusos** (especificação)
- **Vídeo explicativo** (para paciente)

## 🏗️ Arquitetura Técnica

```
┌──────────────────┐
│  Web Dashboard   │ React + Three.js
│  (Cirurgião)     │
└────────┬─────────┘
         │ HTTPS
┌────────▼─────────┐
│  AVL Gateway API │ Axum (Rust)
│                  │
└────────┬─────────┘
         │
┌────────▼─────────┐
│  face3d-rs       │ Skull modeling
│  + 3D Slicer     │ + Osteotomy sim
└────────┬─────────┘
         │
┌────────▼─────────┐
│  DICOM Server    │ Orthanc (PACS)
│  (Tomografias)   │
└────────┬─────────┘
         │
┌────────▼─────────┐
│  AvilaDB         │ Casos clínicos
│  (LGPD/HIPAA)    │ + Outcomes
└──────────────────┘
```

## 📊 Stack Tecnológica

### Core 3D
- **face3d-rs**: Modelagem facial + simulação
- **3D Slicer**: Segmentação CT (Python/C++)
- **VTK**: Visualização 3D
- **ITK**: Registro de imagens médicas

### Backend
- **Axum**: API REST
- **Orthanc**: PACS (DICOM storage)
- **AvilaDB**: Prontuários
- **PostgreSQL + PostGIS**: Dados espaciais

### Frontend
- **React**: Dashboard web
- **Three.js / VTK.js**: Renderização 3D
- **Tailwind CSS**: UI

### CAD/CAM
- **FreeCAD**: Geração de guias
- **Meshmixer**: Pós-processamento
- **Cura**: Slicing para impressão 3D

## 🚀 MVP - Funcionalidades

### Fase 1 (6 meses) - Protótipo
- [ ] Upload CT scan (DICOM)
- [ ] Segmentação automática ossos
- [ ] Análise cefalométrica 3D
- [ ] Simulação 1 tipo de osteotomia (BSSO)
- [ ] Teste: 10 casos reais

### Fase 2 (12 meses) - Produto
- [ ] 5 tipos de osteotomias
- [ ] Predição tecidos moles
- [ ] Geração guias cirúrgicos
- [ ] Dashboard interativo
- [ ] Parceria: 3 hospitais SUS

### Fase 3 (24 meses) - Escala
- [ ] Certificação ANVISA (Classe III)
- [ ] Integração PACS hospitalar
- [ ] IA para sugerir planos
- [ ] 50 hospitais SUS

## 💰 Modelo de Negócio

### Pricing B2G (SUS)

#### Hospitais Universitários
- **Licença anual**: R$ 100k/hospital
- **Ilimitados planejamentos**: Incluído
- **Treinamento**: 2 semanas on-site
- **Suporte**: 24/7

#### Clínicas Privadas
- **R$ 500/planejamento** (pay-per-use)
- **Ou R$ 10k/mês**: Ilimitado

### ROI Hospital
- **Economia/cirurgia**: R$ 5k (redução tempo cirúrgico)
- **100 cirurgias/ano**: R$ 500k economia
- **Custo software**: R$ 100k
- **ROI**: 400%

### Nossa Economia
- Compute (CT processing): R$ 50/caso
- Suporte: R$ 20k/mês por hospital
- **Margem**: 60%

## 🤝 Parcerias Estratégicas

### Hospitais Públicos
1. **HC-FMUSP** (SP) - Referência nacional
2. **Hospital de Clínicas (UFPR)** - Curitiba
3. **HUCAM (UFES)** - Vitória
4. **Hospital das Clínicas (UFMG)** - BH
5. **Rede Sarah** - Reabilitação

### Academia
1. **USP** - Faculdade de Odontologia
2. **Unicamp** - Cirurgia Bucomaxilofacial
3. **UFRJ** - Pesquisa científica

### Indústria
1. **3D Systems** - Impressoras médicas
2. **Materialise** - Software CAD médico (parceria)
3. **Stratasys** - Bioimpressão

### Governo
1. **Ministério da Saúde** - Programa Nacional
2. **ANVISA** - Certificação
3. **CNPq** - Financiamento pesquisa

## 📈 Métricas de Impacto

### Objetivos Ano 1
- 🏥 **5 hospitais** implantados
- 🔧 **100 cirurgias** planejadas
- ⏱️ **30% redução** tempo cirúrgico
- 📊 **10% redução** reoperações

### KPIs Clínicos
- Precisão planejamento (desvio mm)
- Tempo médio planejamento
- Taxa de reoperação (%)
- Satisfação paciente (OQLQ score)
- Satisfação cirurgião (NPS)

## 🛡️ Regulamentação

### ANVISA
- ✅ **Classe III**: Dispositivo médico crítico
- ✅ **GMP**: Boas práticas de fabricação
- ✅ **Rastreabilidade**: Cada guia tem QR code
- ✅ **Estudos clínicos**: Mínimo 30 casos

### Dados Médicos
- 🔒 **LGPD**: Dados sensíveis protegidos
- 🏥 **HIPAA**: Se exportar EUA
- 📋 **Auditoria**: Logs imutáveis
- 🔐 **Criptografia**: At-rest + in-transit

## 📚 Base Científica

### Precisão VSP (Literatura)
- **Planejamento manual**: Desvio médio 3-5mm
- **VSP (Virtual Surgical Planning)**: Desvio médio < 2mm
- **Com guias 3D**: Desvio médio < 1mm ✅

### Papers de Referência
1. **"Accuracy of Virtual Surgical Planning in Orthognathic Surgery"** - J Oral Maxillofac Surg (2023)
2. **"3D-Printed Surgical Guides for Mandibular Osteotomies"** - Int J CARS (2024)
3. **"Soft Tissue Prediction in Orthognathic Surgery"** - JOMS (2022)

## 🎓 Equipe Necessária

### Tech (7 pessoas)
- 1 Tech Lead (Medical Imaging)
- 2 Rust Developers (face3d-rs)
- 1 Full-stack (Dashboard)
- 1 3D Engineer (CAD/CAM)
- 1 ML Engineer (Soft tissue prediction)
- 1 DevOps

### Clínica (3 pessoas)
- 1 Cirurgião Bucomaxilofacial (consultor)
- 1 Ortodontista (consultor)
- 1 Regulatório (ANVISA)

**Custo**: R$ 120k/mês

## 🗓️ Roadmap

### Q1-Q2 2026 - R&D
- Revisão literatura
- Protótipo funcional
- Validação 10 casos retrospectivos
- Submissão protocolo ANVISA

### Q3-Q4 2026 - Piloto
- Certificação ANVISA iniciada
- Implantação 2 hospitais piloto
- 30 casos prospectivos
- Publicação científica

### 2027 - Expansão
- Certificação ANVISA aprovada
- 10 hospitais SUS
- Edital Ministério da Saúde
- 500 cirurgias/ano

### 2028 - Nacional
- 50 hospitais
- Padrão SUS
- 2.000 cirurgias/ano
- Revenue: R$ 5M/ano

## 🌟 Casos de Uso

### Caso 1: Prognatismo Severo
- **Paciente**: 25 anos, classe III severa
- **Queixa**: Estética + má oclusão
- **Plano VSP**: Avanço maxilar 6mm + recuo mandibular 8mm
- **Resultado**: Oclusão ideal + perfil harmonioso

### Caso 2: Apneia do Sono
- **Paciente**: 45 anos, AOS severa (IAH 45)
- **Queixa**: Ronco + cansaço diurno
- **Plano VSP**: Avanço maxilomandibular 10mm
- **Resultado**: IAH 8 (cura)

### Caso 3: Trauma Facial
- **Paciente**: 30 anos, acidente moto
- **Queixa**: Assimetria pós-trauma
- **Plano VSP**: Reconstrução mandíbula + enxerto ósseo
- **Resultado**: Simetria restaurada

## 💬 Depoimentos (Simulados)

> *"Antes do VSP, eu levava 2 semanas para planejar uma cirurgia complexa. Agora levo 2 horas. E a precisão é muito maior."*  
> — Dr. João, Cirurgião Bucomaxilofacial (HC-USP)

> *"Fui operada pelo SUS com essa tecnologia. O médico me mostrou como eu ficaria ANTES da cirurgia. Fiquei exatamente como ele previu!"*  
> — Maria, 28 anos, Paciente

## 📞 Como Participar

### Para Hospitais
- 🏥 **Demo gratuita**: Teste com 5 casos
- 📧 **Contato**: hospitais@avila.cloud
- 📄 **Proposta técnica**: Disponível

### Para Cirurgiões
- 🎓 **Treinamento**: Curso online + presencial
- 🩺 **Beta tester**: Programa piloto

### Para Pesquisadores
- 🔬 **Dados**: Banco de casos (anonimizado)
- 📚 **Publicação**: Parceria acadêmica

---

## 🚀 Call to Action

**Cirurgia precisa. Resultados previsíveis. Sorrisos transformados.**

Vamos levar tecnologia 3D para o SUS.

---

*Desenvolvido com precisão por Avila.inc* 🦷  
*"Transformando sorrisos através da ciência"*
