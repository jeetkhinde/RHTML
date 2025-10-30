// File: src/parser/directive.rs
// Purpose: Parse and identify RHTML directives (r-if, r-else, etc.)

use regex::Regex;

/// Represents a parsed directive
#[derive(Debug, Clone, PartialEq)]
pub enum Directive {
    If(String),     // r-if="condition"
    ElseIf(String), // r-else-if="condition"
    Else,           // r-else
    For {           // r-for="item in items" or r-for="(index, item) in items"
        item_var: String,
        index_var: Option<String>,
        collection: String,
    },
}

/// Parser for RHTML directives
pub struct DirectiveParser;

impl DirectiveParser {
    /// Check if an HTML tag has an r-if directive
    pub fn has_if_directive(tag: &str) -> bool {
        tag.contains("r-if=")
    }

    /// Check if an HTML tag has an r-else-if directive
    pub fn has_else_if_directive(tag: &str) -> bool {
        tag.contains("r-else-if=")
    }

    /// Check if an HTML tag has an r-else directive
    pub fn has_else_directive(tag: &str) -> bool {
        tag.contains("r-else") && !tag.contains("r-else-if")
    }

    /// Check if an HTML tag has an r-for directive
    pub fn has_for_directive(tag: &str) -> bool {
        tag.contains("r-for=")
    }

    /// Extract r-if condition from a tag
    pub fn extract_if_condition(tag: &str) -> Option<String> {
        Self::extract_directive_value(tag, "r-if")
    }

    /// Extract r-else-if condition from a tag
    pub fn extract_else_if_condition(tag: &str) -> Option<String> {
        Self::extract_directive_value(tag, "r-else-if")
    }

    /// Extract r-for loop information from a tag
    /// Supports: "item in items" or "(index, item) in items"
    pub fn extract_for_loop(tag: &str) -> Option<(String, Option<String>, String)> {
        let value = Self::extract_directive_value(tag, "r-for")?;

        // Split by " in "
        let parts: Vec<&str> = value.split(" in ").collect();
        if parts.len() != 2 {
            return None;
        }

        let left = parts[0].trim();
        let collection = parts[1].trim().to_string();

        // Check if it's "(index, item)" format
        if left.starts_with('(') && left.ends_with(')') {
            // Parse (index, item)
            let inner = &left[1..left.len() - 1];
            let vars: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();

            if vars.len() == 2 {
                return Some((
                    vars[1].to_string(), // item
                    Some(vars[0].to_string()), // index
                    collection,
                ));
            }
        }

        // Simple "item in items" format
        Some((left.to_string(), None, collection))
    }

    /// Extract directive value using regex
    fn extract_directive_value(tag: &str, directive: &str) -> Option<String> {
        // Match: r-if="condition" or r-if='condition'
        let pattern = format!(r#"{}=["']([^"']+)["']"#, directive);
        let re = Regex::new(&pattern).ok()?;

        re.captures(tag)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
    }

    /// Remove directive attributes from a tag
    pub fn remove_directives(tag: &str) -> String {
        let mut cleaned = tag.to_string();

        // Remove r-if, r-else-if, r-else, r-for attributes
        let patterns = [
            r#"r-if=["'][^"']*["']"#,
            r#"r-else-if=["'][^"']*["']"#,
            r#"r-for=["'][^"']*["']"#,
            r#"r-else\s*"#,
            r#"r-else="#,
        ];

        for pattern in patterns {
            if let Ok(re) = Regex::new(pattern) {
                cleaned = re.replace_all(&cleaned, "").to_string();
            }
        }

        // Clean up extra spaces
        cleaned = cleaned.trim().to_string();
        cleaned = cleaned.replace("  ", " ");

        cleaned
    }

    /// Parse all directives from a tag
    pub fn parse_directives(tag: &str) -> Vec<Directive> {
        let mut directives = Vec::new();

        if Self::has_if_directive(tag) {
            if let Some(condition) = Self::extract_if_condition(tag) {
                directives.push(Directive::If(condition));
            }
        }

        if Self::has_else_if_directive(tag) {
            if let Some(condition) = Self::extract_else_if_condition(tag) {
                directives.push(Directive::ElseIf(condition));
            }
        }

        if Self::has_else_directive(tag) {
            directives.push(Directive::Else);
        }

        if Self::has_for_directive(tag) {
            if let Some((item_var, index_var, collection)) = Self::extract_for_loop(tag) {
                directives.push(Directive::For {
                    item_var,
                    index_var,
                    collection,
                });
            }
        }

        directives
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_if_condition() {
        let tag = r#"<div r-if="user.is_active" class="active">"#;
        assert_eq!(
            DirectiveParser::extract_if_condition(tag),
            Some("user.is_active".to_string())
        );
    }

    #[test]
    fn test_remove_directives() {
        let tag = r#"<div r-if="true" class="test">"#;
        let cleaned = DirectiveParser::remove_directives(tag);
        assert!(!cleaned.contains("r-if"));
        assert!(cleaned.contains("class=\"test\""));
    }

    #[test]
    fn test_extract_for_loop() {
        let tag = r#"<div r-for="item in items">"#;
        let result = DirectiveParser::extract_for_loop(tag);
        assert_eq!(
            result,
            Some(("item".to_string(), None, "items".to_string()))
        );

        let tag_with_index = r#"<div r-for="(i, item) in items">"#;
        let result_with_index = DirectiveParser::extract_for_loop(tag_with_index);
        assert_eq!(
            result_with_index,
            Some(("item".to_string(), Some("i".to_string()), "items".to_string()))
        );
    }
}
