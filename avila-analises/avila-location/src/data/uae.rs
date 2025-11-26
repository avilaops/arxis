//! UAE (United Arab Emirates) region data

use crate::models::*;
use crate::data::RegionData;
use std::collections::HashMap;

/// Load all UAE regions
pub fn load_uae_regions() -> Vec<RegionData> {
    vec![
        create_dubai(),
        create_abu_dhabi(),
        create_sharjah(),
        create_ajman(),
        create_ras_al_khaimah(),
        create_fujairah(),
        create_umm_al_quwain(),
        // Free Zones
        create_dubai_internet_city(),
        create_dubai_silicon_oasis(),
        create_dubai_media_city(),
        create_jafza(),
        create_adgm(), // Abu Dhabi Global Market
    ]
}

/// Dubai - Principal hub tecnológico do Oriente Médio
fn create_dubai() -> RegionData {
    let location = Location::new(
        "AE-DXB",
        "Dubai",
        Country::UAE,
        "Dubai",
        Coordinate::new(25.2048, 55.2708),
        3_604_000,
        4_114.0,
    )
    .with_temperature(27.0)
    .with_property("timezone", "Asia/Dubai")
    .with_property("financial_hub", "true");

    let region = Region {
        location,
        region_type: RegionType::Metropolitan,
        accessibility: AccessibilityMetrics {
            airport_minutes: 15,
            major_city_minutes: 0,
            public_transport_score: 92.0,
            road_quality_score: 98.0,
            international_flights_weekly: 1200,
        },
        urban_development_index: 98.0,
        safety_score: 95.0,
    };

    let mut companies_by_sector = HashMap::new();
    companies_by_sector.insert(Sector::Technology, 2800);
    companies_by_sector.insert(Sector::FinTech, 850);
    companies_by_sector.insert(Sector::ECommerce, 1200);
    companies_by_sector.insert(Sector::Consulting, 1500);
    companies_by_sector.insert(Sector::Marketing, 780);
    companies_by_sector.insert(Sector::RealEstate, 3500);

    let mut companies_by_size = HashMap::new();
    companies_by_size.insert(CompanySize::Micro, 35000);
    companies_by_size.insert(CompanySize::Small, 12000);
    companies_by_size.insert(CompanySize::Medium, 3200);
    companies_by_size.insert(CompanySize::Large, 1200);

    let market = MarketData {
        total_companies: 51_400,
        companies_by_sector,
        companies_by_size,
        digital_maturity_index: 95.0,
        tech_adoption_rate: 92.0,
        avg_it_spending: 125_000.0,
        market_growth_rate: 15.5,
        tech_events_annual: 280,
        tech_communities: 85,
    };

    let competition = CompetitionData {
        direct_competitors: 220,
        indirect_competitors: 450,
        avg_competitor_age: 5.2,
        market_concentration_hhi: 42.0,
        avg_pricing: 15_000.0,
        service_gaps: vec![
            "Arabic language AI".to_string(),
            "Islamic fintech".to_string(),
            "Smart city solutions".to_string(),
        ],
        top_competitors: vec![],
    };

    let infrastructure = InfrastructureData {
        avg_internet_speed_mbps: 450.0,
        fiber_coverage_percent: 99.0,
        five_g_coverage_percent: 95.0,
        avg_datacenter_latency_ms: 3.0,
        coworking_spaces: 120,
        tech_hubs: 25,
        universities_with_cs: 12,
        cs_graduates_annual: 4500,
        power_reliability: 99.9,
    };

    let talent = TalentData {
        tech_workforce: 85_000,
        avg_dev_salary: 8_500.0, // AED (≈ 2100 EUR)
        salary_vs_market_percent: 180.0,
        unemployment_rate: 2.5,
        tech_job_openings: 8500,
        english_proficiency: 90.0,
        skills: vec![],
    };

    let economic = EconomicData {
        gdp_per_capita: 43_000.0,
        gdp_growth_rate: 6.2,
        cost_of_living_index: 85.0,
        office_rent_per_m2: 45.0, // AED (≈ 11 EUR)
        residential_rent: 6_500.0, // AED (≈ 1600 EUR)
        food_cost_index: 18.0,
        transportation_cost: 180.0,
        healthcare_quality: 92.0,
        fdi_annual_millions: 15_000.0,
    };

    let fiscal = create_dubai_fiscal_data();

    let quality_of_life = QualityOfLife {
        livability_index: 88.0,
        safety_score: 95.0,
        healthcare_score: 92.0,
        education_score: 85.0,
        culture_score: 95.0,
        climate_score: 65.0, // Hot climate
        expat_community_size: 3_100_000,
        air_quality_index: 65.0,
    };

    RegionData {
        region,
        market,
        competition,
        infrastructure,
        talent,
        economic,
        fiscal,
        quality_of_life,
    }
}

