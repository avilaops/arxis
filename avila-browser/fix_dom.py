import sys

# Read file
with open('src/rendering/mod.rs', encoding='utf-8') as f:
    lines = f.readlines()

# Find extract_title function
idx = next((i for i, l in enumerate(lines) if 'fn extract_title' in l), None)
if idx is None:
    sys.exit(1)

# New implementation
new = [
    '    pub fn extract_title(&self) -> Option<String> {\n',
    '        // Extract title directly from root node inner_text\n',
    '        if let NodeType::Element { inner_text, .. } = &self.root.node_type {\n',
    '            if let Some(start) = inner_text.find(\"<title>\") {\n',
    '                if let Some(end) = inner_text[start..].find(\"</title>\") {\n',
    '                    return Some(inner_text[start + 7..start + end].to_string());\n',
    '                }\n',
    '            }\n',
    '        }\n',
    '        None\n',
    '    }\n'
]

# Find end of function
end = next((i for i in range(idx + 1, len(lines)) if lines[i].strip() == '}'), idx) + 1

# Write file
with open('src/rendering/mod.rs', 'w', encoding='utf-8') as f:
    f.writelines(lines[:idx] + new + lines[end:])

print('OK')
