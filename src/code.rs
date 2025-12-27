#![allow(non_snake_case)]
use std::fmt::Display;
use dioxus::prelude::*;

/// A Code component for displaying code snippets.
///
/// # Examples
///
/// Basic usage:
///
/// ```text
/// use daisy_rsx::{Code, CodeType};
///
/// Code {
///     r#type: CodeType::Inline,
///     children: rsx!("const x = 1;")
/// }
/// ```

/// Type options for Code component
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CodeType {
    /// Inline code
    Inline,
    /// Block code
    Block,
}

impl Display for CodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeType::Inline => write!(f, ""),
            CodeType::Block => write!(f, "mockup-code"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CodeProps {
    /// The content to display inside code
    children: Element,
    /// Optional ID for code element
    id: Option<String>,
    /// Additional CSS classes to apply to code
    class: Option<String>,
    /// Type of code (inline or block)
    r#type: Option<CodeType>,
}

#[component]
pub fn Code(props: CodeProps) -> Element {
    let class = props.class.unwrap_or_default();
    let code_type = props.r#type.unwrap_or(CodeType::Inline);

    if code_type == CodeType::Inline {
        // Inline code - use code element
        let mut classes = vec![];
        
        if !class.is_empty() {
            classes.push(class);
        }

        let class_string = classes.join(" ");

        rsx!(
            code {
                class: "{class_string}",
                id: props.id,
                {props.children}
            }
        )
    } else {
        // Block code - use pre with mockup-code class
        let mut classes = vec!["mockup-code".to_string()];
        
        if !class.is_empty() {
            classes.push(class);
        }

        let class_string = classes.join(" ");

        rsx!(
            pre {
                class: "{class_string}",
                id: props.id,
                {props.children}
            }
        )
    }
}

#[test]
fn test_code_inline() {
    let props = CodeProps {
        children: rsx!("const x = 1;"),
        id: None,
        class: None,
        r#type: Some(CodeType::Inline),
    };

    let result = dioxus_ssr::render_element(Code(props));
    assert!(result.contains(r#"<code"#));
}

#[test]
fn test_code_block() {
    let props = CodeProps {
        children: rsx!("const x = 1;"),
        id: None,
        class: None,
        r#type: Some(CodeType::Block),
    };

    let result = dioxus_ssr::render_element(Code(props));
    assert!(result.contains(r#"class="mockup-code""#));
}

#[test]
fn test_code_custom_class() {
    let props = CodeProps {
        children: rsx!("const x = 1;"),
        id: None,
        class: Some("custom-class".to_string()),
        r#type: Some(CodeType::Inline),
    };

    let result = dioxus_ssr::render_element(Code(props));
    assert!(result.contains(r#"class="custom-class""#));
}

#[test]
fn test_code_with_id() {
    let props = CodeProps {
        children: rsx!("const x = 1;"),
        id: Some("test-code".to_string()),
        class: None,
        r#type: Some(CodeType::Inline),
    };

    let result = dioxus_ssr::render_element(Code(props));
    assert!(result.contains(r#"id="test-code""#));
}

#[test]
fn test_code_block_with_custom_class() {
    let props = CodeProps {
        children: rsx!("const x = 1;"),
        id: None,
        class: Some("custom-class".to_string()),
        r#type: Some(CodeType::Block),
    };

    let result = dioxus_ssr::render_element(Code(props));
    assert!(result.contains(r#"class="mockup-code custom-class""#));
}