/// Abu Dhabi - Capital dos Emirados
fn create_abu_dhabi() -> RegionData {
    let location = Location::new(
        "AE-AUH",
        "Abu Dhabi",
        Country::UAE,
        "Abu Dhabi",
        Coordinate::new(24.4539, 54.3773),
        1_807_000,
        972.0,
    )
    .with_temperature(27.5)
    .with_property("capital", "true");

    let region = Region {
        location,
        region_type: RegionType::Metropolitan,
        accessibility: AccessibilityMetrics {
            airport_minutes: 20,
            major_city_minutes: 0,
            public_transport_score: 85.0,
            road_quality_score: 98.0,
            international_flights_weekly: 650,
        },
        urban_development_index: 95.0,
        safety_score: 96.0,
    };

    let mut companies_by_sector = HashMap::new();
    companies_by_sector.insert(Sector::Technology, 1200);
    companies_by_sector.insert(Sector::FinTech, 450);
    companies_by_sector.insert(Sector::ECommerce, 520);
    companies_by_sector.insert(Sector::Consulting, 880);

    let mut companies_by_size = HashMap::new();
    companies_by_size.insert(CompanySize::Micro, 18000);
    companies_by_size.insert(CompanySize::Small, 6500);
    companies_by_size.insert(CompanySize::Medium, 1800);
    companies_by_size.insert(CompanySize::Large, 650);

    let market = MarketData {
        total_companies: 26_950,
        companies_by_sector,
        companies_by_size,
        digital_maturity_index: 92.0,
        tech_adoption_rate: 88.0,
        avg_it_spending: 110_000.0,
        market_growth_rate: 12.0,
        tech_events_annual: 150,
        tech_communities: 45,
    };

    let competition = CompetitionData {
        direct_competitors: 95,
        indirect_competitors: 180,
        avg_competitor_age: 6.8,
        market_concentration_hhi: 38.0,
        avg_pricing: 12_000.0,
        service_gaps: vec![
            "Oil & gas digital transformation".to_string(),
            "Government digitalization".to_string(),
        ],
        top_competitors: vec![],
    };

    let infrastructure = InfrastructureData {
        avg_internet_speed_mbps: 420.0,
        fiber_coverage_percent: 98.0,
        five_g_coverage_percent: 92.0,
        avg_datacenter_latency_ms: 4.0,
        coworking_spaces: 65,
        tech_hubs: 15,
        universities_with_cs: 8,
        cs_graduates_annual: 2200,
        power_reliability: 99.8,
    };

    let talent = TalentData {
        tech_workforce: 42_000,
        avg_dev_salary: 9_200.0, // AED
        salary_vs_market_percent: 195.0,
        unemployment_rate: 2.0,
        tech_job_openings: 3800,
        english_proficiency: 88.0,
        skills: vec![],
    };

    let economic = EconomicData {
        gdp_per_capita: 49_000.0,
        gdp_growth_rate: 5.5,
        cost_of_living_index: 82.0,
        office_rent_per_m2: 42.0, // AED
        residential_rent: 6_000.0, // AED
        food_cost_index: 17.0,
        transportation_cost: 170.0,
        healthcare_quality: 94.0,
        fdi_annual_millions: 12_000.0,
    };

    let fiscal = create_abu_dhabi_fiscal_data();

    let quality_of_life = QualityOfLife {
        livability_index: 90.0,
        safety_score: 96.0,
        healthcare_score: 94.0,
        education_score: 88.0,
        culture_score: 92.0,
        climate_score: 65.0,
        expat_community_size: 1_500_000,
        air_quality_index: 70.0,
    };

    RegionData {
        region,
        market,
        competition,
        infrastructure,
        talent,
        economic,
        fiscal,
        quality_of_life,
    }
}

