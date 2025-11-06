// File: rhtml-parser/src/function_component.rs
// Purpose: Parse function-based component syntax

use regex::Regex;

/// Represents a struct definition for component props
#[derive(Debug, Clone, PartialEq)]
pub struct PropsStruct {
    pub name: String,
    pub fields: Vec<PropField>,
}

/// Represents a field in a props struct
#[derive(Debug, Clone, PartialEq)]
pub struct PropField {
    pub name: String,
    pub type_name: String,
}

/// Represents a function component definition
#[derive(Debug, Clone)]
pub struct FunctionComponent {
    pub name: String,
    pub props_type: Option<String>,
    pub props_fields: Vec<String>, // Destructured field names
    pub body: String,
}

/// Parser for function-based components
pub struct FunctionComponentParser;

impl FunctionComponentParser {
    /// Check if content contains function-style components
    /// (Components without 'cmp' or 'css' keywords)
    pub fn has_function_components(content: &str) -> bool {
        // Look for pattern: ComponentName(...) {
        // But not: cmp ComponentName, css ComponentName, partial ComponentName
        let re = Regex::new(r"(?m)^\s*[A-Z]\w*\s*\([^)]*\)\s*\{").unwrap();

        for mat in re.find_iter(content) {
            let line_start = content[..mat.start()]
                .rfind('\n')
                .map(|pos| pos + 1)
                .unwrap_or(0);
            let line_prefix = &content[line_start..mat.start()];

            // Check it's not preceded by cmp, css, or partial keywords
            if !line_prefix.trim_end().ends_with("cmp")
                && !line_prefix.trim_end().ends_with("css")
                && !line_prefix.trim_end().ends_with("partial")
            {
                return true;
            }
        }

        false
    }

    /// Extract all struct definitions from content
    /// Matches: struct Name { field: Type, ... }
    pub fn extract_structs(content: &str) -> Vec<PropsStruct> {
        let mut structs = Vec::new();

        // Pattern: struct Name { ... }
        let re = Regex::new(r"struct\s+(\w+)\s*\{").unwrap();

        for cap in re.captures_iter(content) {
            if let Some(name_match) = cap.get(1) {
                let struct_name = name_match.as_str().to_string();
                let struct_start = cap.get(0).unwrap().end();

                // Extract struct body
                if let Some(body) = Self::extract_braced_content(&content[struct_start..]) {
                    let fields = Self::parse_struct_fields(&body);
                    structs.push(PropsStruct {
                        name: struct_name,
                        fields,
                    });
                }
            }
        }

        structs
    }

    /// Parse fields from struct body
    /// Example: "name: String, age: u32" -> [("name", "String"), ("age", "u32")]
    fn parse_struct_fields(body: &str) -> Vec<PropField> {
        let mut fields = Vec::new();

        // Split by commas and parse each field
        for line in body.split(',') {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // Parse: field_name: Type
            if let Some(colon_pos) = line.find(':') {
                let name = line[..colon_pos].trim().to_string();
                let type_name = line[colon_pos + 1..].trim().to_string();

                fields.push(PropField { name, type_name });
            }
        }

        fields
    }

    /// Extract function components from content
    /// Matches: ComponentName(props: PropsType) { ... }
    /// Or: ComponentName(PropsType { field1, field2 }: PropsType) { ... }
    pub fn extract_function_components(content: &str) -> Vec<FunctionComponent> {
        let mut components = Vec::new();

        // Pattern: ComponentName(params) {
        let re = Regex::new(r"([A-Z]\w*)\s*\(([^)]*)\)\s*\{").unwrap();

        for cap in re.captures_iter(content) {
            let full_match = cap.get(0).unwrap();
            let match_start = full_match.start();

            // Check it's not preceded by cmp, css, or partial
            let line_start = content[..match_start]
                .rfind('\n')
                .map(|pos| pos + 1)
                .unwrap_or(0);
            let line_prefix = &content[line_start..match_start];

            if line_prefix.trim().ends_with("cmp")
                || line_prefix.trim().ends_with("css")
                || line_prefix.trim().ends_with("partial")
            {
                continue;
            }

            if let Some(name_match) = cap.get(1) {
                let component_name = name_match.as_str().to_string();
                let params = cap.get(2).map(|m| m.as_str()).unwrap_or("");
                let body_start = full_match.end();

                // Parse parameters to extract props type and fields
                let (props_type, props_fields) = Self::parse_component_params(params);

                // Extract component body
                if let Some(body) = Self::extract_braced_content(&content[body_start..]) {
                    components.push(FunctionComponent {
                        name: component_name,
                        props_type,
                        props_fields,
                        body: body.trim().to_string(),
                    });
                }
            }
        }

        components
    }

