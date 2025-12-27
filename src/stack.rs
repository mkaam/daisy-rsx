#![allow(non_snake_case)]
use std::fmt::Display;
use dioxus::prelude::*;

/// A Stack component for stacking elements.
///
/// # Examples
///
/// Basic usage:
///
/// ```text
/// use daisy_rsx::{Stack, StackDirection};
///
/// Stack {
///     direction: StackDirection::Vertical,
///     children: rsx!(
///         div { "Item 1" }
///         div { "Item 2" }
///         div { "Item 3" }
///     )
/// }
/// ```

/// Direction options for Stack component
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum StackDirection {
    /// Vertical direction
    Vertical,
    /// Horizontal direction
    Horizontal,
}

impl Display for StackDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackDirection::Vertical => write!(f, "stack-vertical"),
            StackDirection::Horizontal => write!(f, "stack-horizontal"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct StackProps {
    /// The content to display inside stack
    children: Element,
    /// Optional ID for stack element
    id: Option<String>,
    /// Additional CSS classes to apply to stack
    class: Option<String>,
    /// Direction of stack (vertical or horizontal)
    direction: Option<StackDirection>,
}

#[component]
pub fn Stack(props: StackProps) -> Element {
    let class = props.class.unwrap_or_default();
    let direction = props.direction;

    // Build CSS classes
    let mut classes = vec!["stack".to_string()];
    
    if let Some(dir) = direction {
        classes.push(dir.to_string());
    }
    
    if !class.is_empty() {
        classes.push(class);
    }

    let class_string = classes.join(" ");

    rsx!(
        div {
            class: "{class_string}",
            id: props.id,
            {props.children}
        }
    )
}

#[test]
fn test_stack_basic() {
    let props = StackProps {
        children: rsx!(
            div { "Item 1" }
            div { "Item 2" }
            div { "Item 3" }
        ),
        id: None,
        class: None,
        direction: None,
    };

    let result = dioxus_ssr::render_element(Stack(props));
    assert!(result.contains(r#"class="stack""#));
}

#[test]
fn test_stack_vertical() {
    let props = StackProps {
        children: rsx!(
            div { "Item 1" }
            div { "Item 2" }
        ),
        id: None,
        class: None,
        direction: Some(StackDirection::Vertical),
    };

    let result = dioxus_ssr::render_element(Stack(props));
    assert!(result.contains(r#"class="stack stack-vertical""#));
}

#[test]
fn test_stack_horizontal() {
    let props = StackProps {
        children: rsx!(
            div { "Item 1" }
            div { "Item 2" }
        ),
        id: None,
        class: None,
        direction: Some(StackDirection::Horizontal),
    };

    let result = dioxus_ssr::render_element(Stack(props));
    assert!(result.contains(r#"class="stack stack-horizontal""#));
}

#[test]
fn test_stack_custom_class() {
    let props = StackProps {
        children: rsx!(
            div { "Item 1" }
        ),
        id: None,
        class: Some("custom-class".to_string()),
        direction: None,
    };

    let result = dioxus_ssr::render_element(Stack(props));
    assert!(result.contains(r#"class="stack custom-class""#));
}

#[test]
fn test_stack_with_id() {
    let props = StackProps {
        children: rsx!(
            div { "Item 1" }
        ),
        id: Some("test-stack".to_string()),
        class: None,
        direction: None,
    };

    let result = dioxus_ssr::render_element(Stack(props));
    assert!(result.contains(r#"id="test-stack""#));
}