/// Sharjah - Vizinho de Dubai, mais acessível
fn create_sharjah() -> RegionData {
    let location = Location::new(
        "AE-SHJ",
        "Sharjah",
        Country::UAE,
        "Sharjah",
        Coordinate::new(25.3463, 55.4209),
        1_684_000,
        2_590.0,
    )
    .with_temperature(27.0);

    let region = Region {
        location,
        region_type: RegionType::Urban,
        accessibility: AccessibilityMetrics {
            airport_minutes: 25,
            major_city_minutes: 20,
            public_transport_score: 72.0,
            road_quality_score: 92.0,
            international_flights_weekly: 120,
        },
        urban_development_index: 82.0,
        safety_score: 92.0,
    };

    let mut companies_by_sector = HashMap::new();
    companies_by_sector.insert(Sector::Technology, 380);
    companies_by_sector.insert(Sector::Manufacturing, 850);
    companies_by_sector.insert(Sector::Logistics, 520);

    let mut companies_by_size = HashMap::new();
    companies_by_size.insert(CompanySize::Micro, 8500);
    companies_by_size.insert(CompanySize::Small, 2800);
    companies_by_size.insert(CompanySize::Medium, 680);
    companies_by_size.insert(CompanySize::Large, 220);

    let market = MarketData {
        total_companies: 12_200,
        companies_by_sector,
        companies_by_size,
        digital_maturity_index: 75.0,
        tech_adoption_rate: 78.0,
        avg_it_spending: 55_000.0,
        market_growth_rate: 10.5,
        tech_events_annual: 45,
        tech_communities: 18,
    };

    let competition = CompetitionData {
        direct_competitors: 28,
        indirect_competitors: 65,
        avg_competitor_age: 4.5,
        market_concentration_hhi: 25.0,
        avg_pricing: 7_500.0,
        service_gaps: vec![
            "Industrial IoT".to_string(),
            "Supply chain digitalization".to_string(),
        ],
        top_competitors: vec![],
    };

    let infrastructure = InfrastructureData {
        avg_internet_speed_mbps: 380.0,
        fiber_coverage_percent: 92.0,
        five_g_coverage_percent: 85.0,
        avg_datacenter_latency_ms: 6.0,
        coworking_spaces: 32,
        tech_hubs: 8,
        universities_with_cs: 5,
        cs_graduates_annual: 850,
        power_reliability: 99.5,
    };

    let talent = TalentData {
        tech_workforce: 12_500,
        avg_dev_salary: 6_800.0, // AED
        salary_vs_market_percent: 145.0,
        unemployment_rate: 3.2,
        tech_job_openings: 950,
        english_proficiency: 82.0,
        skills: vec![],
    };

    let economic = EconomicData {
        gdp_per_capita: 32_000.0,
        gdp_growth_rate: 7.0,
        cost_of_living_index: 62.0,
        office_rent_per_m2: 28.0, // AED
        residential_rent: 3_500.0, // AED
        food_cost_index: 14.0,
        transportation_cost: 140.0,
        healthcare_quality: 85.0,
        fdi_annual_millions: 3_200.0,
    };

    let fiscal = create_dubai_fiscal_data(); // Same tax regime

    let quality_of_life = QualityOfLife {
        livability_index: 82.0,
        safety_score: 92.0,
        healthcare_score: 85.0,
        education_score: 80.0,
        culture_score: 88.0,
        climate_score: 65.0,
        expat_community_size: 1_200_000,
        air_quality_index: 75.0,
    };

    RegionData {
        region,
        market,
        competition,
        infrastructure,
        talent,
        economic,
        fiscal,
        quality_of_life,
    }
}

/// Outras cidades menores
fn create_ajman() -> RegionData {
    create_small_emirate("AE-AJM", "Ajman", 25.4052, 55.5136, 504_000, 259.0)
}

fn create_ras_al_khaimah() -> RegionData {
    create_small_emirate("AE-RAK", "Ras Al Khaimah", 25.7889, 55.9433, 416_000, 1684.0)
}

fn create_fujairah() -> RegionData {
    create_small_emirate("AE-FUJ", "Fujairah", 25.1289, 56.3267, 256_000, 1580.0)
}

