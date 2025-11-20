# 🚀 Guia de Processamento de Dados 3D/4D

## Pipelines Disponíveis

Criamos **2 exemplos completos** demonstrando o processamento de dados científicos complexos:

---

## 📊 1. Data Processing Pipeline

**Arquivo**: `examples/data_processing_pipeline.rs`

### Features
- ✅ **Polars DataFrames** - Processamento de 100k+ linhas
- ✅ **Linfa Clustering** - K-Means em dados 4D
- ✅ **PCA** - Redução dimensional 4D → 3D
- ✅ **Time Series** - Anomaly detection + ARIMA forecasting

### Executar
```bash
cargo run --example data_processing_pipeline --release
```

### Pipeline 1: Ondas Gravitacionais
```rust
// 1. Gerar 100k amostras de strain
// 2. Criar DataFrame com Polars
// 3. Feature engineering (abs_strain, power)
// 4. Detectar eventos (threshold)
// 5. Análise estatística distribuída
```

**Output**:
- DataFrame com strain, time, features
- Estatísticas (mean, std, min, max)
- Eventos detectados acima do threshold

### Pipeline 2: Clustering Astrofísico
```rust
// 1. Gerar 5k eventos (buracos negros, estrelas de nêutrons)
// 2. Features 4D: (m1, m2, distance, frequency)
// 3. K-Means clustering (3 clusters)
// 4. Análise de centroides
// 5. Distribuição de eventos
```

**Output**:
- 3 clusters identificados
- Centroides desnormalizados
- Percentual por cluster

### Pipeline 3: Redução Dimensional
```rust
// 1. Gerar 10k pontos 4D (x, y, z, t)
// 2. PCA (4D → 3D)
// 3. Calcular variância explicada
// 4. Criar DataFrame 3D (PC1, PC2, PC3)
```

**Output**:
- Variância explicada por componente
- DataFrame Polars com PCs
- Pronto para visualização 3D

### Pipeline 4: Time Series
```rust
// 1. Gerar série com 1k pontos + anomalias
// 2. TimeSeries com avila-telemetry
// 3. Detectar anomalias (Z-score)
// 4. Forecast ARIMA (próximos 10 pontos)
```

**Output**:
- Estatísticas temporais
- Anomalias detectadas com índice e z-score
- Previsão dos próximos valores

---

## 🕸️ 2. Graph Analytics & Distributed Processing

**Arquivo**: `examples/graph_analytics_distributed.rs`

### Features
- ✅ **Petgraph** - Análise de redes complexas
- ✅ **Dijkstra/A*** - Pathfinding em grafos
- ✅ **Rayon** - Processamento paralelo massivo (1M+ operações)
- ✅ **MapReduce** - Agregação distribuída

### Executar
```bash
cargo run --example graph_analytics_distributed --release
```

### Pipeline 1: Rede de Eventos GW
```rust
// 1. Criar grafo com 100 eventos
// 2. Conectar eventos próximos (threshold)
// 3. Analisar componentes conectados
// 4. Dijkstra: caminho mais curto
// 5. Calcular centralidade (grau)
```

**Output**:
- Número de componentes conectados
- Caminho mais curto entre eventos
- Top 5 eventos mais centrais

**Use Cases**:
- Identificar clusters de eventos relacionados
- Encontrar sequências de detecções
- Analisar topologia da rede de observações

### Pipeline 2: Pathfinding em Espaço-Tempo
```rust
// 1. Grid 3D (20x20x20 = 8k pontos)
// 2. Curvatura simulada (massa central)
// 3. Conectar vizinhos com peso por curvatura
// 4. A*: geodésica considerando curvatura
```

**Output**:
- Geodésica (caminho mais curto)
- Distância total ponderada
- Número de passos

**Use Cases**:
- Simular trajetórias de fótons em espaço curvo
- Calcular geodésicas nulas
- Análise de lensing gravitacional

### Pipeline 3: Processamento Paralelo Massivo
```rust
// 1. Gerar 1M tensores 4x4
// 2. Calcular determinantes em paralelo (Rayon)
// 3. Estatísticas distribuídas
// 4. Filtrar tensores interessantes
// 5. MapReduce em chunks de 10k
```