    /// Parse component parameters
    /// Examples:
    /// - "props: BadgeProps" -> (Some("BadgeProps"), [])
    /// - "BadgeProps { label, color }: BadgeProps" -> (Some("BadgeProps"), ["label", "color"])
    /// - "" -> (None, [])
    fn parse_component_params(params: &str) -> (Option<String>, Vec<String>) {
        let params = params.trim();

        if params.is_empty() {
            return (None, Vec::new());
        }

        // Check for destructuring: TypeName { field1, field2 }: TypeName
        if params.contains('{') && params.contains('}') {
            // Extract type name (after the colon)
            let props_type = if let Some(colon_pos) = params.rfind(':') {
                Some(params[colon_pos + 1..].trim().to_string())
            } else {
                None
            };

            // Extract fields from { ... }
            let fields = if let Some(start) = params.find('{') {
                if let Some(end) = params.find('}') {
                    params[start + 1..end]
                        .split(',')
                        .map(|f| f.trim().to_string())
                        .filter(|f| !f.is_empty())
                        .collect()
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            };

            (props_type, fields)
        } else {
            // Simple: props: TypeName
            let props_type = if let Some(colon_pos) = params.find(':') {
                Some(params[colon_pos + 1..].trim().to_string())
            } else {
                // Just TypeName without variable name
                Some(params.trim().to_string())
            };

            (props_type, Vec::new())
        }
    }

    /// Extract content within braces with proper nesting
    fn extract_braced_content(content: &str) -> Option<String> {
        let mut depth = 1;
        let mut end_pos = None;

        for (i, ch) in content.chars().enumerate() {
            if ch == '{' {
                depth += 1;
            } else if ch == '}' {
                depth -= 1;
                if depth == 0 {
                    end_pos = Some(i);
                    break;
                }
            }
        }

        end_pos.map(|end| content[..end].trim().to_string())
    }

    /// Remove struct definitions from content
    pub fn remove_structs(content: &str) -> String {
        let mut result = content.to_string();

        loop {
            let re = Regex::new(r"struct\s+\w+\s*\{").unwrap();

            if let Some(mat) = re.find(&result) {
                let start = mat.start();
                let body_start = mat.end();

                if let Some(body) = Self::extract_braced_content(&result[body_start..]) {
                    let end = body_start + body.len() + 1; // +1 for closing brace
                    result = format!("{}{}", &result[..start], &result[end..]);
                    continue;
                }
            }

            break;
        }

        result
    }

    /// Convert function component to cmp-style syntax
    /// This allows the existing renderer to handle it
    pub fn convert_to_cmp_syntax(component: &FunctionComponent) -> String {
        format!("cmp {} {{\n{}\n}}", component.name, component.body)
    }

    /// Process content: convert function components to cmp syntax
    /// This maintains backward compatibility
    pub fn process_content(content: &str) -> String {
        // If no function components, return as-is
        if !Self::has_function_components(content) {
            return content.to_string();
        }

        let mut result = content.to_string();

        // Remove struct definitions (we don't need them at runtime)
        result = Self::remove_structs(&result);

        // Extract function components
        let components = Self::extract_function_components(content);

        // Replace each function component with cmp syntax
        for component in components {
            // Find the original function component in result
            let patterns = vec![
                // Pattern 1: Name(PropsType { field1, field2 }: PropsType) {
                format!(
                    r"{}\s*\([^)]*\{{[^}}]*\}}[^)]*\)\s*\{{",
                    regex::escape(&component.name)
                ),
                // Pattern 2: Name(props: PropsType) {
                format!(
                    r"{}\s*\([^)]*:[^)]*\)\s*\{{",
                    regex::escape(&component.name)
                ),
                // Pattern 3: Name(PropsType) {
                format!(
                    r"{}\s*\([^)]*\)\s*\{{",
                    regex::escape(&component.name)
                ),
            ];

            for pattern in patterns {
                if let Ok(re) = Regex::new(&pattern) {
                    if let Some(mat) = re.find(&result) {
                        let start = mat.start();
                        let body_start = mat.end();

                        // Extract the body
                        if let Some(body) = Self::extract_braced_content(&result[body_start..]) {
                            let end = body_start + body.len() + 1;

                            // Replace with cmp syntax
                            let cmp_syntax = format!("cmp {} {{\n{}\n}}", component.name, body);
                            result = format!("{}{}{}", &result[..start], cmp_syntax, &result[end..]);
                            break;
                        }
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_function_components() {
        let content = r#"
            Badge(BadgeProps { label, color }: BadgeProps) {
                <span>{label}</span>
            }
        "#;

        assert!(FunctionComponentParser::has_function_components(content));
    }

    #[test]
    fn test_extract_structs() {
        let content = r#"
            struct BadgeProps {
                label: String,
                color: String,
            }
        "#;

        let structs = FunctionComponentParser::extract_structs(content);
        assert_eq!(structs.len(), 1);
        assert_eq!(structs[0].name, "BadgeProps");
        assert_eq!(structs[0].fields.len(), 2);
        assert_eq!(structs[0].fields[0].name, "label");
        assert_eq!(structs[0].fields[0].type_name, "String");
    }

    #[test]
    fn test_extract_function_components() {
        let content = r#"
            Badge(BadgeProps { label, color }: BadgeProps) {
                <span class="badge">{label}</span>
            }
        "#;

        let components = FunctionComponentParser::extract_function_components(content);
        assert_eq!(components.len(), 1);
        assert_eq!(components[0].name, "Badge");
        assert_eq!(components[0].props_type, Some("BadgeProps".to_string()));
        assert_eq!(components[0].props_fields, vec!["label", "color"]);
        assert!(components[0].body.contains("<span"));
    }

    #[test]
    fn test_parse_component_params() {
        // Test destructuring
        let (props_type, fields) =
            FunctionComponentParser::parse_component_params("BadgeProps { label, color }: BadgeProps");
        assert_eq!(props_type, Some("BadgeProps".to_string()));
        assert_eq!(fields, vec!["label", "color"]);

        // Test simple props
        let (props_type, fields) =
            FunctionComponentParser::parse_component_params("props: BadgeProps");
        assert_eq!(props_type, Some("BadgeProps".to_string()));
        assert!(fields.is_empty());
    }

    #[test]
    fn test_convert_to_cmp_syntax() {
        let component = FunctionComponent {
            name: "Badge".to_string(),
            props_type: Some("BadgeProps".to_string()),
            props_fields: vec!["label".to_string(), "color".to_string()],
            body: "<span>{label}</span>".to_string(),
        };

        let cmp = FunctionComponentParser::convert_to_cmp_syntax(&component);
        assert!(cmp.contains("cmp Badge"));
        assert!(cmp.contains("<span>{label}</span>"));
    }

    #[test]
    fn test_process_content() {
        let content = r#"
struct BadgeProps {
    label: String,
    color: String,
}

Badge(BadgeProps { label, color }: BadgeProps) {
    <span class="badge bg-{color}-500">{label}</span>
}
        "#;

        let processed = FunctionComponentParser::process_content(content);

        // Should contain cmp syntax
        assert!(processed.contains("cmp Badge"));
        // Should not contain struct
        assert!(!processed.contains("struct BadgeProps"));
        // Should preserve HTML
        assert!(processed.contains("<span class=\"badge"));
    }

    #[test]
    fn test_does_not_affect_cmp_syntax() {
        let content = r#"
            cmp Button {
                <button>Click</button>
            }
        "#;

        let processed = FunctionComponentParser::process_content(content);
        assert_eq!(content, processed);
    }
}
