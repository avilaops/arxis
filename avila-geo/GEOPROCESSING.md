# Sistema Completo de Geoprocessamento - Avila Geo

## 🚀 Visão Geral

Sistema completo de geoprocessamento em Rust com **zero dependências externas** para operações core, oferecendo:

- ✅ **Estruturas de Dados Espaciais** - QuadTree, R-Tree, índices espaciais
- ✅ **Operações Geométricas** - Buffer, union, intersection, convex hull
- ✅ **Análise Espacial** - Interpolação IDW, Kernel Density, estatísticas espaciais
- ✅ **Sistema de Coordenadas (CRS)** - WGS84, Web Mercator, UTM
- ✅ **Análise de Redes** - Dijkstra, A*, centralidade, componentes conectados
- ✅ **Análise de Terreno (DEM)** - Slope, aspect, viewshed, hillshade
- ✅ **Engine Unificado** - Interface completa para geoprocessamento
- 🚀 **Processamento Paralelo** - Rayon para operações multi-core
- 🤖 **Machine Learning** - Clustering espacial (K-Means, DBSCAN, Hierarchical)
- ⚡ **Cache Inteligente** - LRU cache para otimização de consultas

## 📦 Instalação

```toml
[dependencies]
avila-geo = { version = "0.1", features = ["geoprocessing", "parallel"] }
```

## 🎯 Features

```toml
[features]
default = ["geojson"]
geojson = ["serde", "serde_json"]
geoprocessing = ["geo", "geo-types", "rstar", "petgraph", "nalgebra"]
parallel = ["rayon"]  # Processamento paralelo
simd = ["wide"]       # Otimizações SIMD
full = ["geojson", "geoprocessing", "parallel", "simd"]
```

## 📚 Módulos do Sistema

### 1. Estruturas de Dados Espaciais (`spatial`)

```rust
use avila_geo::geoprocessing::spatial::{BoundingBox, QuadTree, SpatialFeature};
use avila_geo::coords::GeoCoord;

// Criar QuadTree para consultas espaciais eficientes
let bounds = BoundingBox::new(-50.0, -30.0, -40.0, -20.0);
let mut qtree = QuadTree::new(bounds, 10); // capacidade por nó

// Inserir pontos
qtree.insert(GeoCoord::new(-23.55, -46.63), "São Paulo");
qtree.insert(GeoCoord::new(-22.91, -43.17), "Rio de Janeiro");

// Query espacial
let query_bounds = BoundingBox::new(-24.0, -47.0, -22.0, -43.0);
let mut results = Vec::new();
qtree.query(&query_bounds, &mut results);

// Query por raio
let center = GeoCoord::new(-23.0, -46.0);
let mut nearby = Vec::new();
qtree.query_radius(&center, 100000.0, &mut nearby); // 100km
```

**Características:**
- QuadTree para particionamento espacial
- BoundingBox com operações de interseção, união, contenção
- SpatialFeature genérico para geometrias (Point, LineString, Polygon, Multi*)

### 2. Operações Espaciais (`operations`)

```rust
use avila_geo::geoprocessing::operations::{
    point_in_polygon, buffer_polygon, convex_hull, polygon_area,
    line_intersects_line, simplify_line
};

// Point in polygon (Ray Casting)
let polygon = vec![
    GeoCoord::new(0.0, 0.0),
    GeoCoord::new(0.0, 10.0),
    GeoCoord::new(10.0, 10.0),
    GeoCoord::new(10.0, 0.0),
];
let inside = point_in_polygon(&GeoCoord::new(5.0, 5.0), &polygon);

// Buffer (zona de influência)
let buffered = buffer_polygon(&polygon, 0.1);

// Convex Hull (fecho convexo)
let hull = convex_hull(&scattered_points);

// Área de polígono (fórmula de Shoelace)
let area = polygon_area(&polygon);

// Simplificação de linhas (Douglas-Peucker)
let simplified = simplify_line(&complex_line, 0.01);
```

**Traits disponíveis:**
- `SpatialRelation`: intersects, contains, within, touches, crosses, overlaps
- `GeometryOps`: buffer, union, intersection, difference, symmetric_difference, convex_hull

### 3. Análise Espacial (`analysis`)

```rust
use avila_geo::geoprocessing::analysis::{
    haversine_distance, idw_interpolation, kernel_density,
    morans_i, getis_ord_gi_star, Grid, SpatialWeights, LisaAnalysis
};

// Distância Haversine (metros)
let distance = haversine_distance(&coord1, &coord2);

// Interpolação IDW (Inverse Distance Weighting)
let stations = vec![
    (GeoCoord::new(-23.55, -46.63), 22.0), // temperatura em °C
    (GeoCoord::new(-22.91, -43.17), 28.0),
];
let temp = idw_interpolation(&stations, &target_coord, 2.0);

// Kernel Density Estimation
let grid = Grid::new(bounds, 100, 100);
let density = kernel_density(&crime_points, &grid, 0.02);

// Autocorrelação espacial (Moran's I)
let weights = SpatialWeights::from_distance(&features, 0.05);
let morans = morans_i(&features, &values, &weights);

// Análise de hotspots (Getis-Ord Gi*)
let gi_star = getis_ord_gi_star(&features, &values, &weights, 0);

// LISA (Local Indicators of Spatial Association)
let lisa = LisaAnalysis::compute(&features, &values, &weights);
let categories = lisa.classify(&values, 0.05);
```

