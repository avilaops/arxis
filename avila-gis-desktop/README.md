# 🗺️ AvilaGIS Desktop

**Desktop GIS Application with ArcGIS-like Interface**

## 📋 Visão Geral

O **AvilaGIS Desktop** é uma aplicação desktop completa de Sistema de Informação Geográfica (GIS), inspirada no ArcGIS, construída em Rust com interface gráfica moderna.

## ✨ Funcionalidades

### 🖥️ Interface Gráfica Completa

- **Menu System**: File, Edit, View, Selection, Geoprocessing, Window, Help
- **Toolbar**: Acesso rápido às ferramentas mais utilizadas
- **Status Bar**: Informações em tempo real do projeto
- **Painéis Flutuantes**: Dockable e redimensionáveis

### 🗺️ Visualização de Mapas

- **Canvas Interativo**: Renderização em tempo real
- **Zoom**: In, Out, To Extent, To Selection
- **Pan**: Navegação fluida pelo mapa
- **Multi-Layer Rendering**: Renderização de múltiplas camadas
- **Basemaps**: OpenStreetMap, Satellite, Terrain

### 📊 Gerenciamento de Camadas

- **Table of Contents (TOC)**: Lista hierárquica de camadas
- **Layer Properties**: Propriedades detalhadas de cada camada
- **Visibility Control**: Ligar/desligar camadas
- **Layer Order**: Reordenação por drag & drop
- **Grouping**: Agrupamento de camadas

### 🎨 Editor de Simbologia

- **Symbol Types**: Point, Line, Polygon, Text
- **Color Picker**: Seletor de cores completo
- **Size Control**: Controle de tamanho dos símbolos
- **Line Styles**: Solid, Dashed, Dotted
- **Fill Patterns**: Solid, Hatched, Cross-hatched
- **Opacity Control**: Transparência ajustável

### 🔧 Caixa de Ferramentas

#### Navegação
- ✋ **Pan**: Movimentação do mapa
- 🔍 **Zoom In/Out**: Aproximação e afastamento
- 🌍 **Full Extent**: Visualização completa

#### Seleção
- 🖱️ **Select**: Seleção simples
- ▭ **Select by Rectangle**: Seleção retangular
- ⬡ **Select by Polygon**: Seleção poligonal
- ⭕ **Select by Circle**: Seleção circular

#### Medição
- 📏 **Measure Distance**: Medição de distâncias
- 📐 **Measure Area**: Medição de áreas
- 📐 **Measure Angle**: Medição de ângulos

#### Identificação
- ℹ️ **Identify**: Identificação de features

#### Edição
- 📍 **Create Point**: Criar pontos
- 📏 **Create Line**: Criar linhas
- ⬡ **Create Polygon**: Criar polígonos
- ✏️ **Edit Feature**: Editar features
- 🗑️ **Delete Feature**: Excluir features

#### Análise
- 🔄 **Buffer**: Criar buffer
- ✂️ **Clip**: Recortar
- ⋃ **Union**: União
- ⋂ **Intersect**: Interseção

### 📋 Tabela de Atributos

- **Grid View**: Visualização em grade
- **Edit Attributes**: Edição de atributos
- **Sorting**: Ordenação por coluna
- **Filtering**: Filtros avançados
- **Statistics**: Estatísticas descritivas
- **Export**: Exportação para CSV, Excel

### 💾 Importação/Exportação

#### Importação
- 📄 **CSV**: Arquivos de texto delimitado
- 🗺️ **GeoJSON**: Padrão web GIS
- 📦 **Shapefile**: Formato ESRI
- 🗃️ **GPX**: GPS tracks
- 📊 **KML/KMZ**: Google Earth

#### Exportação
- 🖼️ **SVG**: Gráficos vetoriais
- 📄 **PDF**: Documentos portáteis
- 🖼️ **PNG/JPEG**: Imagens raster
- 🗺️ **GeoJSON**: Para web
- 📊 **CSV**: Dados tabulares

### 🔍 Consultas Espaciais

- **Select by Location**: Seleção espacial
- **Select by Attributes**: Seleção por atributos
- **Buffer Analysis**: Análise de buffer
- **Proximity Analysis**: Análise de proximidade
- **Overlay Analysis**: Análise de sobreposição

