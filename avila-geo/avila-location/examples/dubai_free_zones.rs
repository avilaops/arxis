//! Example: Dubai Free Zones analysis

use avila_location::*;

fn main() {
    println!("🇦🇪 Dubai Free Zones Analysis\n");

    // Load all regions
    let all_regions = data::load_all_regions();

    // Filter UAE regions
    let uae_regions: Vec<_> = all_regions
        .iter()
        .filter(|r| r.region.location.country == Country::UAE)
        .collect();

    // Filter only Free Zones
    let free_zones: Vec<_> = uae_regions
        .iter()
        .filter(|r| r.region.region_type == RegionType::FreeZone)
        .collect();

    println!("📍 Dubai has {} Free Zones with special benefits:\n", free_zones.len());

    // List Free Zones with benefits
    for zone in &free_zones {
        println!("🏢 {}", zone.region.location.name);
        println!("   Benefits:");
        println!("   ✅ 0% Corporate Tax");
        println!("   ✅ 0% VAT");
        println!("   ✅ 100% Foreign Ownership");
        println!("   ✅ No Currency Restrictions");
        println!("   Office rent: AED {:.0}/m² (≈ €{:.0}/m²)",
            zone.economic.office_rent_per_m2,
            zone.economic.office_rent_per_m2 / 4.2
        );

        if zone.region.location.name.contains("Internet City") {
            println!("   🎯 Specialization: Technology & IT");
            println!("   🌐 Fiber: {}% | 5G: {}%",
                zone.infrastructure.fiber_coverage_percent,
                zone.infrastructure.five_g_coverage_percent
            );
        }

        println!();
    }

    // Compare Free Zones vs Mainland Dubai
    println!("⚖️  Free Zone vs Mainland Dubai Comparison:\n");

    let dic = free_zones.iter()
        .find(|r| r.region.location.name.contains("Internet City"))
        .unwrap();

    let mainland_dubai = uae_regions.iter()
        .find(|r| r.region.location.name == "Dubai" && r.region.region_type != RegionType::FreeZone)
        .unwrap();

    println!("  Metric                      | Free Zone (DIC) | Mainland Dubai");
    println!("  ----------------------------|-----------------|----------------");
    println!("  Corporate Tax               | 0%              | 9%");
    println!("  VAT                         | 0%              | 5%");
    println!("  Foreign Ownership           | 100%            | 100%*");
    println!("  Mainland Business Allowed   | No              | Yes");
    println!("  Office Rent (AED/m²)        | {:.0}             | {:.0}",
        dic.economic.office_rent_per_m2,
        mainland_dubai.economic.office_rent_per_m2
    );
    println!("  Setup Time                  | 2-3 days        | 1-2 weeks");
    println!("  Visa Sponsorship            | Yes             | Yes");

    println!("\n  * Foreign ownership 100% since 2021, previously required UAE partner\n");

    // ROI Analysis
    println!("💰 5-Year ROI Comparison (€100k investment, €200k annual revenue):\n");

    let investment = 100_000.0;
    let annual_revenue = 200_000.0;

    let scenarios = vec![
        ("Dubai Free Zone", 0.0, 0.0),
        ("Dubai Mainland", 9.0, 5.0),
        ("Portugal (Lisboa)", 21.0, 23.0),
        ("Portugal (Interior)", 10.5, 23.0),
    ];

    println!("  Location              | Tax Rate | 5-Year Profit | ROI");
    println!("  ----------------------|----------|---------------|--------");

    for (name, corp_tax, vat) in scenarios {
        let total_tax_rate = (corp_tax + vat) / 100.0;
        let annual_profit = annual_revenue * (1.0 - total_tax_rate);
        let five_year_profit = annual_profit * 5.0;
        let roi = ((five_year_profit - investment) / investment) * 100.0;

        println!("  {:20}  | {:.1}%     | €{:.0}k      | {:.0}%",
            name,
            corp_tax + vat,
            five_year_profit / 1000.0,
            roi
        );
    }

    // Cost breakdown
    println!("\n📊 Monthly Cost Breakdown (50m² office + 1 employee):\n");

    println!("  Dubai Internet City (Free Zone):");
    println!("    • Office rent: AED 2,750 (€655)");
    println!("    • Employee visa: AED 500 (€119)");
    println!("    • Developer salary: AED 10,500 (€2,500)");
    println!("    • Total: AED 13,750 (€3,274)");
    println!();

    println!("  Portugal (Porto):");
    println!("    • Office rent: €600");
    println!("    • Employee taxes: €528 (23.75% of €2,200)");
    println!("    • Developer salary: €2,200");
    println!("    • Total: €3,328");
    println!();

    println!("💡 Key Insights:");
    println!("   • Dubai Free Zones: 0% tax but higher salaries (+120%)");
    println!("   • Portugal: Lower salaries but 21% corporate + 23% VAT");
    println!("   • Free Zones optimal for high-margin consulting/SaaS");
    println!("   • Portugal better for bootstrapping with limited capital");

    println!("\n⚠️  Important Notes:");
    println!("   • Free Zone companies cannot do business in UAE mainland");
    println!("   • Mainland license required for local UAE clients");
    println!("   • Can have both Free Zone + Mainland entities");
    println!("   • Free Zone companies can serve international clients");

    println!("\n✨ Analysis complete!");
}
