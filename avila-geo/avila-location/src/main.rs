//! Avila Location Intelligence CLI

use avila_location::*;
use clap::{Parser, Subcommand};
use colored::*;
use std::fs;

#[derive(Parser)]
#[command(name = "avila-location")]
#[command(about = "Geospatial intelligence and market analysis for optimal business location selection", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze all available regions
    Analyze {
        /// Output format (table, json, csv)
        #[arg(short, long, default_value = "table")]
        format: String,

        /// Scenario (default, bootstrap, growth, remote)
        #[arg(short, long, default_value = "default")]
        scenario: String,

        /// Limit results
        #[arg(short, long, default_value = "15")]
        limit: usize,
    },

    /// Compare specific regions
    Compare {
        /// First region ID or name
        region1: String,

        /// Second region ID or name
        region2: String,
    },

    /// List all available regions
    List {
        /// Filter by country (Portugal, UAE)
        #[arg(short, long)]
        country: Option<String>,
    },

    /// Show detailed information about a region
    Detail {
        /// Region ID or name
        region: String,
    },

    /// Generate comprehensive report
    Report {
        /// Output file path
        #[arg(short, long, default_value = "location_report.json")]
        output: String,
    },

    /// Calculate ROI comparison
    Roi {
        /// Initial investment
        #[arg(short, long, default_value = "50000")]
        investment: f64,

        /// Expected annual revenue
        #[arg(short, long, default_value = "120000")]
        revenue: f64,

        /// Number of years
        #[arg(short, long, default_value = "5")]
        years: u32,
    },

    /// Filter regions by criteria
    Filter {
        /// Maximum cost of living index
        #[arg(long)]
        max_cost: Option<f64>,

        /// Minimum market score
        #[arg(long)]
        min_market: Option<f64>,

        /// Maximum competition
        #[arg(long)]
        max_competition: Option<u32>,

        /// Minimum infrastructure score
        #[arg(long)]
        min_infrastructure: Option<f64>,
    },
}

