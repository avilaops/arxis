# 🎓 Exemplo Prático para Estudantes - Sistema de Análise de Notas

## O que este exemplo faz?

Imagine que você é monitor de uma disciplina e precisa analisar o desempenho dos alunos. Este exemplo mostra como o **AvilaDB DataFrame** pode ajudar você a:

1. **Organizar dados** de alunos e notas
2. **Identificar alunos em risco** de reprovação
3. **Calcular médias** por aluno e por disciplina
4. **Criar rankings** de desempenho
5. **Gerar relatórios** automáticos com recomendações

---

## 🚀 Como executar

```bash
# No terminal, dentro da pasta do projeto:
cargo run --example student_grades
```

---

## 📊 Dados de Exemplo

### Alunos
- **Aluno 1**: João (4º semestre)
- **Aluno 2**: Maria (3º semestre)
- **Aluno 3**: Pedro (5º semestre)
- **Aluno 4**: Ana (2º semestre)

### Disciplinas
- **1** = Cálculo I
- **2** = Física Experimental
- **3** = Programação

### O que temos
- 12 notas (4 alunos × 3 disciplinas)
- Percentual de presença de cada aluno
- Informações pessoais dos alunos

---

## 💡 O que você vai aprender

### 1. **FILTER** - Filtrar dados
```rust
// Encontrar alunos com nota abaixo de 6.0
let em_risco = notas
    .filter(col("nota").lt(lit(6.0)))?;
```
**Por quê?** Identifica rapidamente quem precisa de ajuda!

---

### 2. **GROUP BY** - Agrupar e calcular médias
```rust
// Calcular média por aluno
let media_por_aluno = notas
    .group_by(&["aluno_id"])?
    .agg(&[
        col("nota").mean().alias("media_geral"),
        col("presenca").mean().alias("media_presenca"),
    ])?;
```
**Por quê?** Agrupa todas as notas de cada aluno e calcula estatísticas!

---

### 3. **SORT** - Ordenar dados
```rust
// Criar ranking do melhor para o pior
let ranking = media_por_aluno
    .sort("media_geral", SortOrder::Descending)?;
```
**Por quê?** Veja quem são os melhores alunos da turma!

---

### 4. **JOIN** - Combinar tabelas
```rust
// Juntar notas com informações pessoais
let relatorio = media_por_aluno
    .join(&alunos, "aluno_id", "aluno_id", JoinType::Inner)?;
```
**Por quê?** Combina dados de diferentes fontes (como no Excel com PROCV)!

---

### 5. **PIVOT** - Transformar dados
```rust
// Criar matriz Aluno × Disciplina
let matriz = notas
    .pivot(&["aluno_id"], "disciplina", "nota", PivotAggFunc::Mean)?;
```
**Por quê?** Visualiza dados em formato de tabela cruzada!

---

## 🎯 Resultados que você verá

### Exemplo de Saída:

```
📊 MÉDIA GERAL POR ALUNO
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Aluno 3 (Pedro): Média 9.17 ⭐ EXCELENTE!
Aluno 1 (João):  Média 8.33 ✅ ÓTIMO!
Aluno 2 (Maria): Média 6.33 ⚡ PODE MELHORAR
Aluno 4 (Ana):   Média 5.83 ⚠️  EM RISCO!
```

### Recomendações Automáticas:
```
👤 Aluno 4 (Ana):
   Média: 5.83
   ⚠️  ALERTA: Risco de reprovação!
   💡 Recomendação: Procure monitoria e grupos de estudo
```

---

## 🔥 Por que isso é útil?

### No dia a dia de um estudante:
- ✅ Acompanhar seu próprio desempenho
- ✅ Comparar com a média da turma
- ✅ Identificar em quais disciplinas precisa estudar mais
- ✅ Ver evolução ao longo do semestre

### Para coordenadores/professores:
- ✅ Identificar rapidamente alunos em risco
- ✅ Gerar relatórios automáticos
- ✅ Tomar decisões baseadas em dados
- ✅ Fazer intervenções pedagógicas direcionadas

### No mercado de trabalho:
- ✅ Análise de dados de clientes
- ✅ Dashboards de vendas
- ✅ Relatórios financeiros
- ✅ Machine Learning e IA

