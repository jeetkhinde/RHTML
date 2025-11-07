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
    pub is_partial: bool, // true if marked with @partial attribute
}

/// Result of processing function-based content
#[derive(Debug, Clone)]
pub struct ProcessedContent {
    pub content: String,
    pub partials: Vec<String>, // Names of components marked as @partial
}

/// Parser for function-based components
pub struct FunctionComponentParser;

impl FunctionComponentParser {
    /// Check if a component name is "WebPage" (case-insensitive)
    /// Supports: webpage, WEBPAGE, WebPage, etc.
    fn is_webpage(name: &str) -> bool {
        name.to_lowercase() == "webpage"
    }

    /// Check if content has @partial attribute
    /// Format: @partial (on its own line or with whitespace)
    pub fn has_partial_attribute(content: &str) -> bool {
        let re = Regex::new(r"(?m)^\s*@partial\s*$").unwrap();
        re.is_match(content)
    }

    /// Check if a specific function component has @partial attribute before it
    /// Returns true if @partial appears before the component definition
    fn is_partial_component(content: &str, component_start: usize) -> bool {
        // Look for @partial in the content before component_start
        let before_component = &content[..component_start];

        // Find the last occurrence of @partial before the component
        let re = Regex::new(r"(?m)^\s*@partial\s*$").unwrap();

        if let Some(mat) = re.find_iter(before_component).last() {
            // Check if there's a struct definition or just whitespace between @partial and component
            let between = &before_component[mat.end()..];

            // If there's only whitespace and possibly a struct/comments, it's for this component
            let lines_between: Vec<&str> = between
                .lines()
                .map(|l| l.trim())
                .filter(|l| !l.is_empty())
                .collect();

            // Check that all non-empty lines are either struct definitions or comments
            let valid = lines_between.is_empty()
                || lines_between.iter().all(|line| {
                    line.starts_with("struct ")
                        || line.starts_with("//")
                        || line.starts_with("}")
                        || line.ends_with("}")
                        || line.ends_with(",")
                });

            valid
        } else {
            false
        }
    }

    /// Remove @partial attributes from content
    pub fn remove_partial_attributes(content: &str) -> String {
        let re = Regex::new(r"(?m)^\s*@partial\s*\n?").unwrap();
        re.replace_all(content, "").to_string()
    }

    /// Check if content has #[webpage] attribute
    /// Format: #[webpage] (on its own line or before function)
    pub fn has_webpage_attribute(content: &str) -> bool {
        content.contains("#[webpage]")
    }

    /// Extract Rust functions with #[webpage] attribute
    /// Parses: #[webpage] pub fn name(props: Type) { <html> }
    pub fn extract_webpage_functions(content: &str) -> Vec<FunctionComponent> {
        let mut components = Vec::new();

        // Pattern: #[webpage] followed by function definition
        let re = Regex::new(r"#\[webpage\]\s+(?:pub\s+)?fn\s+(\w+)\s*\(([^)]*)\)\s*\{").unwrap();

        for cap in re.captures_iter(content) {
            let full_match = cap.get(0).unwrap();
            let _func_name = cap.get(1).unwrap().as_str().to_string();
            let params = cap.get(2).unwrap().as_str();
            let body_start = full_match.end();

            // Extract props type from parameters
            let props_type = Self::parse_webpage_params(params);

            // Extract function body
            if let Some(body) = Self::extract_braced_content(&content[body_start..]) {
                components.push(FunctionComponent {
                    name: "WebPage".to_string(), // Always treated as WebPage
                    props_type,
                    props_fields: Vec::new(),
                    body: body.trim().to_string(),
                    is_partial: false,
                });
            }
        }

        components
    }

    /// Parse parameters from #[webpage] function
    /// Example: "props: UsersProps" -> Some("UsersProps")
    fn parse_webpage_params(params: &str) -> Option<String> {
        let params = params.trim();
        if params.is_empty() {
            return None;
        }

        // Look for: props: TypeName or just TypeName
        if let Some(colon_pos) = params.find(':') {
            Some(params[colon_pos + 1..].trim().to_string())
        } else {
            Some(params.trim().to_string())
        }
    }