fn create_umm_al_quwain() -> RegionData {
    create_small_emirate("AE-UAQ", "Umm Al Quwain", 25.5647, 55.5552, 84_000, 777.0)
}

/// Dubai Internet City - Free Zone especializada em TI
fn create_dubai_internet_city() -> RegionData {
    let location = Location::new(
        "AE-DIC",
        "Dubai Internet City",
        Country::UAE,
        "Dubai",
        Coordinate::new(25.0945, 55.1562),
        50_000,
        5.0,
    )
    .with_temperature(27.0)
    .with_property("free_zone", "true")
    .with_property("specialization", "technology");

    let region = Region {
        location,
        region_type: RegionType::FreeZone,
        accessibility: AccessibilityMetrics {
            airport_minutes: 25,
            major_city_minutes: 10,
            public_transport_score: 90.0,
            road_quality_score: 98.0,
            international_flights_weekly: 1200,
        },
        urban_development_index: 98.0,
        safety_score: 98.0,
    };

    let mut companies_by_sector = HashMap::new();
    companies_by_sector.insert(Sector::Technology, 1800);
    companies_by_sector.insert(Sector::FinTech, 320);

    let mut companies_by_size = HashMap::new();
    companies_by_size.insert(CompanySize::Micro, 800);
    companies_by_size.insert(CompanySize::Small, 650);
    companies_by_size.insert(CompanySize::Medium, 280);
    companies_by_size.insert(CompanySize::Large, 120);

    let market = MarketData {
        total_companies: 1_850,
        companies_by_sector,
        companies_by_size,
        digital_maturity_index: 98.0,
        tech_adoption_rate: 98.0,
        avg_it_spending: 200_000.0,
        market_growth_rate: 22.0,
        tech_events_annual: 95,
        tech_communities: 32,
    };

    let competition = CompetitionData {
        direct_competitors: 180,
        indirect_competitors: 320,
        avg_competitor_age: 4.2,
        market_concentration_hhi: 32.0,
        avg_pricing: 18_000.0,
        service_gaps: vec!["Web3 development".to_string()],
        top_competitors: vec![],
    };

    let infrastructure = InfrastructureData {
        avg_internet_speed_mbps: 500.0,
        fiber_coverage_percent: 100.0,
        five_g_coverage_percent: 100.0,
        avg_datacenter_latency_ms: 1.0,
        coworking_spaces: 45,
        tech_hubs: 8,
        universities_with_cs: 0,
        cs_graduates_annual: 0,
        power_reliability: 100.0,
    };

    let talent = TalentData {
        tech_workforce: 35_000,
        avg_dev_salary: 10_500.0, // AED - Premium
        salary_vs_market_percent: 220.0,
        unemployment_rate: 1.0,
        tech_job_openings: 4200,
        english_proficiency: 95.0,
        skills: vec![],
    };

    let economic = EconomicData {
        gdp_per_capita: 65_000.0,
        gdp_growth_rate: 12.0,
        cost_of_living_index: 92.0,
        office_rent_per_m2: 55.0, // AED - Premium
        residential_rent: 7_500.0, // AED
        food_cost_index: 20.0,
        transportation_cost: 200.0,
        healthcare_quality: 95.0,
        fdi_annual_millions: 8_500.0,
    };

    let fiscal = create_free_zone_fiscal_data();

    let quality_of_life = QualityOfLife {
        livability_index: 92.0,
        safety_score: 98.0,
        healthcare_score: 95.0,
        education_score: 90.0,
        culture_score: 95.0,
        climate_score: 65.0,
        expat_community_size: 45_000,
        air_quality_index: 60.0,
    };

    RegionData {
        region,
        market,
        competition,
        infrastructure,
        talent,
        economic,
        fiscal,
        quality_of_life,
    }
}

/// Dubai Silicon Oasis - Tech Free Zone
fn create_dubai_silicon_oasis() -> RegionData {
    let mut data = create_dubai_internet_city();
    data.region.location.id = "AE-DSO".to_string();
    data.region.location.name = "Dubai Silicon Oasis".to_string();
    data.region.location.coordinate = Coordinate::new(25.1241, 55.3799);
    data.economic.office_rent_per_m2 = 42.0; // Slightly cheaper
    data.competition.direct_competitors = 120;
    data
}