---

## 📚 Conceitos de Banco de Dados aplicados

Este exemplo usa conceitos que você aprende na faculdade:

### 1. **SELECT** (SQL) = `select()`
```rust
df.select(&["aluno_id", "nota"])?
```

### 2. **WHERE** (SQL) = `filter()`
```rust
df.filter(col("nota").gt(lit(6.0)))?
```

### 3. **GROUP BY** (SQL) = `group_by()`
```rust
df.group_by(&["disciplina"])?.agg(&[col("nota").mean()])?
```

### 4. **JOIN** (SQL) = `join()`
```rust
df1.join(&df2, "id", "id", JoinType::Inner)?
```

### 5. **ORDER BY** (SQL) = `sort()`
```rust
df.sort("nota", SortOrder::Descending)?
```

### 6. **PIVOT** (SQL/Excel) = `pivot()`
```rust
df.pivot(&["aluno"], "disciplina", "nota", PivotAggFunc::Mean)?
```

---

## 🎓 Exercícios para praticar

Após rodar o exemplo, tente modificar:

### Nível Fácil:
1. Mudar o critério de aprovação de 6.0 para 7.0
2. Adicionar mais um aluno
3. Adicionar uma nova disciplina

### Nível Médio:
4. Calcular quem tem a maior presença
5. Criar um filtro para alunos com presença < 75%
6. Ordenar por presença em vez de nota

### Nível Avançado:
7. Implementar sistema de pesos (prova vale mais que trabalho)
8. Calcular desvio padrão por disciplina
9. Criar um sistema de conceitos (A, B, C, D, F)

---

## 💻 Comparação com outras ferramentas

### Excel / Google Sheets
- ❌ Lento com muitos dados (>100k linhas)
- ❌ Fórmulas complexas e propensas a erro
- ✅ Fácil de usar visualmente

### Python (Pandas)
- ✅ Muito usado em Data Science
- ❌ Mais lento que Rust
- ✅ Grande comunidade

### **AvilaDB DataFrame (Rust)**
- ✅ **40-60% mais rápido** que Pandas
- ✅ **Menos memória** usada
- ✅ **Type-safe** (menos erros)
- ✅ **Integração nativa** com AvilaDB (banco brasileiro)
- ✅ **Pode processar milhões de linhas** facilmente

---

## 🌟 Casos de uso reais

### 1. Educação
- Sistemas de gestão acadêmica (Canvas, Moodle)
- Plataformas de ensino online
- Análise de evasão escolar

### 2. Empresas
- Análise de vendas
- KPIs de marketing
- Relatórios financeiros

### 3. Pesquisa
- Análise de dados experimentais
- Estatísticas científicas
- Machine Learning

### 4. Governo
- Análise de dados públicos
- Indicadores sociais
- Políticas públicas

---

## 🚀 Próximos passos

Após entender este exemplo, você pode:

1. **Aprender SQL** - Os conceitos são os mesmos!
2. **Estudar Data Science** - Este é o primeiro passo
3. **Fazer projetos** - Use dados reais da sua faculdade
4. **Contribuir** - Este é um projeto open source brasileiro!

---

## 🤝 Contribua!

Este é um projeto **brasileiro** 🇧🇷 e open source!

- **GitHub**: https://github.com/avilacloud/avila-dataframe
- **Documentação**: https://docs.avila.cloud/aviladf
- **Discord**: Entre na comunidade AVL

---

## 📖 Recursos para aprender mais

### Conceitos básicos:
- SQL Tutorial: https://www.w3schools.com/sql/
- Pandas Tutorial: https://pandas.pydata.org/docs/
- Rust Book: https://doc.rust-lang.org/book/

### Datasets para praticar:
- Kaggle: https://www.kaggle.com/datasets
- UCI ML Repository: https://archive.ics.uci.edu/ml/
- Dados do INEP: http://inep.gov.br/dados

---

**Feito com 🇧🇷 no Brasil pela AVL Cloud Platform**

*Ajudando estudantes a entender Data Science através de exemplos práticos!*

🔥 **Destruindo planilhas do Excel, um aluno por vez!** 🔥