    /// Check if content contains function-style components
    /// (Components without 'cmp' or 'css' keywords, or #[webpage] functions)
    pub fn has_function_components(content: &str) -> bool {
        // Check for #[webpage] attribute first
        if Self::has_webpage_attribute(content) {
            return true;
        }

        // Look for pattern: ComponentName(...) {
        // But not: cmp ComponentName, css ComponentName, partial ComponentName
        // Use a more permissive regex that handles nested parentheses
        let re = Regex::new(r"(?m)^\s*[A-Z]\w*\s*\(").unwrap();

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

        // Pattern: ComponentName( - we'll manually find the closing paren
        let re = Regex::new(r"([A-Z]\w*)\s*\(").unwrap();

        for cap in re.captures_iter(content) {
            let full_match = cap.get(0).unwrap();
            let match_start = full_match.start();
            let params_start = full_match.end();

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

                // Find matching closing parenthesis
                let params_and_rest = &content[params_start..];
                let mut depth = 1;
                let mut params_end = None;

                for (i, ch) in params_and_rest.char_indices() {
                    if ch == '(' {
                        depth += 1;
                    } else if ch == ')' {
                        depth -= 1;
                        if depth == 0 {
                            params_end = Some(i);
                            break;
                        }
                    }
                }

                let params_end = match params_end {
                    Some(end) => end,
                    None => continue, // No matching paren found
                };

                let params = &params_and_rest[..params_end];
                let after_params = &params_and_rest[params_end + 1..];

                // Find the opening brace
                let brace_pos = match after_params.trim_start().chars().next() {
                    Some('{') => after_params.find('{').unwrap(),
                    _ => continue, // No opening brace found
                };

                let body_start = params_start + params_end + 1 + brace_pos + 1;

                // Parse parameters to extract props type and fields
                let (props_type, props_fields) = Self::parse_component_params(params);

                // Check if this component has @partial attribute
                let is_partial = Self::is_partial_component(content, match_start);

                // Extract component body
                if let Some(body) = Self::extract_braced_content(&content[body_start..]) {
                    components.push(FunctionComponent {
                        name: component_name,
                        props_type,
                        props_fields,
                        body: body.trim().to_string(),
                        is_partial,
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

    /// Convert function component to standardized syntax
    /// WebPage (case-insensitive) is kept as WebPage (normalized)
    /// Other components keep their original names
    pub fn convert_to_standard_syntax(component: &FunctionComponent) -> String {
        let component_name = if Self::is_webpage(&component.name) {
            "WebPage".to_string()
        } else {
            component.name.clone()
        };

        format!("{} {{\n{}\n}}", component_name, component.body)
    }

    /// Remove #[webpage] attributes from content
    pub fn remove_webpage_attributes(content: &str) -> String {
        let re = Regex::new(r"#\[webpage\]\s+(?:pub\s+)?fn\s+\w+\s*\([^)]*\)\s*").unwrap();
        re.replace_all(content, "").to_string()
    }

    /// Process content: convert function components to standard syntax
    /// Returns processed content and list of partials
    pub fn process_content(content: &str) -> ProcessedContent {
        // If no function components, return as-is
        if !Self::has_function_components(content) {
            return ProcessedContent {
                content: content.to_string(),
                partials: Vec::new(),
            };
        }

        let mut result = content.to_string();
        let mut all_components = Vec::new();

        // Check for #[webpage] syntax first
        if Self::has_webpage_attribute(content) {
            let webpage_components = Self::extract_webpage_functions(content);
            all_components.extend(webpage_components);

            // Remove #[webpage] function definitions from result
            // We'll replace with WebPage { body } format
            let re = Regex::new(r"#\[webpage\]\s+(?:pub\s+)?fn\s+\w+\s*\([^)]*\)\s*\{").unwrap();
            for mat in re.find_iter(&result.clone()) {
                let start = mat.start();
                let body_start = mat.end();

                // Find matching closing brace
                if let Some(body) = Self::extract_braced_content(&result[body_start..]) {
                    let end = body_start + body.len() + 1;

                    // Replace the entire #[webpage] function with just WebPage { body }
                    let replacement = format!("WebPage {{\n{}\n}}", body.trim());
                    result = format!("{}{}{}", &result[..start], replacement, &result[end..]);
                    break; // Process one at a time
                }
            }
        } else {
            // Extract traditional function components BEFORE removing structs and @partial
            all_components.extend(Self::extract_function_components(content));
        }

        // Track which components are partials
        let partials: Vec<String> = all_components
            .iter()
            .filter(|c| c.is_partial)
            .map(|c| c.name.clone())
            .collect();

        // Remove @partial attributes
        result = Self::remove_partial_attributes(&result);

        // Remove struct definitions (we don't need them at runtime)
        result = Self::remove_structs(&result);

        // Replace each function component with standard syntax (skip if already processed #[webpage])
        if !Self::has_webpage_attribute(content) {
            for component in all_components {
            // Find the original function component in result using a simple pattern
            // Pattern: Name( - then we'll manually find the closing paren and brace
            let search_pattern = format!(r"{}\s*\(", regex::escape(&component.name));

            if let Ok(re) = Regex::new(&search_pattern) {
                if let Some(mat) = re.find(&result) {
                    let start = mat.start();
                    let params_start = mat.end();

                    // Find matching closing parenthesis
                    let after_start = &result[params_start..];
                    let mut depth = 1;
                    let mut params_end = None;

                    for (i, ch) in after_start.char_indices() {
                        if ch == '(' {
                            depth += 1;
                        } else if ch == ')' {
                            depth -= 1;
                            if depth == 0 {
                                params_end = Some(i);
                                break;
                            }
                        }
                    }

                    if let Some(params_end) = params_end {
                        let after_params = &after_start[params_end + 1..];

                        // Find the opening brace
                        if let Some(brace_pos) = after_params.find('{') {
                            let body_start = params_start + params_end + 1 + brace_pos + 1;

                            // Extract the body
                            if let Some(body) = Self::extract_braced_content(&result[body_start..]) {
                                let end = body_start + body.len() + 1;

                                // Create a temporary component with the extracted body
                                let temp_component = FunctionComponent {
                                    name: component.name.clone(),
                                    props_type: component.props_type.clone(),
                                    props_fields: component.props_fields.clone(),
                                    body: body.to_string(),
                                    is_partial: component.is_partial,
                                };

                                // Replace with standard syntax (normalizes WebPage case)
                                let standard_syntax = Self::convert_to_standard_syntax(&temp_component);
                                result = format!("{}{}{}", &result[..start], standard_syntax, &result[end..]);
                            }
                        }
                    }
                }
            }
            }
        }

        ProcessedContent {
            content: result,
            partials,
        }
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
    fn test_convert_to_standard_syntax() {
        let component = FunctionComponent {
            name: "Badge".to_string(),
            props_type: Some("BadgeProps".to_string()),
            props_fields: vec!["label".to_string(), "color".to_string()],
            body: "<span>{label}</span>".to_string(),
            is_partial: false,
        };

        let standard = FunctionComponentParser::convert_to_standard_syntax(&component);
        assert!(standard.contains("Badge {"));
        assert!(standard.contains("<span>{label}</span>"));
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

        // Should contain standard syntax
        assert!(processed.content.contains("Badge {"));
        // Should not contain struct
        assert!(!processed.content.contains("struct BadgeProps"));
        // Should preserve HTML
        assert!(processed.content.contains("<span class=\"badge"));
        // Should have no partials
        assert!(processed.partials.is_empty());
    }

    #[test]
    fn test_does_not_affect_cmp_syntax() {
        let content = r#"
            cmp Button {
                <button>Click</button>
            }
        "#;

        let processed = FunctionComponentParser::process_content(content);
        assert_eq!(content, processed.content);
        assert!(processed.partials.is_empty());
    }

    #[test]
    fn test_partial_attribute_detection() {
        let content = r#"
@partial
struct BadgeProps {
    label: String,
}

Badge(BadgeProps { label }: BadgeProps) {
    <span>{label}</span>
}
        "#;

        assert!(FunctionComponentParser::has_partial_attribute(content));
        let components = FunctionComponentParser::extract_function_components(content);
        assert_eq!(components.len(), 1);
        assert!(components[0].is_partial);
        assert_eq!(components[0].name, "Badge");
    }

    #[test]
    fn test_process_content_with_partial() {
        let content = r#"
@partial
struct BadgeProps {
    label: String,
    color: String,
}

Badge(BadgeProps { label, color }: BadgeProps) {
    <span class="badge bg-{color}-500">{label}</span>
}
        "#;

        let processed = FunctionComponentParser::process_content(content);

        // Should contain standard syntax
        assert!(processed.content.contains("Badge {"));
        // Should not contain @partial attribute
        assert!(!processed.content.contains("@partial"));
        // Should not contain struct
        assert!(!processed.content.contains("struct BadgeProps"));
        // Should preserve HTML
        assert!(processed.content.contains("<span class=\"badge"));
        // Should have Badge as a partial
        assert_eq!(processed.partials.len(), 1);
        assert_eq!(processed.partials[0], "Badge");
    }

    #[test]
    fn test_remove_partial_attributes() {
        let content = r#"
@partial
Badge() {
    <span>Test</span>
}
        "#;

        let cleaned = FunctionComponentParser::remove_partial_attributes(content);
        assert!(!cleaned.contains("@partial"));
        assert!(cleaned.contains("Badge()"));
    }

    #[test]
    fn test_is_webpage() {
        assert!(FunctionComponentParser::is_webpage("WebPage"));
        assert!(FunctionComponentParser::is_webpage("webpage"));
        assert!(FunctionComponentParser::is_webpage("WEBPAGE"));
        assert!(FunctionComponentParser::is_webpage("wEbPaGe"));
        assert!(!FunctionComponentParser::is_webpage("Page"));
        assert!(!FunctionComponentParser::is_webpage("Component"));
    }

    #[test]
    fn test_webpage_conversion() {
        let component = FunctionComponent {
            name: "WebPage".to_string(),
            props_type: Some("PageProps<()>".to_string()),
            props_fields: vec![],
            body: "<div>Content</div>".to_string(),
            is_partial: false,
        };

        let standard = FunctionComponentParser::convert_to_standard_syntax(&component);
        assert!(standard.contains("WebPage {"));
        assert!(standard.contains("<div>Content</div>"));
    }

    #[test]
    fn test_webpage_case_insensitive() {
        // Test different cases - all should normalize to "WebPage"
        for webpage_name in &["WebPage", "webpage", "WEBPAGE", "wEbPaGe"] {
            let component = FunctionComponent {
                name: webpage_name.to_string(),
                props_type: None,
                props_fields: vec![],
                body: "<div>Test</div>".to_string(),
                is_partial: false,
            };

            let standard = FunctionComponentParser::convert_to_standard_syntax(&component);
            assert!(
                standard.contains("WebPage {"),
                "Failed for case: {} - got: {}",
                webpage_name,
                standard
            );
        }
    }

    #[test]
    fn test_webpage_full_example() {
        let content = r#"
WebPage(props: &PageProps<()>) {
    <div class="container">
        <h1>Users Directory</h1>
        <p>Browse all users</p>
    </div>
}
        "#;

        // Debug: check if it detects function components
        println!("Has function components: {}", FunctionComponentParser::has_function_components(content));

        let components = FunctionComponentParser::extract_function_components(content);
        println!("Extracted {} components", components.len());
        for comp in &components {
            println!("  Component: {}", comp.name);
        }

        let processed = FunctionComponentParser::process_content(content);

        // Debug: print the processed content
        println!("Processed content:\n{}", processed.content);

        // Should contain WebPage { (normalized, no cmp)
        assert!(
            processed.content.contains("WebPage {"),
            "Content does not contain 'WebPage {{': {}",
            processed.content
        );
        // Should preserve HTML
        assert!(processed.content.contains("Users Directory"));
    }

    #[test]
    fn test_webpage_attribute_detection() {
        let content = r#"
#[webpage]
pub fn users(props: UsersProps) {
    <div>Users</div>
}
        "#;

        assert!(FunctionComponentParser::has_webpage_attribute(content));
        assert!(FunctionComponentParser::has_function_components(content));
    }

    #[test]
    fn test_extract_webpage_functions() {
        let content = r#"
#[webpage]
pub fn users(props: UsersProps) {
    <div class="container">
        <h1>Users</h1>
        <div r-for="user in props.data">
            <user_card user={user} />
        </div>
    </div>
}
        "#;

        let components = FunctionComponentParser::extract_webpage_functions(content);
        assert_eq!(components.len(), 1);
        assert_eq!(components[0].name, "WebPage");
        assert_eq!(components[0].props_type, Some("UsersProps".to_string()));
        assert!(components[0].body.contains("<h1>Users</h1>"));
    }

    #[test]
    fn test_process_webpage_attribute() {
        let content = r#"
slots {
    title: "Users",
}

#[webpage]
pub fn users(props: UsersProps) {
    <div class="container">
        <h1>Users</h1>
        <div r-for="user in props.data">
            <user_card user={user} />
        </div>
    </div>
}
        "#;

        let processed = FunctionComponentParser::process_content(content);

        // Should contain WebPage {
        assert!(
            processed.content.contains("WebPage {"),
            "Content does not contain 'WebPage {{': {}",
            processed.content
        );

        // Should preserve HTML
        assert!(processed.content.contains("<h1>Users</h1>"));
        assert!(processed.content.contains("r-for="));

        // Should not contain #[webpage] anymore
        assert!(!processed.content.contains("#[webpage]"));
        assert!(!processed.content.contains("pub fn users"));
    }

    #[test]
    fn test_webpage_attribute_without_pub() {
        let content = r#"
#[webpage]
fn home(props: PageProps) {
    <div>Home</div>
}
        "#;

        let components = FunctionComponentParser::extract_webpage_functions(content);
        assert_eq!(components.len(), 1);
        assert_eq!(components[0].name, "WebPage");
    }
}
