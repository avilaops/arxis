//! Example: Deep web mapping - crawl and index .onion sites

use avila_darknet::crawler::{DeepWebCrawler, HiddenService};
use avila_darknet::indexer::{InvertedIndex, CategoryIndex};
use avila_darknet::mapper::{DeepWebMap, NodeType};

fn main() {
    println!("=== Deep Web Mapping Example ===\n");

    // 1. Initialize crawler
    println!("1. Initializing crawler...");
    let mut crawler = DeepWebCrawler::new();
    crawler.max_pages = 20;

    // Seed with known .onion addresses
    let seeds = vec![
        "marketplace1.onion".to_string(),
        "forum1.onion".to_string(),
        "wiki.onion".to_string(),
    ];

    crawler.seed(seeds);
    println!("   Seeded with {} addresses", crawler.queue.len());

    // 2. Crawl deep web
    println!("\n2. Crawling .onion sites...");
    let crawled = crawler.crawl_all();

    let stats = crawler.stats();
    println!("   Discovered: {} services", stats.discovered);
    println!("   Visited: {} pages", stats.visited);
    println!("   Online: {} services", stats.online);

    // 3. Build search index
    println!("\n3. Building search index...");
    let mut index = InvertedIndex::new();
    let mut category_index = CategoryIndex::new();

    for (addr, service) in &crawler.discovered {
        // Full-text index
        let title = service.title.clone().unwrap_or_else(|| addr.clone());
        let description = service.description.clone().unwrap_or_default();

        index.add_document(addr.clone(), title, description);

        // Category index
        for category in &service.categories {
            category_index.add(category.clone(), addr.clone());
        }
    }

    let index_stats = index.stats();
    println!("   Indexed: {} documents", index_stats.documents);
    println!("   Terms: {} unique terms", index_stats.terms);
    println!("   Avg terms per doc: {:.1}", index_stats.avg_terms_per_doc);

    // 4. Search
    println!("\n4. Testing search...");

    let queries = vec!["marketplace", "forum", "anonymous"];

    for query in queries {
        let results = index.search(query);
        println!("\n   Query: '{}'", query);
        println!("   Results: {}", results.len());

        for (i, result) in results.iter().take(3).enumerate() {
            println!("     {}. {} (score: {:.2})",
                i + 1,
                result.document.title,
                result.score
            );
        }
    }

    // 5. Category breakdown
    println!("\n5. Categories:");
    let categories = category_index.list_categories();
    for (cat, count) in categories {
        println!("   {} : {} sites", cat, count);
    }

    // 6. Build network map
    println!("\n6. Building network map...");
    let mut map = DeepWebMap::new();

    let mut services_data = Vec::new();
    for (addr, service) in &crawler.discovered {
        let node_type = if service.categories.contains(&"marketplace".to_string()) {
            NodeType::Marketplace
        } else if service.categories.contains(&"forum".to_string()) {
            NodeType::Forum
        } else if service.categories.contains(&"wiki".to_string()) {
            NodeType::Wiki
        } else {
            NodeType::Unknown
        };

        services_data.push((addr.clone(), service.links.clone(), node_type));
    }

    map.build_from_services(services_data);

    let net_stats = map.graph.stats();
    println!("   Nodes: {}", net_stats.nodes);
    println!("   Edges: {}", net_stats.edges);
    println!("   Avg connections: {:.2}", net_stats.avg_connections);
    println!("   Density: {:.4}", net_stats.density);

    // 7. Find hubs (most important sites)
    println!("\n7. Top sites by PageRank:");
    let hubs = map.get_hubs(5);

    for (i, hub) in hubs.iter().enumerate() {
        println!("   {}. {} (PR: {:.4}, type: {:?})",
            i + 1,
            hub.onion_address.split('.').next().unwrap_or(&hub.onion_address),
            hub.pagerank,
            hub.node_type
        );
    }

    // 8. Find communities
    println!("\n8. Network communities:");
    let communities = map.graph.find_communities();
    println!("   Found {} communities", communities.len());

    for (i, community) in communities.iter().enumerate() {
        println!("   Community {}: {} members", i + 1, community.members.len());
    }

    // 9. Export map
    println!("\n9. Exporting network map...");
    let dot = map.export_dot();

    println!("   GraphViz DOT format ({} bytes)", dot.len());
    println!("   Save to 'deepweb_map.dot' and render with:");
    println!("   $ dot -Tpng deepweb_map.dot -o deepweb_map.png");

    // Save to file (simulated)
    println!("\n=== Mapping Complete ===");
    println!("Discovered {} hidden services in the deep web", stats.discovered);
}
