#![allow(non_snake_case)]
use std::fmt::Display;
use dioxus::prelude::*;

/// A Collapse component for collapsible content.
///
/// # Examples
///
/// Basic usage:
///
/// ```text
/// use daisy_rsx::{Collapse, CollapseTitle, CollapseContent};
///
/// Collapse {
///     children: rsx!(
///         CollapseTitle { children: rsx!("Click me") }
///         CollapseContent { children: rsx!("Hidden content") }
///     )
/// }
/// ```

#[derive(Props, Clone, PartialEq)]
pub struct CollapseProps {
    /// The content to display inside collapse (CollapseTitle and CollapseContent children)
    children: Element,
    /// Optional ID for collapse element
    id: Option<String>,
    /// Additional CSS classes to apply to collapse
    class: Option<String>,
}

#[component]
pub fn Collapse(props: CollapseProps) -> Element {
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["collapse".to_string()];
    
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

#[derive(Props, Clone, PartialEq)]
pub struct CollapseTitleProps {
    /// The content to display inside collapse title
    children: Element,
    /// Optional ID for collapse title element
    id: Option<String>,
    /// Additional CSS classes to apply to collapse title
    class: Option<String>,
}

#[component]
pub fn CollapseTitle(props: CollapseTitleProps) -> Element {
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["collapse-title".to_string()];
    
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

#[derive(Props, Clone, PartialEq)]
pub struct CollapseContentProps {
    /// The content to display inside collapse content
    children: Element,
    /// Optional ID for collapse content element
    id: Option<String>,
    /// Additional CSS classes to apply to collapse content
    class: Option<String>,
}

#[component]
pub fn CollapseContent(props: CollapseContentProps) -> Element {
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["collapse-content".to_string()];
    
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
fn test_collapse_basic() {
    let props = CollapseProps {
        children: rsx!(
            CollapseTitle { children: rsx!("Click me") }
            CollapseContent { children: rsx!("Hidden content") }
        ),
        id: None,
        class: None,
    };

    let result = dioxus_ssr::render_element(Collapse(props));
    assert!(result.contains(r#"class="collapse""#));
}

#[test]
fn test_collapse_title() {
    let props = CollapseTitleProps {
        children: rsx!("Title"),
        id: None,
        class: None,
    };

    let result = dioxus_ssr::render_element(CollapseTitle(props));
    assert!(result.contains(r#"class="collapse-title""#));
}

#[test]
fn test_collapse_content() {
    let props = CollapseContentProps {
        children: rsx!("Content"),
        id: None,
        class: None,
    };

    let result = dioxus_ssr::render_element(CollapseContent(props));
    assert!(result.contains(r#"class="collapse-content""#));
}

#[test]
fn test_collapse_custom_class() {
    let props = CollapseProps {
        children: rsx!(
            CollapseTitle { children: rsx!("Click me") }
            CollapseContent { children: rsx!("Hidden content") }
        ),
        id: None,
        class: Some("custom-class".to_string()),
    };

    let result = dioxus_ssr::render_element(Collapse(props));
    assert!(result.contains(r#"class="collapse custom-class""#));
}

#[test]
fn test_collapse_with_id() {
    let props = CollapseProps {
        children: rsx!(
            CollapseTitle { children: rsx!("Click me") }
            CollapseContent { children: rsx!("Hidden content") }
        ),
        id: Some("test-collapse".to_string()),
        class: None,
    };

    let result = dioxus_ssr::render_element(Collapse(props));
    assert!(result.contains(r#"id="test-collapse""#));
}

#[test]
fn test_collapse_title_with_id() {
    let props = CollapseTitleProps {
        children: rsx!("Title"),
        id: Some("test-title".to_string()),
        class: None,
    };

    let result = dioxus_ssr::render_element(CollapseTitle(props));
    assert!(result.contains(r#"id="test-title""#));
}

#[test]
fn test_collapse_content_with_id() {
    let props = CollapseContentProps {
        children: rsx!("Content"),
        id: Some("test-content".to_string()),
        class: None,
    };

    let result = dioxus_ssr::render_element(CollapseContent(props));
    assert!(result.contains(r#"id="test-content""#));
}
