//! Quickstart - AvilaDB DataFrame 100% Rust Nativo
//!
//! Zero overhead, máxima simplicidade.

use avila_dataframe::prelude::*;

fn main() -> Result<()> {
    println!("🚀 AvilaDB DataFrame - 100% Rust Nativo\n");

    // ========== CRIAÇÃO ==========
    println!("1️⃣  Criar DataFrame");
    println!("{}", "=".repeat(60));

    let df = DataFrame::from_series(vec![
        Series::new_str(
            "nome",
            vec![
                "Ana".to_string(),
                "Bruno".to_string(),
                "Carlos".to_string(),
                "Diana".to_string(),
            ],
        ),
        Series::new_int("idade", vec![25, 30, 28, 32]),
        Series::new_float("salario", vec![5000.0, 6500.0, 5800.0, 7200.0]),
        Series::new_float("bonus", vec![500.0, 650.0, 580.0, 720.0]),
    ])?;

    println!("{}\n", df);

    // ========== SELEÇÃO ==========
    println!("2️⃣  Selecionar Colunas");
    println!("{}", "=".repeat(60));

    let subset = df.select(&["nome", "salario"])?;
    println!("{}\n", subset);

    // ========== FILTRO ==========
    println!("3️⃣  Filtrar Linhas");
    println!("{}", "=".repeat(60));

    // Criar mask: salário > 6000
    let salario_col = df.column("salario")?;
    let mask: Vec<bool> = (0..df.height())
        .map(|i| {
            if let Some(Value::Float(sal)) = salario_col.get(i) {
                *sal > 6000.0
            } else {
                false
            }
        })
        .collect();

    let filtered = df.filter(&mask)?;
    println!("Pessoas com salário > 6000:");
    println!("{}\n", filtered);

    // ========== HEAD/TAIL ==========
    println!("4️⃣  Head & Tail");
    println!("{}", "=".repeat(60));

    println!("Primeiros 2:");
    println!("{}", df.head(2));

    println!("Últimos 2:");
    println!("{}\n", df.tail(2));

    // ========== ESTATÍSTICAS ==========
    println!("5️⃣  Estatísticas");
    println!("{}", "=".repeat(60));

    let stats = df.describe();
    println!("{}\n", stats);

    // ========== AGREGAÇÕES ==========
    println!("6️⃣  Agregações");
    println!("{}", "=".repeat(60));

    let salario = df.column("salario")?;
    println!("Salário médio: {:.2}", salario.mean().unwrap());
    println!("Salário total: {:.2}", salario.sum().unwrap());
    println!("Salário mínimo: {:.2}", salario.min().unwrap());
    println!("Salário máximo: {:.2}\n", salario.max().unwrap());

    // ========== ITERAÇÃO ==========
    println!("7️⃣  Iterar Linhas");
    println!("{}", "=".repeat(60));

    println!("Primeira linha:");
    if let Ok(row) = df.row(0) {
        println!("  {:?}\n", row);
    }

    // ========== TRANSFORMAÇÃO ==========
    println!("8️⃣  Transformar Dados");
    println!("{}", "=".repeat(60));

    let bonus = df.column("bonus")?;
    let bonus_dobrado = bonus.map(|v| {
        if let Value::Float(val) = v {
            Value::Float(val * 2.0)
        } else {
            v.clone()
        }
    });

    println!("Bônus original:");
    for (i, val) in bonus.data.iter().enumerate().take(4) {
        println!("  [{}] {}", i, val);
    }

    println!("\nBônus dobrado:");
    for (i, val) in bonus_dobrado.data.iter().enumerate().take(4) {
        println!("  [{}] {}", i, val);
    }

    // ========== RESUMO ==========
    println!("\n{}", "=".repeat(60));
    println!("✅ SUCESSO!");
    println!("{}", "=".repeat(60));
    println!("Shape: {:?}", df.shape());
    println!("Colunas: {:?}", df.column_names());
    println!("Linhas: {}", df.height());
    println!("\n🔥 DataFrame 100% Rust - Zero overhead! 🇧🇷");

    Ok(())
}
