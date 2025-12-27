#![allow(non_snake_case)]
use std::fmt::Display;
use dioxus::prelude::*;

/// A Divider component for creating visual separators.
///
/// # Examples
///
/// Basic usage:
///
/// ```text
/// use daisy_rsx::{Divider, DividerOrientation};
///
/// Divider {
///     orientation: DividerOrientation::Horizontal,
///     children: rsx!("Or")
/// }
/// ```

/// Orientation options for Divider component
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DividerOrientation {
    /// Horizontal orientation
    Horizontal,
    /// Vertical orientation
    Vertical,
}

impl Display for DividerOrientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DividerOrientation::Horizontal => write!(f, "divider-horizontal"),
            DividerOrientation::Vertical => write!(f, "divider-vertical"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DividerProps {
    /// The content to display inside divider (optional text)
    children: Element,
    /// Optional ID for divider element
    id: Option<String>,
    /// Additional CSS classes to apply to divider
    class: Option<String>,
    /// Orientation of divider (horizontal or vertical)
    orientation: Option<DividerOrientation>,
}

#[component]
pub fn Divider(props: DividerProps) -> Element {
    let class = props.class.unwrap_or_default();
    let orientation = props.orientation;

    // Build CSS classes
    let mut classes = vec!["divider".to_string()];
    
    if let Some(orient) = orientation {
        classes.push(orient.to_string());
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
fn test_divider_basic() {
    let props = DividerProps {
        children: rsx!("Or"),
        id: None,
        class: None,
        orientation: None,
    };

    let result = dioxus_ssr::render_element(Divider(props));
    assert!(result.contains(r#"class="divider""#));
}

#[test]
fn test_divider_horizontal() {
    let props = DividerProps {
        children: rsx!("Horizontal Divider"),
        id: None,
        class: None,
        orientation: Some(DividerOrientation::Horizontal),
    };

    let result = dioxus_ssr::render_element(Divider(props));
    assert!(result.contains(r#"class="divider divider-horizontal""#));
}

#[test]
fn test_divider_vertical() {
    let props = DividerProps {
        children: rsx!("Vertical Divider"),
        id: None,
        class: None,
        orientation: Some(DividerOrientation::Vertical),
    };

    let result = dioxus_ssr::render_element(Divider(props));
    assert!(result.contains(r#"class="divider divider-vertical""#));
}

#[test]
fn test_divider_custom_class() {
    let props = DividerProps {
        children: rsx!("Custom Divider"),
        id: None,
        class: Some("custom-class".to_string()),
        orientation: None,
    };

    let result = dioxus_ssr::render_element(Divider(props));
    assert!(result.contains(r#"class="divider custom-class""#));
}

#[test]
fn test_divider_with_id() {
    let props = DividerProps {
        children: rsx!("Divider with ID"),
        id: Some("test-divider".to_string()),
        class: None,
        orientation: None,
    };

    let result = dioxus_ssr::render_element(Divider(props));
    assert!(result.contains(r#"id="test-divider""#));
}

#[test]
fn test_divider_empty() {
    let props = DividerProps {
        children: rsx!(),
        id: None,
        class: None,
        orientation: None,
    };

    let result = dioxus_ssr::render_element(Divider(props));
    assert!(result.contains(r#"class="divider""#));
}