/// Dubai Media City - Media & Creative Free Zone
fn create_dubai_media_city() -> RegionData {
    let mut data = create_dubai_internet_city();
    data.region.location.id = "AE-DMC".to_string();
    data.region.location.name = "Dubai Media City".to_string();
    data.region.location.coordinate = Coordinate::new(25.0973, 55.1628);
    data.market.companies_by_sector.clear();
    data.market.companies_by_sector.insert(Sector::Marketing, 450);
    data.market.companies_by_sector.insert(Sector::Technology, 280);
    data
}

/// JAFZA - Jebel Ali Free Zone
fn create_jafza() -> RegionData {
    let mut data = create_dubai_internet_city();
    data.region.location.id = "AE-JAFZA".to_string();
    data.region.location.name = "Jebel Ali Free Zone".to_string();
    data.region.location.coordinate = Coordinate::new(24.9857, 55.0272);
    data.market.companies_by_sector.clear();
    data.market.companies_by_sector.insert(Sector::Logistics, 1200);
    data.market.companies_by_sector.insert(Sector::Manufacturing, 800);
    data.market.companies_by_sector.insert(Sector::Technology, 220);
    data.economic.office_rent_per_m2 = 35.0; // Cheaper
    data
}

/// ADGM - Abu Dhabi Global Market (Financial Free Zone)
fn create_adgm() -> RegionData {
    let mut data = create_dubai_internet_city();
    data.region.location.id = "AE-ADGM".to_string();
    data.region.location.name = "Abu Dhabi Global Market".to_string();
    data.region.location.coordinate = Coordinate::new(24.4968, 54.3821);
    data.region.location.country = Country::UAE;
    data.market.companies_by_sector.clear();
    data.market.companies_by_sector.insert(Sector::FinTech, 650);
    data.market.companies_by_sector.insert(Sector::Technology, 280);
    data.economic.office_rent_per_m2 = 60.0; // Premium financial district
    data.fiscal = create_adgm_fiscal_data();
    data
}

/// Helper: Create small emirate data
fn create_small_emirate(
    id: &str,
    name: &str,
    lat: f64,
    lon: f64,
    population: u64,
    area: f64,
) -> RegionData {
    let location = Location::new(
        id,
        name,
        Country::UAE,
        name,
        Coordinate::new(lat, lon),
        population,
        area,
    )
    .with_temperature(27.0);

    let region = Region {
        location,
        region_type: RegionType::Urban,
        accessibility: AccessibilityMetrics {
            airport_minutes: 45,
            major_city_minutes: 60,
            public_transport_score: 65.0,
            road_quality_score: 88.0,
            international_flights_weekly: 30,
        },
        urban_development_index: 75.0,
        safety_score: 90.0,
    };

    let mut companies_by_sector = HashMap::new();
    companies_by_sector.insert(Sector::Technology, 120);
    companies_by_sector.insert(Sector::Retail, 350);

    let mut companies_by_size = HashMap::new();
    companies_by_size.insert(CompanySize::Micro, 3500);
    companies_by_size.insert(CompanySize::Small, 950);
    companies_by_size.insert(CompanySize::Medium, 220);
    companies_by_size.insert(CompanySize::Large, 80);

    let market = MarketData {
        total_companies: 4_750,
        companies_by_sector,
        companies_by_size,
        digital_maturity_index: 68.0,
        tech_adoption_rate: 72.0,
        avg_it_spending: 40_000.0,
        market_growth_rate: 9.0,
        tech_events_annual: 18,
        tech_communities: 8,
    };

    let competition = CompetitionData {
        direct_competitors: 12,
        indirect_competitors: 28,
        avg_competitor_age: 5.5,
        market_concentration_hhi: 20.0,
        avg_pricing: 5_500.0,
        service_gaps: vec!["SME digital transformation".to_string()],
        top_competitors: vec![],
    };

    let infrastructure = InfrastructureData {
        avg_internet_speed_mbps: 320.0,
        fiber_coverage_percent: 85.0,
        five_g_coverage_percent: 75.0,
        avg_datacenter_latency_ms: 10.0,
        coworking_spaces: 12,
        tech_hubs: 3,
        universities_with_cs: 2,
        cs_graduates_annual: 280,
        power_reliability: 99.0,
    };

    let talent = TalentData {
        tech_workforce: 3_800,
        avg_dev_salary: 6_000.0, // AED
        salary_vs_market_percent: 127.0,
        unemployment_rate: 3.8,
        tech_job_openings: 320,
        english_proficiency: 78.0,
        skills: vec![],
    };

    let economic = EconomicData {
        gdp_per_capita: 28_000.0,
        gdp_growth_rate: 5.5,
        cost_of_living_index: 55.0,
        office_rent_per_m2: 22.0, // AED
        residential_rent: 2_800.0, // AED
        food_cost_index: 12.0,
        transportation_cost: 120.0,
        healthcare_quality: 80.0,
        fdi_annual_millions: 850.0,
    };

    let fiscal = create_dubai_fiscal_data();

    let quality_of_life = QualityOfLife {
        livability_index: 78.0,
        safety_score: 90.0,
        healthcare_score: 80.0,
        education_score: 75.0,
        culture_score: 80.0,
        climate_score: 65.0,
        expat_community_size: 150_000,
        air_quality_index: 78.0,
    };

    RegionData {
        region,
        market,
        competition,
        infrastructure,
        talent,
        economic,
        fiscal,
        quality_of_life,
    }
}

