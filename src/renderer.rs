// File: src/renderer.rs
// Purpose: Render RHTML templates with directive support

use crate::parser::{DirectiveParser, ExpressionEvaluator};
use anyhow::Result;
use regex::Regex;

/// HTML renderer with directive support
pub struct Renderer {
    evaluator: ExpressionEvaluator,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            evaluator: ExpressionEvaluator::new(),
        }
    }

    /// Set a variable for expression evaluation
    pub fn set_var(&mut self, name: impl Into<String>, value: crate::parser::expression::Value) {
        self.evaluator.set(name, value);
    }

    /// Render a template to HTML
    pub fn render(&self, template_content: &str) -> Result<String> {
        let html = self.extract_html(template_content);
        let processed = self.process_directives(&html);
        let interpolated = self.process_interpolations(&processed);
        Ok(interpolated)
    }

    /// Extract HTML content from RHTML template
    /// This needs to extract ONLY the cmp function content, not slots block
    fn extract_html(&self, content: &str) -> String {
        // First, skip past any slots block if it exists
        let search_start = if let Some(slots_pos) = content.find("slots {") {
            // Find the end of slots block
            let mut depth = 0;
            let mut found_opening = false;
            let mut slots_end = slots_pos;

            for (i, ch) in content[slots_pos..].chars().enumerate() {
                if ch == '{' {
                    depth += 1;
                    found_opening = true;
                } else if ch == '}' {
                    depth -= 1;
                    if found_opening && depth == 0 {
                        slots_end = slots_pos + i + 1;
                        break;
                    }
                }
            }
            slots_end
        } else {
            0
        };

        // Now find "cmp" keyword after the slots block
        if let Some(cmp_pos) = content[search_start..].find("cmp ") {
            let abs_cmp_pos = search_start + cmp_pos;
            // Find the opening brace after cmp
            if let Some(start) = content[abs_cmp_pos..].find('{') {
                let abs_start = abs_cmp_pos + start;

                // Find matching closing brace
                let mut depth = 0;
                let mut end_pos = None;

                for (i, ch) in content[abs_start..].chars().enumerate() {
                    if ch == '{' {
                        depth += 1;
                    } else if ch == '}' {
                        depth -= 1;
                        if depth == 0 {
                            end_pos = Some(abs_start + i);
                            break;
                        }
                    }
                }

                if let Some(end) = end_pos {
                    let html = &content[abs_start + 1..end];
                    return html.trim().to_string();
                }
            }
        }

        content.to_string()
    }

    /// Extract slot values from page template
    fn extract_slots(&self, page_content: &str) -> std::collections::HashMap<String, String> {
        let mut slots = std::collections::HashMap::new();

        // Look for slots { ... } block
        if let Some(slots_start) = page_content.find("slots {") {
            // Find matching closing brace
            let mut depth = 0;
            let mut found_opening = false;
            let mut end_pos = None;

            for (i, ch) in page_content[slots_start..].chars().enumerate() {
                if ch == '{' {
                    depth += 1;
                    found_opening = true;
                } else if ch == '}' {
                    depth -= 1;
                    if found_opening && depth == 0 {
                        end_pos = Some(slots_start + i);
                        break;
                    }
                }
            }

            if let Some(end) = end_pos {
                let slots_block = &page_content[slots_start + 7..end]; // Skip "slots {"

                // Parse each slot line: title: "value",
                for line in slots_block.lines() {
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    }

                    // Parse: key: "value" or key: "value",
                    if let Some(colon_pos) = line.find(':') {
                        let key = line[..colon_pos].trim();
                        let value_part = line[colon_pos + 1..].trim().trim_end_matches(',');

                        // Remove quotes
                        let value = value_part.trim_matches('"');

                        slots.insert(key.to_string(), value.to_string());
                    }
                }
            }
        }

        slots
    }

    /// Process r-if, r-else-if, r-else directives
    fn process_directives(&self, html: &str) -> String {
        let mut result = String::new();
        let mut chars = html.chars().peekable();
        let mut buffer = String::new();

        while let Some(ch) = chars.next() {
            buffer.push(ch);

            // Look for opening tags
            if ch == '<' && chars.peek() != Some(&'/') && chars.peek() != Some(&'!') {
                // Read until we find the end of the tag
                let tag_start = buffer.len() - 1;
                while let Some(&next_ch) = chars.peek() {
                    buffer.push(chars.next().unwrap());
                    if next_ch == '>' {
                        break;
                    }
                }

                let tag = &buffer[tag_start..];

                // Check if this tag has conditional directives
                if DirectiveParser::has_if_directive(tag)
                    || DirectiveParser::has_else_if_directive(tag)
                    || DirectiveParser::has_else_directive(tag)
                {
                    // Extract the element (tag + content + closing tag)
                    let (element, _consumed) = self.extract_element(tag, &mut chars);

                    // Process the conditional
                    let processed = self.process_conditional(&element);

                    // Remove the tag from buffer and add processed result
                    buffer.truncate(tag_start);
                    result.push_str(&buffer);
                    result.push_str(&processed);
                    buffer.clear();
                    continue;
                }
            }
        }

        result.push_str(&buffer);
        result
    }

    /// Extract a complete HTML element (opening tag, content, closing tag)
    fn extract_element(
        &self,
        opening_tag: &str,
        chars: &mut std::iter::Peekable<std::str::Chars>,
    ) -> (String, usize) {
        let mut element = opening_tag.to_string();
        let mut consumed = 0;

        // Get tag name
        let tag_name = self.get_tag_name(opening_tag);

        // If self-closing, return immediately
        if opening_tag.trim_end().ends_with("/>") {
            return (element, consumed);
        }

        // Read content until closing tag
        let mut depth = 1;

        while let Some(ch) = chars.next() {
            consumed += 1;
            element.push(ch);

            // Check for tags
            if ch == '<' {
                let mut tag_buffer = String::from('<');
                while let Some(&next_ch) = chars.peek() {
                    chars.next();
                    consumed += 1;
                    tag_buffer.push(next_ch);
                    element.push(next_ch);
                    if next_ch == '>' {
                        break;
                    }
                }

                // Check if opening or closing tag
                if tag_buffer.starts_with("</") {
                    let closing_name = self.get_tag_name(&tag_buffer);
                    if closing_name == tag_name {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                    }
                } else if !tag_buffer.ends_with("/>") && !tag_buffer.starts_with("<!") {
                    let opening_name = self.get_tag_name(&tag_buffer);
                    if opening_name == tag_name {
                        depth += 1;
                    }
                }
            }
        }

        (element, consumed)
    }

    /// Get tag name from an HTML tag
    fn get_tag_name(&self, tag: &str) -> String {
        let tag = tag.trim_start_matches('<').trim_start_matches('/');
        tag.split_whitespace()
            .next()
            .unwrap_or("")
            .trim_end_matches('>')
            .to_string()
    }

    /// Process a conditional element (r-if, r-else-if, r-else)
    fn process_conditional(&self, element: &str) -> String {
        // Extract opening tag
        let tag_end = element.find('>').unwrap_or(element.len());
        let opening_tag = &element[..=tag_end];

        // Determine which directive it has
        let should_render = if DirectiveParser::has_if_directive(opening_tag) {
            if let Some(condition) = DirectiveParser::extract_if_condition(opening_tag) {
                self.evaluator.eval_bool(&condition)
            } else {
                false
            }
        } else if DirectiveParser::has_else_if_directive(opening_tag) {
            if let Some(condition) = DirectiveParser::extract_else_if_condition(opening_tag) {
                self.evaluator.eval_bool(&condition)
            } else {
                false
            }
        } else if DirectiveParser::has_else_directive(opening_tag) {
            true // r-else always renders (we'll handle chaining later)
        } else {
            false
        };

        if should_render {
            // Remove directive and render content
            let cleaned_tag = DirectiveParser::remove_directives(opening_tag);
            element.replacen(opening_tag, &cleaned_tag, 1)
        } else {
            // Don't render
            String::new()
        }
    }

    /// Process {expression} interpolations
    fn process_interpolations(&self, html: &str) -> String {
        let re = Regex::new(r"\{([^}]+)\}").unwrap();

        re.replace_all(html, |caps: &regex::Captures| {
            let expr = &caps[1];
            self.evaluator.eval_string(expr)
        })
        .to_string()
    }

    /// Render page with layout
    pub fn render_with_layout(&self, layout_content: &str, page_content: &str) -> Result<String> {
        // Extract slots from page (before rendering)
        let slots = self.extract_slots(page_content);

        // Extract and process layout HTML WITHOUT interpolations yet
        let layout_html_raw = self.extract_html(layout_content);
        let layout_processed = self.process_directives(&layout_html_raw);

        // Render page HTML fully (with interpolations)
        let page_html = self.render(page_content)?;

        // Replace {slots.content} with page HTML
        let mut result = layout_processed.replace("{slots.content}", &page_html);

        // Replace slot placeholders
        // Pattern 1: {slots.get("key").unwrap_or("default")}
        let slot_pattern =
            Regex::new(r#"\{slots\.get\("([^"]+)"\)\.unwrap_or\("([^"]*)"\)\}"#).unwrap();
        result = slot_pattern
            .replace_all(&result, |caps: &regex::Captures| {
                let key = &caps[1];
                let default = &caps[2];
                slots
                    .get(key)
                    .map(|s| s.as_str())
                    .unwrap_or(default)
                    .to_string()
            })
            .to_string();

        // NOW process interpolations on the final result
        result = self.process_interpolations(&result);

        Ok(result)
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}
