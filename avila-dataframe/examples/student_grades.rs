//! Exemplo Prático para Estudantes Universitários
//!
//! Este exemplo demonstra como usar o AvilaDB DataFrame para:
//! - Analisar notas de alunos em diferentes disciplinas
//! - Calcular médias por turma e disciplina
//! - Identificar alunos com baixo desempenho
//! - Fazer ranking de estudantes
//! - Visualizar dados agregados

use avila_dataframe::prelude::*;
use avila_dataframe::ops::{JoinType, SortOrder, PivotAggFunc};

fn main() -> Result<()> {
    println!("🎓 Sistema de Análise de Notas Acadêmicas");
    println!("==========================================\n");

    // ========== 1. DADOS DOS ALUNOS ==========
    println!("📊 1. DADOS DOS ALUNOS E NOTAS");
    println!("━".repeat(60));

    // DataFrame com notas de alunos em diferentes provas
    let notas = DataFrame::new(vec![
        Series::new("aluno_id", vec![1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4.0, 4.0, 4.0]),
        Series::new("disciplina", vec![1.0, 2.0, 3.0, 1.0, 2.0, 3.0, 1.0, 2.0, 3.0, 1.0, 2.0, 3.0]),
        // Disciplinas: 1=Cálculo, 2=Física, 3=Programação
        Series::new("nota", vec![8.5, 7.0, 9.5, 6.0, 5.5, 7.5, 9.0, 8.5, 10.0, 4.5, 6.0, 7.0]),
        Series::new("presenca", vec![95.0, 90.0, 100.0, 80.0, 75.0, 85.0, 100.0, 95.0, 100.0, 70.0, 65.0, 75.0]),
    ])?;

    println!("Notas dos Alunos:");
    println!("{}", notas);

    // DataFrame com informações dos alunos
    let alunos = DataFrame::new(vec![
        Series::new("aluno_id", vec![1.0, 2.0, 3.0, 4.0]),
        Series::new("nome_id", vec![1001.0, 1002.0, 1003.0, 1004.0]), // João, Maria, Pedro, Ana
        Series::new("semestre", vec![4.0, 3.0, 5.0, 2.0]),
    ])?;

    println!("\nInformações dos Alunos:");
    println!("{}", alunos);

    // ========== 2. FILTRAR ALUNOS EM RISCO ==========
    println!("\n📊 2. IDENTIFICAR ALUNOS EM RISCO (nota < 6.0)");
    println!("━".repeat(60));

    let em_risco = notas
        .filter(col("nota").lt(lit(6.0)))?;

    println!("Alunos com notas abaixo de 6.0:");
    println!("{}", em_risco);
    println!("⚠️  Total de provas abaixo da média: {} registros", em_risco.height());

    // ========== 3. CALCULAR MÉDIA POR ALUNO ==========
    println!("\n📊 3. MÉDIA GERAL POR ALUNO");
    println!("━".repeat(60));

    let media_por_aluno = notas
        .group_by(&["aluno_id"])?
        .agg(&[
            col("nota").mean().alias("media_geral"),
            col("nota").sum().alias("soma_notas"),
            col("presenca").mean().alias("media_presenca"),
        ])?;

    println!("Médias por Aluno:");
    println!("{}", media_por_aluno);

    // ========== 4. MÉDIA POR DISCIPLINA ==========
    println!("\n📊 4. DESEMPENHO POR DISCIPLINA");
    println!("━".repeat(60));

    let media_por_disciplina = notas
        .group_by(&["disciplina"])?
        .agg(&[
            col("nota").mean().alias("media_turma"),
            col("nota").std().alias("desvio_padrao"),
            col("nota").min().alias("nota_minima"),
            col("nota").max().alias("nota_maxima"),
        ])?;

    println!("Estatísticas por Disciplina:");
    println!("(1=Cálculo, 2=Física, 3=Programação)");
    println!("{}", media_por_disciplina);

    // ========== 5. RANKING DE ALUNOS ==========
    println!("\n📊 5. RANKING GERAL DOS ALUNOS");
    println!("━".repeat(60));

    let ranking = media_por_aluno
        .sort("media_geral", SortOrder::Descending)?;

    println!("Ranking (do melhor para o pior):");
    println!("{}", ranking);

    // ========== 6. JOIN COM INFORMAÇÕES DOS ALUNOS ==========
    println!("\n📊 6. RELATÓRIO COMPLETO (com informações pessoais)");
    println!("━".repeat(60));

    let relatorio_completo = media_por_aluno
        .join(&alunos, "aluno_id", "aluno_id", JoinType::Inner)?
        .sort("media_geral", SortOrder::Descending)?;

    println!("Relatório Completo:");
    println!("{}", relatorio_completo);

    // ========== 7. ANÁLISE ESPECÍFICA: CÁLCULO ==========
    println!("\n📊 7. ANÁLISE ESPECÍFICA - DISCIPLINA DE CÁLCULO");
    println!("━".repeat(60));

    let calculo = notas
        .filter(col("disciplina").eq(lit(1.0)))?  // 1 = Cálculo
        .sort("nota", SortOrder::Descending)?;

    println!("Notas em Cálculo (ordenadas):");
    println!("{}", calculo);

    // ========== 8. ALUNOS COM BOA PRESENÇA E BOAS NOTAS ==========
    println!("\n📊 8. ALUNOS EXEMPLARES (presença > 90% E nota > 8.0)");
    println!("━".repeat(60));

    let exemplares = notas
        .filter(col("presenca").gt(lit(90.0)))?
        .filter(col("nota").gt(lit(8.0)))?;

    println!("Registros de Alunos Exemplares:");
    println!("{}", exemplares);
    println!("🌟 Total de registros exemplares: {}", exemplares.height());

    // ========== 9. PIVOT: MATRIZ DE NOTAS ==========
    println!("\n📊 9. MATRIZ DE NOTAS (Aluno × Disciplina)");
    println!("━".repeat(60));

    let matriz_notas = notas
        .pivot(
            &["aluno_id"],
            "disciplina",
            "nota",
            PivotAggFunc::Mean
        )?;

    println!("Matriz Aluno × Disciplina:");
    println!("{}", matriz_notas);

    // ========== 10. ANÁLISE COMPARATIVA ==========
    println!("\n📊 10. ANÁLISE COMPARATIVA - QUEM ESTÁ ACIMA DA MÉDIA?");
    println!("━".repeat(60));

    // Calcular média geral da turma
    let todas_notas: Vec<f64> = (0..notas.height())
        .map(|i| notas.column("nota").unwrap().get_f64(i).unwrap())
        .collect();
    let media_turma = todas_notas.iter().sum::<f64>() / todas_notas.len() as f64;

    println!("📈 Média geral da turma: {:.2}", media_turma);

    let acima_media = media_por_aluno
        .filter(col("media_geral").gt(lit(media_turma)))?;

    println!("\nAlunos com média acima da turma:");
    println!("{}", acima_media);

    // ========== 11. RECOMENDAÇÕES PERSONALIZADAS ==========
    println!("\n📊 11. RECOMENDAÇÕES PERSONALIZADAS");
    println!("━".repeat(60));

    println!("\n🎯 SISTEMA DE RECOMENDAÇÕES:");
    println!();

    for i in 0..media_por_aluno.height() {
        let aluno_id = media_por_aluno.column("aluno_id")?.get_f64(i)?;
        let media = media_por_aluno.column("media_geral")?.get_f64(i)?;
        let presenca = media_por_aluno.column("media_presenca")?.get_f64(i)?;

        println!("👤 Aluno ID {:.0}:", aluno_id);
        println!("   Média: {:.2}", media);
        println!("   Presença: {:.1}%", presenca);

        // Recomendações baseadas em regras
        if media < 6.0 {
            println!("   ⚠️  ALERTA: Risco de reprovação!");
            println!("   💡 Recomendação: Procure monitoria e grupos de estudo");
        } else if media < 7.0 {
            println!("   ⚡ Atenção: Desempenho pode melhorar");
            println!("   💡 Recomendação: Revise os conceitos básicos");
        } else if media < 8.0 {
            println!("   ✅ Bom desempenho!");
            println!("   💡 Recomendação: Continue assim e aprofunde estudos");
        } else if media < 9.0 {
            println!("   🌟 Ótimo desempenho!");
            println!("   💡 Recomendação: Considere projetos de pesquisa");
        } else {
            println!("   🏆 Excelente! Top da turma!");
            println!("   💡 Recomendação: Candidate-se a bolsas e intercâmbio");
        }

        if presenca < 75.0 {
            println!("   ⚠️  Presença baixa! Risco de reprovação por falta");
        }
        println!();
    }

    // ========== 12. ESTATÍSTICAS FINAIS ==========
    println!("\n📊 12. RESUMO ESTATÍSTICO GERAL");
    println!("━".repeat(60));

    let total_alunos = alunos.height();
    let total_provas = notas.height();
    let aprovados = media_por_aluno
        .filter(col("media_geral").gt(lit(6.0)))?
        .height();
    let reprovados = total_alunos - aprovados;

    println!("📚 Total de alunos: {}", total_alunos);
    println!("📝 Total de avaliações: {}", total_provas);
    println!("✅ Alunos aprovados (média ≥ 6.0): {}", aprovados);
    println!("❌ Alunos em risco (média < 6.0): {}", reprovados);
    println!("📊 Taxa de aprovação: {:.1}%", (aprovados as f64 / total_alunos as f64) * 100.0);
    println!("📈 Média geral da turma: {:.2}", media_turma);

    // ========== CONCLUSÃO ==========
    println!("\n" + &"=".repeat(60));
    println!("✅ ANÁLISE COMPLETA!");
    println!("=".repeat(60));
    println!();
    println!("🎓 Este exemplo demonstrou:");
    println!("   ✅ Filtragem de dados (alunos em risco)");
    println!("   ✅ Agregações (médias por aluno e disciplina)");
    println!("   ✅ Ordenação (ranking de desempenho)");
    println!("   ✅ Joins (combinar dados de múltiplas tabelas)");
    println!("   ✅ Pivot (transformar dados em matriz)");
    println!("   ✅ Análises comparativas e recomendações");
    println!();
    println!("💡 Casos de uso no mundo real:");
    println!("   • Sistemas de gestão acadêmica");
    println!("   • Plataformas de ensino online");
    println!("   • Análise de desempenho estudantil");
    println!("   • Identificação de alunos em risco");
    println!("   • Geração de relatórios automáticos");
    println!();
    println!("🚀 Com AvilaDB DataFrame você pode:");
    println!("   • Processar milhões de notas rapidamente");
    println!("   • Integrar com bancos de dados (AvilaDB)");
    println!("   • Fazer análises em tempo real");
    println!("   • Gerar insights automáticos");
    println!();
    println!("🔥 Destruindo planilhas do Excel, um aluno por vez! 🇧🇷");

    Ok(())
}