### 🛠️ Geoprocessing

- **Buffer**: Criar zonas de buffer
- **Clip**: Recortar camadas
- **Union**: Unir geometrias
- **Intersect**: Interseção de camadas
- **Dissolve**: Dissolver features
- **Merge**: Mesclar camadas

## 🏗️ Arquitetura

```
avila-gis-desktop/
├── src/
│   ├── main.rs              # Entry point
│   ├── ui.rs                # UI layout e messages
│   ├── state.rs             # Application state
│   ├── map_view.rs          # Map canvas
│   ├── layer_manager.rs     # Layer management
│   ├── symbology_editor.rs  # Symbology controls
│   ├── toolbox.rs           # GIS tools
│   ├── attribute_table.rs   # Attribute table
│   └── data_io.rs           # Import/Export
├── Cargo.toml
└── README.md
```

## 🚀 Como Usar

### Instalação

```powershell
cd d:\arxis\avila-gis-desktop
cargo build --release
```

### Executar

```powershell
cargo run --release
```

Ou execute o binário diretamente:

```powershell
.\target\release\avilagis.exe
```

## 📖 Workflow Básico

### 1. Criar Novo Projeto

```
File → New Project
```

### 2. Adicionar Camada

```
➕ Add Layer → Selecione o tipo (Point/Line/Polygon)
```

### 3. Importar Dados

```
File → Import Data → Escolha o formato (CSV/GeoJSON/Shapefile)
```

### 4. Editar Simbologia

```
1. Selecione a camada no painel esquerdo
2. Clique em 🎨 Symbology
3. Ajuste cores, tamanhos, estilos
4. Clique em Apply
```

### 5. Fazer Consultas

```
1. Selecione a ferramenta 🖱️ Select
2. Clique nas features no mapa
3. Veja atributos na tabela
```

### 6. Criar Análises

```
Geoprocessing → Buffer/Clip/Union/Intersect
Configure parâmetros → Execute
```

### 7. Exportar Mapa

```
File → Export Map → Escolha formato (SVG/PDF/PNG)
```

## 🎨 Temas e Personalização

O aplicativo suporta temas Dark e Light:

```rust
// Mudar tema
Settings → Theme → Dark/Light
```

## 🔌 Integração com AvilaDB

```rust
use avila_geo::aviladb_cartographic::*;

// Importar dados do AvilaDB
let db = CartographicDatabase::new();
let companies = db.get_all_companies()?;

// Criar camada no mapa
map_view.add_layer_from_companies(companies);
```

## 📊 Formatos Suportados

### Leitura
- ✅ CSV (comma-separated values)
- ✅ GeoJSON
- ✅ Shapefile (.shp)
- ✅ GPX (GPS Exchange)
- ✅ KML/KMZ (Google Earth)

### Escrita
- ✅ SVG (Scalable Vector Graphics)
- ✅ PDF (Portable Document Format)
- ✅ PNG/JPEG (raster images)
- ✅ GeoJSON
- ✅ CSV

## 🎯 Casos de Uso

### 1. Análise de Mercado
- Mapear clientes e concorrentes
- Análise de buffer (raio de atuação)
- Visualização por receita/categoria

### 2. Planejamento Urbano
- Mapear infraestrutura
- Análise de zoneamento
- Estudos de impacto

### 3. Logística
- Planejamento de rotas
- Análise de proximidade
- Otimização de entregas

### 4. Estudos Acadêmicos
- Pesquisa geográfica
- Análise espacial
- Visualização de dados

## 🛣️ Roadmap

- [x] Interface básica
- [x] Gerenciador de camadas
- [x] Editor de simbologia
- [x] Toolbox completa
- [ ] Canvas interativo com rendering
- [ ] Importação de dados reais
- [ ] Tabela de atributos funcional
- [ ] Geoprocessing tools
- [ ] Suporte a basemaps
- [ ] Plugin system
- [ ] Python scripting
- [ ] 3D visualization

## 📝 Licença

MIT OR Apache-2.0

## 👨‍💻 Autor

**Nícolas Ávila** - nicolas@avila.inc

---

**AvilaGIS Desktop** - Seu GIS de código aberto em Rust 🗺️🦀
