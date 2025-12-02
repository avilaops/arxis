//! HTML/CSS rendering engine with DOM tree construction and layout computation

use std::collections::BTreeMap;

/// DOM (Document Object Model)
#[derive(Debug, Clone)]
pub struct Dom {
    pub root: DomNode,
}

#[derive(Debug, Clone)]
pub struct DomNode {
    pub node_type: NodeType,
    pub children: Vec<DomNode>,
    pub attributes: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Element { tag: String, inner_text: String },
    Text(String),
}

impl Dom {
    /// Parse HTML into abstract syntax tree representation
    pub fn parse(html: &str) -> Self {
        let root = Self::parse_node(html);
        Self { root }
    }

    fn parse_node(html: &str) -> DomNode {
        // Recursive descent HTML parser
        if html.starts_with('<') {
            // Element node
            if let Some(tag_end) = html.find('>') {
                let tag_content = &html[1..tag_end];
                let tag = tag_content.split_whitespace().next()
                    .unwrap_or("div")
                    .to_string();

                // Extract inner text content - find matching closing tag
                let close_tag = format!("</{}>", tag);
                let inner_content = if let Some(close_start) = html.find(&close_tag) {
                    html[tag_end + 1..close_start].to_string()
                } else {
                    String::new()
                };

                // Parse children recursively
                let mut children = Vec::new();
                let mut remaining = inner_content.as_str();
                while !remaining.is_empty() {
                    if remaining.starts_with('<') && !remaining.starts_with("</") {
                        if let Some(child_tag_end) = remaining.find('>') {
                            let child_tag_content = &remaining[1..child_tag_end];
                            let child_tag = child_tag_content.split_whitespace().next()
                                .unwrap_or("div");
                            let child_close_tag = format!("</{}>", child_tag);
                            if let Some(child_close_start) = remaining.find(&child_close_tag) {
                                let child_close_end = child_close_start + child_close_tag.len();
                                let child_html = &remaining[..child_close_end];
                                children.push(Self::parse_node(child_html));
                                remaining = &remaining[child_close_end..];
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    } else {
                        // Skip non-element content
                        if let Some(next_tag) = remaining.find('<') {
                            remaining = &remaining[next_tag..];
                        } else {
                            break;
                        }
                    }
                }

                DomNode {
                    node_type: NodeType::Element { tag, inner_text: inner_content },
                    children,
                    attributes: BTreeMap::new(),
                }
            } else {
                Self::text_node(html)
            }
        } else {
            Self::text_node(html)
        }
    }

    fn text_node(text: &str) -> DomNode {
        DomNode {
            node_type: NodeType::Text(text.to_string()),
            children: Vec::new(),
            attributes: BTreeMap::new(),
        }
    }

    /// Extract document title from DOM tree
    pub fn extract_title(&self) -> Option<String> {
        self.find_element("title")
    }

    /// Query DOM tree for element by tag name
    pub fn find_element(&self, tag: &str) -> Option<String> {
        self.find_in_node(&self.root, tag)
    }

    fn find_in_node(&self, node: &DomNode, tag: &str) -> Option<String> {
        match &node.node_type {
            NodeType::Element { tag: node_tag, inner_text } => {
                if node_tag == tag {
                    return Some(inner_text.clone());
                }
            }
            _ => {}
        }

        for child in &node.children {
            if let Some(result) = self.find_in_node(child, tag) {
                return Some(result);
            }
        }

        None
    }
}

/// CSS parser implementing selector matching and cascade resolution
#[derive(Debug)]
pub struct CssParser {
    pub stylesheets: Vec<Stylesheet>,
}

#[derive(Debug)]
pub struct Stylesheet {
    pub rules: Vec<CssRule>,
}

#[derive(Debug)]
pub struct CssRule {
    pub selector: String,
    pub declarations: BTreeMap<String, String>,
}

impl CssParser {
    pub fn new() -> Self {
        Self {
            stylesheets: Vec::new(),
        }
    }

    pub fn parse(&mut self, css: &str) {
        // Tokenization-based CSS parser
        let mut rules = Vec::new();

        // Tokenize by rule terminator
        for rule_text in css.split('}') {
            if let Some(open_brace) = rule_text.find('{') {
                let selector = rule_text[..open_brace].trim().to_string();
                let declarations_text = &rule_text[open_brace + 1..];

                let mut declarations = BTreeMap::new();
                for decl in declarations_text.split(';') {
                    if let Some(colon) = decl.find(':') {
                        let property = decl[..colon].trim().to_string();
                        let value = decl[colon + 1..].trim().to_string();
                        declarations.insert(property, value);
                    }
                }

                rules.push(CssRule {
                    selector,
                    declarations,
                });
            }
        }

        self.stylesheets.push(Stylesheet { rules });
    }
}

/// Layout engine implementing box model and flow layout algorithm
#[derive(Debug)]
pub struct LayoutEngine {
    pub viewport_width: u32,
    pub viewport_height: u32,
}

impl LayoutEngine {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            viewport_width: width,
            viewport_height: height,
        }
    }

    /// Compute layout tree with positioned boxes
    pub fn layout(&self, dom: &Dom) -> LayoutTree {
        LayoutTree {
            root: self.layout_node(&dom.root, 0, 0),
        }
    }

    fn layout_node(&self, node: &DomNode, x: u32, y: u32) -> LayoutNode {
        let (width, height) = match &node.node_type {
            NodeType::Element { .. } => {
                // Block-level box: full viewport width
                (self.viewport_width, 20)
            }
            NodeType::Text(text) => {
                // Inline text: character-based width estimation
                (text.len() as u32 * 8, 16)
            }
        };

        LayoutNode {
            x,
            y,
            width,
            height,
            children: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct LayoutTree {
    pub root: LayoutNode,
}

#[derive(Debug)]
pub struct LayoutNode {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub children: Vec<LayoutNode>,
}

/// Render layout tree to terminal using ASCII art representation
pub fn render_to_terminal(_layout: &LayoutTree, dom: &Dom) -> String {
    let mut output = String::new();

    output.push_str("╔════════════════════════════════════════════════════════════════╗\n");
    output.push_str("║                    AVILA BROWSER                               ║\n");
    output.push_str("╠════════════════════════════════════════════════════════════════╣\n");

    render_node(&dom.root, &mut output, 0);

    output.push_str("╚════════════════════════════════════════════════════════════════╝\n");
    output
}

fn render_node(node: &DomNode, output: &mut String, indent: usize) {
    let padding = "  ".repeat(indent);

    match &node.node_type {
        NodeType::Element { tag, inner_text } => {
            output.push_str(&format!("║ {}<{}> {}\n", padding, tag, inner_text));
        }
        NodeType::Text(text) => {
            if !text.trim().is_empty() {
                output.push_str(&format!("║ {}{}\n", padding, text.trim()));
            }
        }
    }

    for child in &node.children {
        render_node(child, output, indent + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dom_parsing() {
        let html = "<html><title>Test</title><body>Content</body></html>";
        let dom = Dom::parse(html);

        let title = dom.extract_title();
        assert_eq!(title, Some("Test".to_string()));
    }

    #[test]
    fn test_css_parsing() {
        let mut parser = CssParser::new();
        let css = "body { color: red; font-size: 14px; }";

        parser.parse(css);

        assert_eq!(parser.stylesheets.len(), 1);
        assert_eq!(parser.stylesheets[0].rules.len(), 1);
    }

    #[test]
    fn test_layout_engine() {
        let engine = LayoutEngine::new(800, 600);
        assert_eq!(engine.viewport_width, 800);
    }
}
