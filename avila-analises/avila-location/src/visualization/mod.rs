//! Visualization utilities

use crate::models::LocationScore;

pub mod table;

pub use table::*;

/// Simple bar chart for terminal
pub fn print_bar_chart(title: &str, data: &[(String, f64)], width: usize) {
    println!("\n{}", title);
    println!("{}", "=".repeat(width + 30));

    let max_value = data.iter().map(|(_, v)| v).fold(0.0, |a, b| a.max(*b));

    for (label, value) in data {
        let bar_length = ((value / max_value) * width as f64) as usize;
        let bar = "█".repeat(bar_length);
        println!("{:20} {} {:.1}", label, bar, value);
    }

    println!();
}

/// Print comparison table
pub fn print_comparison_table(scores: &[LocationScore]) {
    use comfy_table::{Table, Cell, Attribute, Color};

    let mut table = Table::new();
    table.set_header(vec![
        Cell::new("Rank").add_attribute(Attribute::Bold),
        Cell::new("Location").add_attribute(Attribute::Bold),
        Cell::new("Score").add_attribute(Attribute::Bold),
        Cell::new("Cost").add_attribute(Attribute::Bold),
        Cell::new("Market").add_attribute(Attribute::Bold),
        Cell::new("Competition").add_attribute(Attribute::Bold),
        Cell::new("Infrastructure").add_attribute(Attribute::Bold),
    ]);

    for score in scores.iter().take(15) {
        let rank_cell = if score.rank == Some(1) {
            Cell::new(format!("🥇 {}", score.rank.unwrap())).fg(Color::Yellow)
        } else if score.rank == Some(2) {
            Cell::new(format!("🥈 {}", score.rank.unwrap())).fg(Color::Grey)
        } else if score.rank == Some(3) {
            Cell::new(format!("🥉 {}", score.rank.unwrap())).fg(Color::Rgb { r: 205, g: 127, b: 50 })
        } else {
            Cell::new(score.rank.unwrap().to_string())
        };

        table.add_row(vec![
            rank_cell,
            Cell::new(&score.location_name),
            Cell::new(format!("{:.1}", score.total_score)),
            Cell::new(format!("{:.1}", score.cost_of_living)),
            Cell::new(format!("{:.1}", score.market_demand)),
            Cell::new(format!("{:.1}", score.competition)),
            Cell::new(format!("{:.1}", score.infrastructure)),
        ]);
    }

    println!("\n{}", table);
}

/// Print region detail
pub fn print_region_detail(score: &LocationScore) {
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║  {} - Detailed Analysis", score.location_name);
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();
    println!("  🏆 Overall Score: {:.1}/100", score.total_score);
    println!("  📍 Rank: #{}", score.rank.unwrap_or(0));
    println!();
    println!("  Criteria Breakdown:");
    println!("  ├─ 💰 Cost of Living:      {:.1}/100", score.cost_of_living);
    println!("  ├─ 📈 Market Demand:       {:.1}/100", score.market_demand);
    println!("  ├─ 🤝 Competition:         {:.1}/100", score.competition);
    println!("  ├─ 🌐 Infrastructure:      {:.1}/100", score.infrastructure);
    println!("  ├─ 🏡 Quality of Life:     {:.1}/100", score.quality_of_life);
    println!("  ├─ 💼 Fiscal Incentives:   {:.1}/100", score.fiscal_incentives);
    println!("  ├─ ✈️  Accessibility:       {:.1}/100", score.accessibility);
    println!("  └─ 👥 Talent Availability: {:.1}/100", score.talent_availability);
    println!();
}
