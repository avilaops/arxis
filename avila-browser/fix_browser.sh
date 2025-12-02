#!/bin/bash
cd /d/arxis/avila-browser
cp src/rendering/mod.rs src/rendering/mod.rs.bak

awk '
BEGIN { in_extract_title = 0; skip_lines = 0 }
/pub fn extract_title/ {
    in_extract_title = 1
    print "    pub fn extract_title(&self) -> Option<String> {"
    print "        // Extract title from inner_text of root node"
    print "        match &self.root.node_type {"
    print "            NodeType::Element { inner_text, .. } => {"
    print "                if let Some(start) = inner_text.find(\"<title>\") {"
    print "                    let after_open = &inner_text[start + 7..];"
    print "                    if let Some(end) = after_open.find(\"</title>\") {"
    print "                        return Some(after_open[..end].to_string());"
    print "                    }"
    print "                }"
    print "                None"
    print "            }"
    print "            _ => None,"
    print "        }"
    skip_lines = 2
    next
}
skip_lines > 0 { skip_lines--; next }
{ print }
' src/rendering/mod.rs.bak > src/rendering/mod.rs

echo "Correção aplicada"