fn main() {
    let cli = Cli::parse();

    // Load all regions
    let regions = data::load_all_regions();

    match cli.command {
        Commands::Analyze { format, scenario, limit } => {
            println!("{}", "🌍 Analyzing Locations...".bright_cyan().bold());
            println!();

            let weights = match scenario.as_str() {
                "bootstrap" => ScoringWeights::bootstrap(),
                "growth" => ScoringWeights::growth_focused(),
                "remote" => ScoringWeights::remote_first(),
                _ => ScoringWeights::default(),
            };

            let analysis = analysis::analyze_regions(&regions, &weights);

            match format.as_str() {
                "json" => {
                    let json = serde_json::to_string_pretty(&analysis.scores).unwrap();
                    println!("{}", json);
                }
                "csv" => {
                    let csv = analysis::export_csv(&analysis.scores);
                    println!("{}", csv);
                }
                _ => {
                    visualization::print_comparison_table(&analysis.scores.iter().take(limit).cloned().collect::<Vec<_>>());

                    println!("\n{}", "📊 Analysis Summary".bright_green().bold());
                    println!("  • Best Overall: {}", analysis.scores.first().map(|s| s.location_name.as_str()).unwrap_or("N/A").bright_yellow());

                    if let Some(idx) = analysis.best_for_bootstrap {
                        println!("  • Best for Bootstrap: {}", regions[idx].region.location.name.bright_yellow());
                    }
                    if let Some(idx) = analysis.best_for_growth {
                        println!("  • Best for Growth: {}", regions[idx].region.location.name.bright_yellow());
                    }
                    if let Some(idx) = analysis.best_for_remote {
                        println!("  • Best for Remote: {}", regions[idx].region.location.name.bright_yellow());
                    }
                }
            }
        }

        Commands::Compare { region1, region2 } => {
            let r1 = find_region(&regions, &region1);
            let r2 = find_region(&regions, &region2);

            match (r1, r2) {
                (Some(reg1), Some(reg2)) => {
                    println!("{}", format!("⚖️  Comparing {} vs {}", reg1.region.location.name, reg2.region.location.name).bright_cyan().bold());
                    println!();

                    let comparison = analysis::compare_two_regions(reg1, reg2);

                    let winner_name = if comparison.winner == 1 { &comparison.region1_name } else { &comparison.region2_name };

                    println!("  🏆 Winner: {}", winner_name.bright_green().bold());
                    println!();
                    println!("  Scores:");
                    println!("    {} {:.1}/100", comparison.region1_name, comparison.region1_score);
                    println!("    {} {:.1}/100", comparison.region2_name, comparison.region2_score);
                    println!();
                    println!("  Differences:");
                    println!("    Cost: {:.1} points", comparison.cost_difference);
                    println!("    Market: {:.1} points", comparison.market_difference);
                    println!("    Competition: {:.1} points", comparison.competition_difference);
                }
                _ => {
                    eprintln!("{}", "❌ One or both regions not found".red());
                }
            }
        }

        Commands::List { country } => {
            println!("{}", "📍 Available Regions".bright_cyan().bold());
            println!();

            let filtered: Vec<_> = if let Some(ref c) = country {
                regions.iter().filter(|r| r.region.location.country.name().to_lowercase().contains(&c.to_lowercase())).collect()
            } else {
                regions.iter().collect()
            };

            let mut current_country = "";
            for region in filtered {
                let country_name = region.region.location.country.name();
                if country_name != current_country {
                    println!("\n  {}", country_name.bright_yellow().bold());
                    current_country = country_name;
                }
                println!("    • {} [{}]", region.region.location.name, region.region.location.id.dimmed());
            }
            println!();
        }

        Commands::Detail { region } => {
            if let Some(reg) = find_region(&regions, &region) {
                let score = reg.calculate_score(&ScoringWeights::default());
                visualization::print_region_detail(&score);

                println!("  📊 Detailed Metrics:");
                println!("    Population: {:?}", reg.region.location.population);
                println!("    GDP per capita: {:.0} {}", reg.economic.gdp_per_capita, reg.region.location.country.currency_symbol());
                println!("    Cost of living index: {:.1}", reg.economic.cost_of_living_index);
                println!("    Office rent: {:.0} {}/m²", reg.economic.office_rent_per_m2, reg.region.location.country.currency_symbol());
                println!("    Corporate tax: {:.1}%", reg.fiscal.corporate_tax_rate);
                println!("    Tech companies: {}", reg.market.total_companies);
                println!("    Direct competitors: {}", reg.competition.direct_competitors);
                println!();
            } else {
                eprintln!("{}", "❌ Region not found".red());
            }
        }

        Commands::Report { output } => {
            println!("{}", "📝 Generating Report...".bright_cyan().bold());

            let weights = ScoringWeights::default();
            let analysis = analysis::analyze_regions(&regions, &weights);
            let report = analysis::generate_report(&regions, &analysis);

            let json = analysis::export_json(&report).unwrap();
            fs::write(&output, json).expect("Failed to write report");

            println!("{}", format!("✅ Report saved to {}", output).bright_green());
        }

        Commands::Roi { investment, revenue, years } => {
            println!("{}", "💰 ROI Comparison".bright_cyan().bold());
            println!();

            let comparisons = analysis::calculate_roi_comparison(&regions, investment, revenue, years);

            let mut sorted = comparisons.clone();
            sorted.sort_by(|a, b| b.roi.partial_cmp(&a.roi).unwrap());

            for (i, comp) in sorted.iter().take(10).enumerate() {
                let rank_icon = match i {
                    0 => "🥇",
                    1 => "🥈",
                    2 => "🥉",
                    _ => "  ",
                };

                println!("  {} {} - ROI: {:.1}% | Profit: {:.0} | Tax: {:.1}%",
                    rank_icon,
                    comp.location,
                    comp.roi,
                    comp.total_profit,
                    comp.effective_tax_rate
                );
            }
            println!();
        }

        Commands::Filter { max_cost, min_market, max_competition, min_infrastructure } => {
            println!("{}", "🔍 Filtering Regions...".bright_cyan().bold());
            println!();

            let filtered = analysis::filter_regions(
                &regions,
                max_cost,
                min_market,
                max_competition,
                min_infrastructure,
            );

            println!("  Found {} matching regions:", filtered.len());
            println!();

            for region in filtered {
                println!("    • {} - Cost: {:.1} | Market: {:.1} | Competition: {}",
                    region.region.location.name,
                    region.economic.cost_of_living_index,
                    region.market.market_score(),
                    region.competition.direct_competitors
                );
            }
            println!();
        }
    }
}

fn find_region<'a>(regions: &'a [data::RegionData], query: &str) -> Option<&'a data::RegionData> {
    let query_lower = query.to_lowercase();
    regions.iter().find(|r| {
        r.region.location.id.to_lowercase() == query_lower ||
        r.region.location.name.to_lowercase() == query_lower ||
        r.region.location.name.to_lowercase().contains(&query_lower)
    })
}
