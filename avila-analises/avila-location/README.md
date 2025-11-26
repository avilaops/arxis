# Avila Location Intelligence

🌍 Sistema completo de inteligência geoespacial e análise de mercado para seleção de localização ideal para empresas de tecnologia.

## 🎯 Visão Geral

Sistema desenvolvido em **Rust 100%** para análise quantitativa e qualitativa de localizações para abertura de empresas de TI, com foco especial em **Portugal** e **Dubai/UAE**.

## 🚀 Funcionalidades

### 📊 Análise de Localização Geográfica
- **Weber Problem**: Minimiza distância total ponderada até clientes
- **P-Median Problem**: Encontra P localizações ótimas
- **Maximal Coverage**: Maximiza cobertura dentro de raio de serviço
- **Gravity Model**: Calcula atratividade baseada em tamanho e distância
- **Voronoi Diagrams**: Define áreas de influência
- **TSP Solver**: Otimiza rotas de visita

### 🏢 Análise de Mercado
- **K-Means & DBSCAN Clustering**: Segmentação de mercado
- **RFM Analysis**: Análise de Recência, Frequência e Valor
- **Lead Scoring**: Pontuação de leads potenciais
- **CLV Calculation**: Customer Lifetime Value

### 💰 Análise Financeira
- **NPV & IRR**: Valor Presente Líquido e Taxa Interna de Retorno
- **Break-Even Analysis**: Ponto de equilíbrio
- **Monte Carlo Simulation**: Simulação de múltiplos cenários
- **Tax Optimization**: Otimização fiscal multi-jurisdição
- **Sensitivity Analysis**: Análise de sensibilidade

### 🎯 Scoring Multi-Critério
- **AHP** (Analytic Hierarchy Process): Ponderação hierárquica
- **TOPSIS**: Similaridade com solução ideal
- **ELECTRE**: Eliminação por concordância/discordância
- **MAUT**: Teoria de utilidade multi-atributo

## 🗺️ Regiões Cobertas

### 🇵🇹 Portugal (18 regiões)
- **Metropolitanas**: Lisboa, Porto
- **Urbanas**: Braga, Coimbra, Aveiro, Faro, Funchal, Setúbal, Leiria, Viana do Castelo
- **Interior** (incentivos fiscais 50%): Évora, Viseu, Guarda, Castelo Branco, Portalegre, Beja, Santarém, Ponta Delgada

### 🇦🇪 UAE (12 regiões)
- **Metropolitanas**: Dubai, Abu Dhabi
- **Urbanas**: Sharjah, Ajman, Ras Al Khaimah, Fujairah, Umm Al Quwain
- **Free Zones** (0% impostos):
  - Dubai Internet City (especializada em TI)
  - Dubai Silicon Oasis
  - Dubai Media City
  - JAFZA (Jebel Ali)
  - ADGM (Abu Dhabi Global Market)

## 📦 Instalação

```bash
cargo build --release
```

## 🎮 Uso

### Analisar todas as regiões
```bash
cargo run -- analyze
cargo run -- analyze --scenario bootstrap  # Foco em baixo custo
cargo run -- analyze --scenario growth     # Foco em crescimento
cargo run -- analyze --scenario remote     # Foco em trabalho remoto
```

### Comparar duas regiões
```bash
cargo run -- compare "Lisboa" "Dubai"
cargo run -- compare "Porto" "Dubai Internet City"
cargo run -- compare "Braga" "Sharjah"
```

### Listar regiões disponíveis
```bash
cargo run -- list
cargo run -- list --country Portugal
cargo run -- list --country UAE
```

### Ver detalhes de uma região
```bash
cargo run -- detail "Porto"
cargo run -- detail "Dubai Internet City"
cargo run -- detail "Braga"
```

### Gerar relatório completo
```bash
cargo run -- report --output analysis.json
```

### Comparar ROI
```bash
cargo run -- roi --investment 50000 --revenue 120000 --years 5
```

### Filtrar regiões por critério
```bash
cargo run -- filter --max-cost 60 --min-market 70
cargo run -- filter --max-competition 30 --min-infrastructure 80
```

## 📊 Exemplo de Análise

```
🌍 Analyzing Locations...

┌──────┬─────────────────────────┬────────┬──────┬────────┬─────────────┬────────────────┐
│ Rank │ Location                │ Score  │ Cost │ Market │ Competition │ Infrastructure │
├──────┼─────────────────────────┼────────┼──────┼────────┼─────────────┼────────────────┤
│ 🥇 1 │ Dubai Internet City     │  89.2  │ 54.0 │  98.0  │    68.0     │     100.0      │
│ 🥈 2 │ Porto                   │  84.5  │ 83.5 │  76.0  │    72.0     │      88.0      │
│ 🥉 3 │ Braga                   │  82.1  │ 88.0 │  72.0  │    82.0     │      75.0      │
│  4   │ Dubai Silicon Oasis     │  87.8  │ 58.0 │  98.0  │    65.0     │     100.0      │
│  5   │ Lisboa                  │  78.9  │ 62.5 │  85.0  │    65.0     │      95.0      │
└──────┴─────────────────────────┴────────┴──────┴────────┴─────────────┴────────────────┘

📊 Analysis Summary
  • Best Overall: Dubai Internet City
  • Best for Bootstrap: Braga
  • Best for Growth: Lisboa
  • Best for Remote: Dubai Internet City
```

## 🔍 Critérios de Avaliação