**Recursos:**
- Cálculos de distância (Haversine, Euclidiana)
- Interpolação espacial (IDW)
- Análise de densidade (Kernel Density)
- Estatísticas espaciais (Moran's I, Getis-Ord, LISA)

### 4. Sistema de Coordenadas (`crs`)

```rust
use avila_geo::geoprocessing::crs::{
    CoordinateTransformer, CoordinateSystem, UTMZoneInfo
};

// WGS84 → Web Mercator
let (x, y) = CoordinateTransformer::wgs84_to_web_mercator(&coord);
let converted = CoordinateTransformer::web_mercator_to_wgs84(x, y);

// WGS84 → UTM
let (zone, north) = CoordinateTransformer::get_utm_zone(&coord);
let (easting, northing) = CoordinateTransformer::wgs84_to_utm(&coord, zone, north);
let wgs84 = CoordinateTransformer::utm_to_wgs84(easting, northing, zone, north);

// Transformação genérica
let from = CoordinateSystem::WGS84;
let to = CoordinateSystem::UTM { zone: 23, north: false };
let transformed = CoordinateTransformer::transform(&coord, &from, &to);

// Web Mercator tiles (para mapas web)
let (tile_x, tile_y) = CoordinateTransformer::wgs84_to_web_mercator_tile(&coord, 10);
```

**Sistemas suportados:**
- WGS84 (GPS padrão)
- Web Mercator (Google Maps, OpenStreetMap)
- UTM (Universal Transverse Mercator)
- Sistema customizado

### 5. Análise de Redes (`network`)

```rust
use avila_geo::geoprocessing::network::{
    SpatialNetwork, NetworkNode, NetworkEdge, Path
};

let mut network = SpatialNetwork::new();

// Adicionar nós
network.add_node(NetworkNode::new(0, coord).with_property("name", "Centro"));
network.add_node(NetworkNode::new(1, coord2).with_property("name", "Subúrbio"));

// Adicionar arestas com pesos
let weight = haversine_distance(&coord, &coord2);
network.add_edge(NetworkEdge::new(0, 0, 1, weight));

// Caminho mais curto (Dijkstra)
let path = network.shortest_path(0, 5).unwrap();
println!("Distância: {:.2}m", path.total_cost);

// A* com heurística geográfica
let path_astar = network.shortest_path_astar(0, 5).unwrap();

// Centralidade de grau
let degree_centrality = network.degree_centrality();

// Centralidade de intermediação
let betweenness = network.betweenness_centrality();

// Componentes conectados
let components = network.connected_components();

// Árvore geradora mínima
let mst_edges = network.minimum_spanning_tree();

// Nós dentro de um raio
let nearby = network.nodes_within_radius(&center, 5000.0);
```

**Algoritmos:**
- Dijkstra (caminho mais curto)
- A* com heurística espacial
- Centralidade (degree, betweenness)
- Componentes conectados
- Minimum Spanning Tree (Prim)

### 6. Análise de Terreno (`terrain`)

```rust
use avila_geo::geoprocessing::terrain::DigitalElevationModel;

let bounds = BoundingBox::new(-50.0, -30.0, -40.0, -20.0);
let mut dem = DigitalElevationModel::new(bounds, 0.01);

// Definir elevações
dem.set_elevation(row, col, 1000.0);

// Obter elevação em coordenada (interpolada)
let elevation = dem.elevation_at(&coord).unwrap();

// Calcular inclinação (slope) em graus
let slope = dem.slope(row, col).unwrap();

// Calcular orientação (aspect) em graus (0-360)
let aspect = dem.aspect(row, col).unwrap();

// Gerar mapas completos
let slope_map = dem.slope_map();
let aspect_map = dem.aspect_map();

// Hillshade (sombreamento para visualização)
let hillshade = dem.hillshade(315.0, 45.0); // azimute, altitude

// Viewshed (análise de visibilidade)
let viewshed = dem.viewshed(obs_row, obs_col, 2.0); // altura do observador

// Perfil de elevação
let profile = dem.elevation_profile(&start, &end, 100);

// Contornos (isolinhas)
let contours = dem.contour_lines(500.0); // contorno de 500m

// Curvatura do terreno
let curvature = dem.curvature(row, col).unwrap();

// Detecção de picos
let peaks = dem.find_peaks(50.0); // proeminência mínima de 50m
```

**Recursos:**
- Digital Elevation Model (DEM)
- Slope (inclinação) e Aspect (orientação)
- Hillshade para visualização
- Viewshed (análise de visibilidade)
- Perfis de elevação
- Detecção de picos

### 7. Engine de Geoprocessamento (`engine`)

```rust
use avila_geo::geoprocessing::engine::{
    GeoprocessingEngine, Layer, OverlayOp, Raster
};

let mut engine = GeoprocessingEngine::new();

// Criar e adicionar camadas
let mut layer = Layer::new("POIs");
layer.add_feature(SpatialFeature::point(0, coord));
engine.add_layer(layer);

// Query espacial
let results = engine.query_spatial("POIs", &bounds);

// Análise de proximidade
let nearby = engine.proximity_analysis("POIs", &center, 5000.0);

// Overlay operations
let intersection = engine.overlay("layer1", "layer2", OverlayOp::Intersection);
let union = engine.overlay("layer1", "layer2", OverlayOp::Union);
let difference = engine.overlay("layer1", "layer2", OverlayOp::Difference);

// Buffer
let buffered = engine.buffer_layer("POIs", 0.01);

// Rasterização de vetores
let raster = engine.rasterize("POIs", 0.001);

// Vetorização de raster
let vector_layer = engine.vectorize("raster", 0.5);

// Estatísticas de camada
let stats = engine.layer_statistics("POIs").unwrap();
println!("Features: {}", stats.feature_count);
println!("Área total: {:.2}", stats.total_area);

// Adicionar rede e DEM
engine.add_network("roads", network);
engine.add_dem("elevation", dem);
```

**Operações:**
- Query espacial com índices
- Análise de proximidade
- Overlay (union, intersection, difference)
- Buffer (zona de influência)
- Rasterização / Vetorização
- Estatísticas de camadas

## 🎮 Exemplos Completos

### Exemplo 1: Análise Espacial Completa

```bash
cargo run --example geoprocessing_demo --features geoprocessing
```

Demonstra:
- QuadTree para consultas espaciais
- Interpolação IDW
- Kernel Density Estimation
- Análise estatística espacial (Moran's I)
- Operações geométricas

### Exemplo 2: Análise de Redes

```bash
cargo run --example network_analysis --features geoprocessing
```

Demonstra:
- Criação de redes de transporte
- Dijkstra e A* para roteamento
- Análise de centralidade
- Componentes conectados
- Árvore geradora mínima

### Exemplo 3: Análise de Terreno

```bash
cargo run --example terrain_analysis --features geoprocessing
```

Demonstra:
- Digital Elevation Model
- Slope e Aspect
- Hillshade
- Viewshed (análise de visibilidade)
- Perfis de elevação
- Detecção de picos

## 🧪 Testes

Execute todos os testes:

```bash
cargo test --all-features
```

Testes por módulo:

```bash
cargo test --features geoprocessing spatial
cargo test --features geoprocessing operations
cargo test --features geoprocessing analysis
cargo test --features geoprocessing network
cargo test --features geoprocessing terrain
```

## 📊 Benchmarks

Execute benchmarks de performance:

```bash
cargo bench --features geoprocessing
```

## 🎯 Casos de Uso

### 1. Análise Urbana
- Planejamento de transporte público
- Análise de acessibilidade a serviços
- Estudo de tráfego e rotas ótimas
- Zoneamento e uso do solo

### 2. Análise Ambiental
- Modelagem de terreno
- Análise de bacias hidrográficas
- Visibilidade e impacto visual
- Áreas de proteção ambiental

### 3. Análise Criminal
- Hotspot detection
- Patrulhamento otimizado
- Análise de distribuição espacial de crimes

### 4. Saúde Pública
- Análise de clusters de doenças
- Acessibilidade a hospitais
- Planejamento de ambulâncias

### 5. Imobiliário
- Análise de preços por região
- Acessibilidade e valorização
- Clusters de mercado

## 🚀 Performance

- **QuadTree**: O(log n) para inserção e query
- **Dijkstra**: O((V + E) log V)
- **A***: Mais rápido que Dijkstra com heurística geográfica
- **Kernel Density**: O(n × m × k) onde n=pontos, m=células, k=bandwidth
- **Viewshed**: O(n²) onde n=células do DEM

## 📖 Documentação

Gere a documentação completa:

```bash
cargo doc --open --features geoprocessing
```

## 🤝 Contribuindo

Este é um projeto open-source. Contribuições são bem-vindas!

## 📜 Licença

MIT OR Apache-2.0

## 🎓 Referências

- **Algoritmos Geoespaciais**: De Berg et al. "Computational Geometry"
- **Análise Espacial**: Anselin, Luc. "Spatial Econometrics"
- **Redes Espaciais**: Rodrigue et al. "The Geography of Transport Systems"
- **DEM**: USGS Digital Elevation Model standards

---

**Avila Geo** - Sistema completo de geoprocessamento em Rust 🦀🗺️
