# avila-metadata-extractor

**BIM Metadata Extraction - 100% Rust**

Extract semantic BIM properties from IFC elements to JSON.

## Features

- **Element metadata**: Type, GUID, properties, quantities
- **Spatial structure**: Project → Buildings → Storeys hierarchy
- **Statistics**: Aggregated counts, areas, volumes
- **JSON export**: Clean, structured output for web apps

## Example

```rust
use avila_metadata_extractor::{MetadataExtractor, BimElement};

// Create extractor
let extractor = MetadataExtractor::new();

// Extract from element
let element = BimElement {
    id: "wall_001".into(),
    ifc_type: "IfcWall".into(),
    name: Some("Exterior Wall".into()),
    properties: vec![
        ("Height".into(), "3.0".into()),
        ("Material".into(), "Concrete".into()),
    ],
};

let metadata = extractor.extract_element(&element);

// Export to JSON
let json = serde_json::to_string_pretty(&metadata)?;
std::fs::write("metadata.json", json)?;
```

## Output Structure

```json
{
  "elements": [
    {
      "id": "wall_001",
      "type": "IfcWall",
      "properties": {
        "Height": "3.0",
        "Material": "Concrete"
      },
      "boundingBox": {
        "min": [0, 0, 0],
        "max": [10, 0.2, 3]
      }
    }
  ],
  "spatialStructure": {
    "project": "My Building",
    "buildings": [...]
  },
  "statistics": {
    "elementCounts": { "IfcWall": 42 },
    "totalArea": 1250.5
  }
}
```

## License

MIT OR Apache-2.0
