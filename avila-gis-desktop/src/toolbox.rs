//! Toolbox - GIS Tools

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tool {
    // Navigation
    Pan,
    ZoomIn,
    ZoomOut,

    // Selection
    Select,
    SelectByRectangle,
    SelectByPolygon,
    SelectByCircle,

    // Measurement
    Measure,
    MeasureArea,
    MeasureAngle,

    // Identification
    Identify,

    // Editing
    CreatePoint,
    CreateLine,
    CreatePolygon,
    Edit,
    Delete,

    // Analysis
    Buffer,
    Clip,
    Union,
    Intersect,
}

impl Tool {
    pub fn icon(&self) -> &str {
        match self {
            Tool::Pan => "✋",
            Tool::ZoomIn => "🔍+",
            Tool::ZoomOut => "🔍-",
            Tool::Select => "🖱️",
            Tool::SelectByRectangle => "▭",
            Tool::SelectByPolygon => "⬡",
            Tool::SelectByCircle => "⭕",
            Tool::Measure => "📏",
            Tool::MeasureArea => "📐",
            Tool::MeasureAngle => "📐",
            Tool::Identify => "ℹ️",
            Tool::CreatePoint => "📍",
            Tool::CreateLine => "📏",
            Tool::CreatePolygon => "⬡",
            Tool::Edit => "✏️",
            Tool::Delete => "🗑️",
            Tool::Buffer => "🔄",
            Tool::Clip => "✂️",
            Tool::Union => "⋃",
            Tool::Intersect => "⋂",
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Tool::Pan => "Pan",
            Tool::ZoomIn => "Zoom In",
            Tool::ZoomOut => "Zoom Out",
            Tool::Select => "Select",
            Tool::SelectByRectangle => "Select by Rectangle",
            Tool::SelectByPolygon => "Select by Polygon",
            Tool::SelectByCircle => "Select by Circle",
            Tool::Measure => "Measure Distance",
            Tool::MeasureArea => "Measure Area",
            Tool::MeasureAngle => "Measure Angle",
            Tool::Identify => "Identify",
            Tool::CreatePoint => "Create Point",
            Tool::CreateLine => "Create Line",
            Tool::CreatePolygon => "Create Polygon",
            Tool::Edit => "Edit Feature",
            Tool::Delete => "Delete Feature",
            Tool::Buffer => "Buffer",
            Tool::Clip => "Clip",
            Tool::Union => "Union",
            Tool::Intersect => "Intersect",
        }
    }

    pub fn category(&self) -> ToolCategory {
        match self {
            Tool::Pan | Tool::ZoomIn | Tool::ZoomOut => ToolCategory::Navigation,
            Tool::Select | Tool::SelectByRectangle | Tool::SelectByPolygon | Tool::SelectByCircle => {
                ToolCategory::Selection
            }
            Tool::Measure | Tool::MeasureArea | Tool::MeasureAngle => ToolCategory::Measurement,
            Tool::Identify => ToolCategory::Query,
            Tool::CreatePoint | Tool::CreateLine | Tool::CreatePolygon | Tool::Edit | Tool::Delete => {
                ToolCategory::Editing
            }
            Tool::Buffer | Tool::Clip | Tool::Union | Tool::Intersect => ToolCategory::Analysis,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToolCategory {
    Navigation,
    Selection,
    Measurement,
    Query,
    Editing,
    Analysis,
}

impl ToolCategory {
    pub fn name(&self) -> &str {
        match self {
            ToolCategory::Navigation => "Navigation",
            ToolCategory::Selection => "Selection",
            ToolCategory::Measurement => "Measurement",
            ToolCategory::Query => "Query",
            ToolCategory::Editing => "Editing",
            ToolCategory::Analysis => "Analysis",
        }
    }

    pub fn tools(&self) -> Vec<Tool> {
        Tool::all()
            .into_iter()
            .filter(|t| t.category() == *self)
            .collect()
    }
}

impl Tool {
    pub fn all() -> Vec<Tool> {
        vec![
            Tool::Pan,
            Tool::ZoomIn,
            Tool::ZoomOut,
            Tool::Select,
            Tool::SelectByRectangle,
            Tool::SelectByPolygon,
            Tool::SelectByCircle,
            Tool::Measure,
            Tool::MeasureArea,
            Tool::MeasureAngle,
            Tool::Identify,
            Tool::CreatePoint,
            Tool::CreateLine,
            Tool::CreatePolygon,
            Tool::Edit,
            Tool::Delete,
            Tool::Buffer,
            Tool::Clip,
            Tool::Union,
            Tool::Intersect,
        ]
    }
}