/// Dubai/Sharjah fiscal data - 0% corporate tax (traditional)
fn create_dubai_fiscal_data() -> FiscalData {
    FiscalData {
        country: Country::UAE,
        corporate_tax_rate: 9.0, // New UAE CT (2023+) on profits > AED 375k
        vat_rate: 5.0,
        social_security_rate: 0.0, // No social security tax
        personal_income_tax: 0.0,
        capital_gains_tax: 0.0,
        dividend_tax: 0.0,
        incentives: vec![
            TaxIncentive {
                name: "Small Business Relief".to_string(),
                description: "0% CT on revenue below AED 3M".to_string(),
                tax_reduction_percent: 100.0,
                duration_years: 999,
                requirements: vec!["Revenue < AED 3M".to_string()],
            },
        ],
        ease_of_business_rank: 16,
    }
}

/// Abu Dhabi fiscal data
fn create_abu_dhabi_fiscal_data() -> FiscalData {
    create_dubai_fiscal_data() // Same UAE tax regime
}

/// Free Zone fiscal data - Even better incentives
fn create_free_zone_fiscal_data() -> FiscalData {
    FiscalData {
        country: Country::UAE,
        corporate_tax_rate: 0.0, // Free zones = 0% CT
        vat_rate: 0.0, // VAT exemption in free zones
        social_security_rate: 0.0,
        personal_income_tax: 0.0,
        capital_gains_tax: 0.0,
        dividend_tax: 0.0,
        incentives: vec![
            TaxIncentive {
                name: "Free Zone Tax Exemption".to_string(),
                description: "0% corporate tax for qualified free zone companies".to_string(),
                tax_reduction_percent: 100.0,
                duration_years: 999,
                requirements: vec![
                    "Registered in free zone".to_string(),
                    "No mainland business".to_string(),
                ],
            },
            TaxIncentive {
                name: "100% Foreign Ownership".to_string(),
                description: "Full foreign ownership allowed".to_string(),
                tax_reduction_percent: 0.0,
                duration_years: 999,
                requirements: vec!["Free zone company".to_string()],
            },
            TaxIncentive {
                name: "Import/Export Duty Exemption".to_string(),
                description: "No customs duty on imports/exports".to_string(),
                tax_reduction_percent: 0.0,
                duration_years: 999,
                requirements: vec!["Free zone company".to_string()],
            },
        ],
        ease_of_business_rank: 11, // Free zones even easier
    }
}

/// ADGM specific fiscal data
fn create_adgm_fiscal_data() -> FiscalData {
    let mut fiscal = create_free_zone_fiscal_data();
    fiscal.incentives.push(TaxIncentive {
        name: "Financial Services Incentives".to_string(),
        description: "Special incentives for financial services companies".to_string(),
        tax_reduction_percent: 0.0,
        duration_years: 50,
        requirements: vec!["FSR license".to_string()],
    });
    fiscal
}
