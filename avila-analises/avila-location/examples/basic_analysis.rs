//! Example: Basic location analysis

use avila_location::*;

fn main() {
    println!("🌍 Avila Location Intelligence - Example\n");

    // Load all available regions
    let regions = data::load_all_regions();
    println!("📍 Loaded {} regions\n", regions.len());

    // Default weights for analysis
    let weights = ScoringWeights::default();

    // Perform comparative analysis
    println!("🔍 Analyzing all regions...");
    let analysis = analysis::analyze_regions(&regions, &weights);

    // Display top 10 results
    println!("\n🏆 Top 10 Locations:\n");
    for (i, score) in analysis.scores.iter().take(10).enumerate() {
        let region = regions
            .iter()
            .find(|r| r.region.location.id == score.location_id)
            .unwrap();

        println!("  {}. {} ({}) - Score: {:.1}/100",
            i + 1,
            score.location_name,
            region.region.location.country.name(),
            score.total_score
        );
    }

    // Show best for different scenarios
    println!("\n📊 Best by Scenario:\n");

    if let Some(idx) = analysis.best_for_bootstrap {
        println!("  💰 Best for Bootstrap: {}", regions[idx].region.location.name);
    }

    if let Some(idx) = analysis.best_for_growth {
        println!("  📈 Best for Growth: {}", regions[idx].region.location.name);
    }

    if let Some(idx) = analysis.best_for_remote {
        println!("  🌐 Best for Remote: {}", regions[idx].region.location.name);
    }

    // Compare Porto vs Dubai
    println!("\n⚖️  Comparing Porto vs Dubai:\n");
    let porto = regions.iter().find(|r| r.region.location.name == "Porto").unwrap();
    let dubai = regions.iter().find(|r| r.region.location.name == "Dubai").unwrap();

    let comparison = analysis::compare_two_regions(porto, dubai);
    println!("  Winner: {}",
        if comparison.winner == 1 { &comparison.region1_name } else { &comparison.region2_name }
    );
    println!("  Porto Score: {:.1}", comparison.region1_score);
    println!("  Dubai Score: {:.1}", comparison.region2_score);

    // ROI Comparison
    println!("\n💰 ROI Comparison (€50k investment, €120k revenue, 5 years):\n");
    let roi_analysis = analysis::calculate_roi_comparison(&regions, 50000.0, 120000.0, 5);

    let mut sorted_roi = roi_analysis.clone();
    sorted_roi.sort_by(|a, b| b.roi.partial_cmp(&a.roi).unwrap());

    for (i, roi) in sorted_roi.iter().take(5).enumerate() {
        println!("  {}. {} - ROI: {:.1}% | Profit: €{:.0}k | Tax: {:.1}%",
            i + 1,
            roi.location,
            roi.roi,
            roi.total_profit / 1000.0,
            roi.effective_tax_rate
        );
    }

    // Generate full report
    println!("\n📝 Generating comprehensive report...");
    let report = analysis::generate_report(&regions, &analysis);

    println!("  ✅ Report generated with {} recommendations", report.recommendations.len());
    println!("  📊 Countries analyzed: {}", report.analysis.countries_covered.join(", "));
    println!("  🏆 Best overall: {}", report.analysis.best_overall);

    println!("\n✨ Analysis complete!");
}
