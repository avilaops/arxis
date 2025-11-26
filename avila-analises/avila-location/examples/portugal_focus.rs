//! Example: Portugal-specific analysis

use avila_location::*;

fn main() {
    println!("🇵🇹 Portugal Location Analysis\n");

    // Load all regions
    let all_regions = data::load_all_regions();

    // Filter only Portugal
    let portugal_regions: Vec<_> = all_regions
        .iter()
        .filter(|r| r.region.location.country == Country::Portugal)
        .collect();

    println!("📍 Analyzing {} Portuguese regions\n", portugal_regions.len());

    // List all regions
    println!("📋 Available regions:");
    for region in &portugal_regions {
        let region_type = match region.region.region_type {
            RegionType::Metropolitan => "🏙️  Metropolitan",
            RegionType::Urban => "🏘️  Urban",
            RegionType::Rural => "🌾 Interior",
            _ => "📍 Other",
        };

        println!("  • {} - {} (Pop: {})",
            region.region.location.name,
            region_type,
            region.region.location.population
        );
    }

    // Analyze with bootstrap weights (for someone starting with limited budget)
    println!("\n💰 Analysis for Bootstrap Scenario:");
    println!("   (Prioritizing low cost and basic infrastructure)\n");

    let weights = ScoringWeights::bootstrap();
    let mut scores: Vec<_> = portugal_regions
        .iter()
        .map(|r| r.calculate_score(&weights))
        .collect();

    scoring::rank_locations(&mut scores);

    println!("🏆 Top 5 for Bootstrap:");
    for score in scores.iter().take(5) {
        let region = portugal_regions
            .iter()
            .find(|r| r.region.location.id == score.location_id)
            .unwrap();

        println!("  {}. {} - Score: {:.1}/100",
            score.rank.unwrap(),
            score.location_name,
            score.total_score
        );
        println!("     💰 Cost Index: {:.1} | 🏢 Market: {:.1} | 🌐 Infrastructure: {:.1}",
            region.economic.cost_of_living_index,
            region.market.market_score(),
            region.infrastructure.infrastructure_score()
        );
        println!("     📍 Office rent: €{:.0}/m² | Residential: €{:.0}/month",
            region.economic.office_rent_per_m2,
            region.economic.residential_rent
        );
        println!();
    }

    // Highlight interior regions with tax benefits
    println!("\n🎁 Interior Regions with Tax Incentives:");
    for region in portugal_regions.iter() {
        if region.region.region_type == RegionType::Rural {
            println!("  • {} - 50% IRC reduction for 10 years",
                region.region.location.name
            );
            println!("    Effective tax: {:.1}% (vs 21% standard)",
                region.fiscal.effective_corporate_tax_rate()
            );
        }
    }

    // Compare Lisboa vs Porto vs Braga
    println!("\n⚖️  Lisboa vs Porto vs Braga:\n");

    let lisboa = portugal_regions.iter().find(|r| r.region.location.name == "Lisboa").unwrap();
    let porto = portugal_regions.iter().find(|r| r.region.location.name == "Porto").unwrap();
    let braga = portugal_regions.iter().find(|r| r.region.location.name == "Braga").unwrap();

    let regions_to_compare = vec![lisboa, porto, braga];

    println!("  Metric                 | Lisboa    | Porto     | Braga");
    println!("  ----------------------|-----------|-----------|----------");

    for region in &regions_to_compare {
        let score = region.calculate_score(&weights);
        println!("  Overall Score         | {:.1}      | {:.1}      | {:.1}",
            if region.region.location.name == "Lisboa" { score.total_score } else { 0.0 },
            if region.region.location.name == "Porto" { score.total_score } else { 0.0 },
            if region.region.location.name == "Braga" { score.total_score } else { 0.0 }
        );
    }

    println!("\n  Cost of Living Index  | {:.1}      | {:.1}      | {:.1}",
        lisboa.economic.cost_of_living_index,
        porto.economic.cost_of_living_index,
        braga.economic.cost_of_living_index
    );

    println!("  Office Rent (€/m²)    | {:.0}        | {:.0}        | {:.0}",
        lisboa.economic.office_rent_per_m2,
        porto.economic.office_rent_per_m2,
        braga.economic.office_rent_per_m2
    );

    println!("  Tech Companies        | {}     | {}     | {}",
        lisboa.market.total_companies,
        porto.market.total_companies,
        braga.market.total_companies
    );

    println!("  Direct Competitors    | {}        | {}        | {}",
        lisboa.competition.direct_competitors,
        porto.competition.direct_competitors,
        braga.competition.direct_competitors
    );

    println!("  Market Growth Rate    | {:.1}%     | {:.1}%    | {:.1}%",
        lisboa.market.market_growth_rate,
        porto.market.market_growth_rate,
        braga.market.market_growth_rate
    );

    println!("\n💡 Recommendation:");
    println!("   • Lisboa: Best market but highest cost and competition");
    println!("   • Porto: Excellent balance - 30% cheaper, growing fast (12.5%/year)");
    println!("   • Braga: Best value - 50% cheaper, young workforce, low competition");

    println!("\n✨ Analysis complete!");
}