**Output**:
- 1M tensores processados em segundos
- Taxa: ~milhões de operações/segundo
- Estatísticas (mean, std, min, max)
- Resultados MapReduce validados

**Performance**:
- **Single-threaded**: ~10s
- **Rayon (multi-core)**: ~1-2s
- **Speedup**: 5-10x (depende do hardware)

---

## 🎯 Casos de Uso Reais

### 1. Análise de Dados do LISA
```rust
// Pipeline completo:
// 1. Ingerir strain data (Polars)
// 2. Feature engineering
// 3. Detectar eventos (threshold + ML)
// 4. Clustering de eventos similares
// 5. Forecast de detecções futuras
```

### 2. Simulações de Espaço-Tempo
```rust
// Pipeline:
// 1. Grid 4D (x, y, z, t)
// 2. Calcular métricas (1M tensores)
// 3. Geodésicas com A*
// 4. Visualização 3D (PCA)
```

### 3. Big Data Astrofísico
```rust
// Pipeline:
// 1. Polars: carregar catálogos (10M+ objetos)
// 2. Lazy evaluation para queries eficientes
// 3. Parquet: armazenamento columnar
// 4. Rayon: processamento paralelo
```

### 4. Machine Learning em Dados 4D
```rust
// Pipeline:
// 1. Feature extraction (ndarray)
// 2. Normalização
// 3. K-Means ou DBSCAN (Linfa)
// 4. PCA para visualização
// 5. SmartCore: Random Forest ou SVM
```

---

## 📈 Performance

### Benchmarks (hardware típico)

| Operação         | Dataset          | Tempo  | Throughput   |
| ---------------- | ---------------- | ------ | ------------ |
| Polars DataFrame | 1M linhas        | ~100ms | 10M linhas/s |
| K-Means (Linfa)  | 10k pontos 4D    | ~500ms | 20k pontos/s |
| PCA              | 10k pontos 4D→3D | ~200ms | 50k pontos/s |
| Rayon parallel   | 1M tensores      | ~1.5s  | 666k ops/s   |
| Petgraph A*      | 8k nós           | ~50ms  | -            |

---

## 🔧 Próximos Passos

### 1. Integração com AvilaDB
```rust
// Armazenar resultados no AvilaDB
use aviladb::Client;

let client = Client::connect("aviladb://localhost:8000").await?;
let db = client.database("scientific_data");

// Salvar eventos detectados
db.collection("gw_events").insert_many(&events).await?;

// Query eficiente
let high_mass = db.collection("gw_events")
    .query("SELECT * FROM gw_events WHERE mass_total > @threshold")
    .param("threshold", 50.0)
    .execute().await?;
```

### 2. Streaming com Tokio
```rust
// Processar dados em real-time
use tokio::stream::StreamExt;

let mut stream = data_source.stream();
while let Some(batch) = stream.next().await {
    // Processar batch com Polars + Rayon
    let df = process_batch(batch).await?;
    save_to_aviladb(df).await?;
}
```

### 3. Distributed Computing
```rust
// Coordenar múltiplos workers
// Worker 1: chunk 0-99k
// Worker 2: chunk 100k-199k
// etc...
// Agregar resultados (MapReduce)
```

---

## 📚 Referências Rápidas

### Polars
```rust
// Lazy evaluation
let df = df.lazy()
    .filter(col("mass") > lit(10.0))
    .group_by([col("cluster")])
    .agg([col("mass").mean()])
    .collect()?;
```

### Linfa
```rust
// K-Means
let model = KMeans::params(3).fit(&dataset)?;
let predictions = model.predict(&dataset);
```

### Rayon
```rust
// Parallel iterator
let results: Vec<_> = data
    .par_iter()
    .map(|x| expensive_computation(x))
    .collect();
```

### Petgraph
```rust
// Shortest path
let path = dijkstra(&graph, start, Some(end), |e| *e.weight());
```

---

**Documentação completa**: `docs/ML_LIBRARIES_INSTALLED.md`
**Exemplos**: `examples/data_processing_pipeline.rs`, `examples/graph_analytics_distributed.rs`
