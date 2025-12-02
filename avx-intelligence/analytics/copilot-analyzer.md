# Copilot Analyzer - Semantic Kernel

## Objetivo
Analisar comportamento dos copilots como se fossem desenvolvedores humanos.

## Arquitetura

```
Logs Copilot → Parser → Embeddings → Analytics → Dashboard
                  ↓         ↓            ↓
              Metadata   Vectors    Scores/Alerts
```

## Métricas Coletadas

### 1. Instruction Adherence (IA)
- **Fórmula**: `cosine(embed(prompt), embed(response))`
- **Target**: > 0.85
- **Alerta**: < 0.60

### 2. Code Correctness (CC)
- **Fórmula**: `tests_passed / tests_total`
- **Target**: > 0.90
- **Alerta**: < 0.70

### 3. Novelty (NV)
- **Fórmula**: `1 - max(similarity with past responses)`
- **Target**: 0.50 - 0.75
- **Alerta**: > 0.90 (possível hallucination)

### 4. Consistency (CONS)
- **Fórmula**: `1 - std(pairwise_similarities)`
- **Target**: > 0.80
- **Alerta**: < 0.60

### 5. Quality Score (Q)
- **Fórmula**: Weighted combination
  - IA: 30%
  - CC: 25%
  - Relevance: 15%
  - Instruction-follow: 10%
  - Brevity: 5%
  - Latency: 5%
  - Novelty: 10%

## Pipeline de Análise

### Step 1: Ingestão
```bash
python avx-inspector/ingest.py \
  --source logs/copilots/ \
  --output events/raw.jsonl
```

### Step 2: Embedding
```bash
python avx-inspector/embed.py \
  --input events/raw.jsonl \
  --model all-mpnet-base-v2 \
  --output vectors/faiss.index
```

### Step 3: Scoring
```bash
python avx-inspector/score.py \
  --vectors vectors/faiss.index \
  --output scores/agents.json
```

### Step 4: Visualização
```bash
python avx-inspector/dashboard.py \
  --scores scores/agents.json \
  --port 8080
```

## Semantic Kernel Integration

### C# Setup
```csharp
var kernel = Kernel.CreateBuilder()
    .AddOpenAITextEmbedding("text-embedding-3")
    .Build();

// Gerar embedding
var embedding = await kernel.Embeddings
    .CreateEmbeddingAsync(text);

// Análise de aderência
var reason = await kernel.RunAsync(
    "Analyze instruction adherence",
    variables
);
```

## Alertas Automáticos

| Condição | Threshold | Ação |
|----------|-----------|------|
| Q < 0.50 | Crítico | Revisar copilot config |
| CC < 0.70 | Alto | Verificar código gerado |
| NV > 0.85 | Médio | Risco de hallucination |
| CONS < 0.60 | Alto | Copilot instável |

## Output Esperado

```json
{
  "agent_id": "copilot-machine-3-vscode-1",
  "period": "2025-12-02",
  "metrics": {
    "quality_score": 0.87,
    "adherence": 0.92,
    "correctness": 0.95,
    "novelty": 0.65,
    "consistency": 0.89
  },
  "status": "green",
  "alerts": []
}
```

## Roadmap

- [ ] Setup Semantic Kernel pipeline
- [ ] Integrar com log auto-capture
- [ ] Dashboard Grafana
- [ ] Alertas Slack/Email
- [ ] ML para detectar padrões
- [ ] Recomendações automáticas de melhoria
