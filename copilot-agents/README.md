# Avelan Copilot Agents - Portugal Market Analysis System

## Overview

This directory contains **7 specialized Copilot agents** designed to work together to provide comprehensive market intelligence and location optimization for business expansion into Portugal.

## Agent Architecture

```
┌─────────────────────────────────────────────────────────┐
│         Integration Orchestrator (Master Agent)         │
│  Coordinates all agents, manages workflow, delivers     │
│  final analysis                                         │
└────────────────┬────────────────────────────────────────┘
                 │
        ┌────────┴────────┐
        │                 │
    ┌───▼───┐         ┌──▼───┐
    │ Data  │         │ ML   │
    │Extract│         │Pred. │
    └───┬───┘         └──┬───┘
        │                │
    ┌───▼────┐       ┌──▼─────┐
    │ Market │       │ Geo    │
    │ Intel  │       │Spatial │
    └───┬────┘       └──┬─────┘
        │               │
    ┌───▼─────┐    ┌───▼────┐
    │Financial│    │  Viz   │
    │Optimize │    │Dashboard│
    └─────────┘    └────────┘
```

## Agents

### 1. [Integration Orchestrator](./integration-orchestrator/)
**Role**: Master coordinator and system architect

**Responsibilities**:
- Coordinate all 6 specialized agents
- Manage complete analysis workflow
- Handle error recovery and retry logic
- Provide CLI and API interfaces
- Generate final reports

**Key Deliverable**: Complete market analysis with optimal location recommendation

---

### 2. [Geospatial Analysis](./geospatial-analysis/)
**Role**: Location intelligence and spatial optimization

**Responsibilities**:
- Implement spatial algorithms (R-Trees, Voronoi diagrams, etc.)
- Calculate distances, accessibility, and travel times
- Generate isochrones and service areas
- Optimize facility locations (Weber problem, P-Median)
- Perform viewshed and terrain analysis

**Algorithms**:
- Haversine distance, Vincenty formula
- Dijkstra, A* pathfinding
- K-nearest neighbors spatial queries
- Kernel density estimation
- Hot spot analysis (Getis-Ord Gi*)

---

### 3. [Market Intelligence](./market-intelligence/)
**Role**: Competitive analysis and market research