### Pesos Padrão
- **Custo de Vida**: 20%
- **Demanda de Mercado**: 25%
- **Concorrência**: 15%
- **Infraestrutura**: 15%
- **Qualidade de Vida**: 10%
- **Incentivos Fiscais**: 5%
- **Acessibilidade**: 5%
- **Disponibilidade de Talento**: 5%

### Cenários Pré-configurados

#### Bootstrap (baixo capital)
- Custo de Vida: 35%
- Demanda de Mercado: 25%
- Outros critérios menores

#### Growth (crescimento)
- Demanda de Mercado: 35%
- Infraestrutura: 15%
- Acessibilidade: 10%

#### Remote-First
- Infraestrutura: 25%
- Qualidade de Vida: 15%
- Custo de Vida: 25%

## 💡 Insights Chave

### 🇵🇹 Portugal
- **Lisboa**: Maior mercado, alta concorrência, custos elevados
- **Porto**: Excelente custo-benefício, crescimento acelerado (12.5%/ano)
- **Braga**: Melhor valor, jovem e universitária, baixa concorrência
- **Interior**: Incentivos fiscais de 50% no IRC, custo 70% menor que Lisboa

### 🇦🇪 Dubai/UAE
- **Dubai Internet City**: 0% impostos, infraestrutura perfeita, salários 2x maiores
- **Free Zones**: 0% corporate tax, 0% VAT, 100% propriedade estrangeira
- **ADGM**: Distrito financeiro premium para FinTech
- **Sharjah**: Alternativa mais acessível próxima a Dubai

## 📈 Comparação Fiscal

| Jurisdição | IRC/Corporate Tax | IVA/VAT | Social Security | Incentivos |
|------------|-------------------|---------|-----------------|------------|
| **Portugal** | 21% | 23% | 23.75% | NHR, Interior 50% |
| **Portugal Interior** | 10.5% (50% redução) | 23% | 23.75% | 10 anos |
| **Dubai** | 9% (>375k AED lucro) | 5% | 0% | Small business 0% |
| **Dubai Free Zone** | 0% | 0% | 0% | Indefinido |

## 🏗️ Arquitetura

```
avila-location/
├── models/          # Estruturas de dados core
├── algorithms/      # Algoritmos de otimização
│   ├── geographic   # Weber, P-Median, MCLP, Voronoi
│   ├── clustering   # K-Means, DBSCAN, RFM, Lead Scoring
│   ├── financial    # Monte Carlo, Tax Optimization
│   └── routing      # TSP, Isochrone, Time-Distance Matrix
├── scoring/         # Sistemas de decisão multi-critério
│   ├── ahp          # Analytic Hierarchy Process
│   ├── topsis       # TOPSIS
│   ├── electre      # ELECTRE III
│   └── maut         # Multi-Attribute Utility Theory
├── data/            # Dados pré-populados
│   ├── portugal     # 18 regiões portuguesas
│   └── uae          # 12 regiões UAE + Free Zones
├── analysis/        # Análise comparativa e relatórios
└── visualization/   # Tabelas e visualizações
```

## 🔬 Tecnologias

- **Rust** 100%
- **nalgebra**: Álgebra linear
- **rayon**: Processamento paralelo
- **serde**: Serialização
- **geo**: Operações geoespaciais
- **statrs**: Estatísticas
- **clap**: CLI
- **comfy-table**: Tabelas bonitas

## 🎯 Casos de Uso

### Para Empresário Brasileiro em Portugal
```bash
# Análise focada em baixo custo inicial
cargo run -- analyze --scenario bootstrap --limit 10

# Comparar Porto vs Braga
cargo run -- compare "Porto" "Braga"

# Ver detalhes de Braga
cargo run -- detail "Braga"
```

### Para Empresa Tech em Expansão
```bash
# Foco em crescimento de mercado
cargo run -- analyze --scenario growth

# Comparar Lisboa vs Dubai
cargo run -- compare "Lisboa" "Dubai"

# ROI comparison
cargo run -- roi --investment 100000 --revenue 250000 --years 5
```

### Para Remote-First Company
```bash
# Foco em infraestrutura e qualidade de vida
cargo run -- analyze --scenario remote

# Filtrar por ótima infraestrutura e baixo custo
cargo run -- filter --min-infrastructure 80 --max-cost 60
```

## 📝 Exportação de Dados

```bash
# JSON
cargo run -- analyze --format json > results.json

# CSV
cargo run -- analyze --format csv > results.csv

# Relatório completo
cargo run -- report --output comprehensive_analysis.json
```

## 🤝 Contribuições

Este é um projeto da **AVL Platform** para análise de localizações. Para contribuir:

1. Fork o repositório
2. Crie uma branch (`git checkout -b feature/nova-regiao`)
3. Commit suas mudanças (`git commit -am 'Adiciona região X'`)
4. Push para a branch (`git push origin feature/nova-regiao`)
5. Abra um Pull Request

## 📄 Licença

MIT OR Apache-2.0

## 🌟 Próximos Passos

- [ ] Adicionar mais regiões (UK, Alemanha, EUA)
- [ ] Integração com APIs de dados em tempo real
- [ ] Dashboard web interativo
- [ ] Análise de custos de salários por região
- [ ] Previsão de crescimento com ML
- [ ] Exportação para PDF com gráficos

---

**AVL Platform** - Construindo ferramentas para decisões inteligentes 🚀

Para mais informações: https://avila.cloud