**Responsibilities**:
- Segment markets using K-Means, DBSCAN
- Analyze competition (Porter's Five Forces)
- Forecast demand (ARIMA, Prophet)
- Score and qualify leads
- Identify market opportunities (TAM/SAM/SOM)

**Data Sources**:
- INE Portugal (statistics)
- Companies House Portugal
- LinkedIn, Crunchbase
- Google Trends, SimilarWeb
- Job boards (ITJobs.pt, LinkedIn Jobs)

---

### 4. [Financial Optimization](./financial-optimization/)
**Role**: Tax optimization and cost minimization

**Responsibilities**:
- Portugal tax system optimization (IRC, VAT, incentives)
- Corporate structure optimization
- Cost-benefit analysis (NPV, IRR, ROI)
- Monte Carlo risk simulation
- Pricing optimization

**Portugal-Specific**:
- IRC: 21% standard, 12.5% interior regions
- SIFIDE R&D tax credit (up to 82.5%)
- Patent Box (50% exemption)
- Interior region incentives

---

### 5. [Data Extraction & Web Scraping](./data-extraction/)
**Role**: Automated data collection and integration

**Responsibilities**:
- Scrape business directories (LinkedIn, Google Maps, Yelp)
- Extract job postings for demand analysis
- Collect company information
- Monitor competitor pricing
- API integration (Crunchbase, Places API)

**Features**:
- Ethical scraping (respect robots.txt, rate limits)
- Anti-detection strategies
- JavaScript rendering (headless browsers)
- Data cleaning and normalization
- Deduplication and quality control

---

### 6. [Machine Learning & Prediction](./ml-prediction/)
**Role**: Predictive modeling and forecasting

**Responsibilities**:
- Lead scoring (Logistic Regression, Random Forest)
- Churn prediction
- Demand forecasting (ARIMA, Holt-Winters)
- Market segmentation (clustering)
- Recommendation systems

**Models**:
- Supervised: Classification, Regression
- Unsupervised: K-Means, DBSCAN, PCA
- Time Series: ARIMA, Exponential Smoothing
- Deep Learning: LSTM, Neural Networks

---

### 7. [Data Visualization](./visualization/)
**Role**: Interactive dashboards and reporting

**Responsibilities**:
- Interactive web dashboards
- Choropleth maps for regional analysis
- Time series charts and forecasts
- Competition matrices and scatter plots
- PDF/PowerPoint report generation

**Technologies**:
- D3.js, Plotly, Chart.js
- Leaflet, Mapbox GL
- React dashboards
- Export to PDF, Excel, PowerPoint

---

## Quick Start

### 1. Run Complete Analysis
```bash
cd d:\GitHub\Avelan
cargo run --bin avelan analyze --budget 5000 --regions "Porto,Braga,Coimbra"
```

### 2. Start Interactive Dashboard
```bash
cargo run --bin avelan dashboard --port 3000
```
Then open: http://localhost:3000

### 3. Update Market Data
```bash
cargo run --bin avelan update
```

### 4. Generate Report
```bash
cargo run --bin avelan report --analysis-id abc123 --format pdf
```

## Workflow Example

Here's how the agents work together:

```rust
// 1. User specifies preferences
let preferences = UserPreferences {
    budget_eur: 5000.0,
    preferred_regions: vec!["Porto", "Braga"],
    industry_focus: vec![Industry::Technology],
    cost_weight: 0.30,
    market_weight: 0.25,
    // ...
};

// 2. Orchestrator coordinates agents
let orchestrator = SystemOrchestrator::new().await?;

// 3. Data collection
let companies = data_agent.scrape_company_data().await?;
let jobs = data_agent.extract_job_postings().await?;

// 4. Market analysis
let segments = market_agent.segment_market(&companies).await?;
let demand = ml_agent.forecast_demand(&historical).await?;

// 5. Location optimization
let locations = geospatial_agent.find_optimal_locations(&criteria).await?;
let accessibility = geospatial_agent.calculate_accessibility(&locations).await?;

// 6. Financial analysis
let costs = financial_agent.analyze_costs(&locations).await?;
let tax = financial_agent.optimize_tax(&locations).await?;

// 7. Lead scoring
let leads = ml_agent.score_leads(&companies).await?;

// 8. Decision
let optimal = orchestrator.select_optimal_location(...).await?;

// 9. Visualization
let dashboard = viz_agent.generate_dashboard(&optimal).await?;
let report = viz_agent.generate_pdf_report(&optimal).await?;
```

## Output Examples

### Console Output
```
🚀 Starting Portugal Market Analysis...

📊 Phase 1: Data Collection
  ✓ Scraped 1,247 companies
  ✓ Extracted 834 job postings
  ✓ Collected market data for 18 districts

🔍 Phase 2: Market Analysis
  ✓ Identified 5 market segments
  ✓ Analyzed 23 competitors
  ✓ Forecasted 12-month demand

📍 Phase 3: Location Optimization
  ✓ Evaluated 8 candidate locations
  ✓ Calculated accessibility scores
  ✓ Generated isochrones

💰 Phase 4: Financial Optimization
  ✓ Analyzed costs for all locations
  ✓ Optimized tax structure
  ✓ Projected 5-year ROI

🎯 Phase 5: Lead Generation & Scoring
  ✓ Scored 1,247 potential clients
  ✓ Identified 87 hot leads

⚖️  Phase 6: Multi-Criteria Optimization
  ✓ Selected optimal location

📈 Phase 7: Generating Reports & Dashboards
  ✓ Created interactive dashboard
  ✓ Generated PDF report

✅ Analysis Complete!

================================================================================
🎉 OPTIMAL LOCATION FOUND
================================================================================

📍 Location: Porto
⭐ Overall Score: 87.3/100

📊 Score Breakdown:
  💰 Cost: 26.1
  📈 Market: 22.5
  🏛️  Tax: 16.8
  🚗 Accessibility: 13.2
  🏡 Quality of Life: 8.7

🎯 Top Reasons:
  1. 30% lower costs than Lisboa
  2. Growing tech hub with 234 potential clients
  3. Excellent accessibility (30min to airport)
  4. Strong talent pool (FEUP, UPorto)
  5. High quality of life

💼 Market Opportunity:
  Total Addressable Market: €45.3M
  Projected 1st Year Revenue: €180K
  Break-Even: 8 months

🎯 Top 5 Potential Clients:
  1. Farfetch (Score: 94)
  2. SWORD Health (Score: 91)
  3. Talkdesk (Score: 88)
  4. Unbabel (Score: 85)
  5. Feedzai (Score: 83)

📊 Dashboard: http://localhost:3000/dashboard/abc123
📄 Full Report: output/portugal_analysis.pdf
```

### Dashboard Preview
- **Map View**: Choropleth map of Portugal showing market scores by region
- **Cost Chart**: Bar chart comparing monthly costs across cities
- **Forecast**: Line chart with 12-month revenue projection
- **Competition Matrix**: Scatter plot of competition vs opportunity
- **Lead Table**: Sortable, filterable table of scored leads

## Configuration

Each agent can be configured via:

1. **Code**: Modify agent parameters directly
2. **Config Files**: YAML/TOML configuration
3. **Environment Variables**: `.env` files
4. **CLI Arguments**: Command-line flags

Example `.env`:
```env
AVILADB_ENDPOINT=http://localhost:8000
LINKEDIN_API_KEY=your_key
CRUNCHBASE_API_KEY=your_key
GOOGLE_MAPS_API_KEY=your_key
```

## Testing

Each agent includes:
- **Unit Tests**: Test individual functions
- **Integration Tests**: Test agent interactions
- **Benchmarks**: Performance testing
- **E2E Tests**: Complete workflow validation

Run tests:
```bash
cargo test --workspace
cargo test --package geospatial-analysis
cargo bench
```

## Performance

Target metrics:
- **Complete Analysis**: < 5 minutes
- **Dashboard Load**: < 2 seconds
- **Map Rendering**: < 500ms
- **API Response**: < 100ms
- **Concurrent Users**: 1000+

## Deployment

### Docker
```bash
docker build -t avelan .
docker run -p 3000:3000 avelan
```

### Kubernetes
```bash
kubectl apply -f k8s/deployment.yaml
```

### Serverless
Deploy individual agents as AWS Lambda or Azure Functions

## Monitoring

- **Logs**: Structured logging with `tracing`
- **Metrics**: Prometheus metrics
- **Alerts**: Alert on failures, degraded performance
- **Health Checks**: `/health` endpoint for each agent

## Documentation

Each agent folder contains:
- `INSTRUCTIONS.md`: Detailed agent specifications
- `README.md`: Quick start guide
- `examples/`: Usage examples
- `docs/`: Additional documentation

## Contributing

1. Read agent instructions
2. Implement features following guidelines
3. Add tests
4. Update documentation
5. Submit for review

## License

See main project LICENSE

---

**Built for Portugal market expansion** 🇵🇹

**Powered by**: Rust, AvilaDB, Machine Learning, GIS
