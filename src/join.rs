#![allow(non_snake_case)]
use std::fmt::Display;
use dioxus::prelude::*;

/// A Join component that allows joining multiple elements together visually, removing borders between adjacent elements.
///
/// # Examples
///
/// Basic usage:
///
/// ```text
/// use daisy_rsx::{Join, JoinItem, JoinOrientation};
///
/// Join {
///     orientation: JoinOrientation::Horizontal,
///     JoinItem { ButtonUI { "Button 1" } }
///     JoinItem { ButtonUI { "Button 2" } }
///     JoinItem { ButtonUI { "Button 3" } }
/// }
/// ```
///
/// With input and button:
///
/// ```text
/// Join {
///     orientation: JoinOrientation::Horizontal,
///     JoinItem { Input { placeholder: "Search..." } }
///     JoinItem { ButtonUI { "Search" } }
/// }
/// ```

/// Orientation options for Join component
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum JoinOrientation {
    #[default]
    /// Horizontal orientation (default)
    Horizontal,
    /// Vertical orientation
    Vertical,
}

impl Display for JoinOrientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JoinOrientation::Horizontal => write!(f, "join-horizontal"),
            JoinOrientation::Vertical => write!(f, "join-vertical"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct JoinProps {
    /// The content to display inside the join container
    children: Element,
    /// Optional ID for the join container
    id: Option<String>,
    /// Additional CSS classes to apply to the join container
    class: Option<String>,
    /// Orientation of the join (horizontal or vertical)
    orientation: Option<JoinOrientation>,
}

#[component]
pub fn Join(props: JoinProps) -> Element {
    let orientation = props.orientation.unwrap_or_default();
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["join".to_string()];
    classes.push(orientation.to_string());
    
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
pub struct JoinItemProps {
    /// The content to display inside the join item
    children: Element,
    /// Additional CSS classes to apply to the join item
    class: Option<String>,
}

#[component]
pub fn JoinItem(props: JoinItemProps) -> Element {
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["join-item".to_string()];
    
    if !class.is_empty() {
        classes.push(class);
    }

    let class_string = classes.join(" ");

    rsx!(
        div {
            class: "{class_string}",
            {props.children}
        }
    )
}

#[test]
fn test_join_basic() {
    let props = JoinProps {
        children: rsx!(
            JoinItem { children: rsx!("Item 1") }
            JoinItem { children: rsx!("Item 2") }
            JoinItem { children: rsx!("Item 3") }
        ),
        id: None,
        class: None,
        orientation: None,
    };

    let result = dioxus_ssr::render_element(Join(props));
    assert!(result.contains(r#"class="join join-horizontal""#));
    assert!(result.contains(r#"class="join-item""#));
}

#[test]
fn test_join_horizontal() {
    let props = JoinProps {
        children: rsx!(
            JoinItem { children: rsx!("Item 1") }
            JoinItem { children: rsx!("Item 2") }
        ),
        id: None,
        class: None,
        orientation: Some(JoinOrientation::Horizontal),
    };

    let result = dioxus_ssr::render_element(Join(props));
    assert!(result.contains(r#"class="join join-horizontal""#));
}

#[test]
fn test_join_vertical() {
    let props = JoinProps {
        children: rsx!(
            JoinItem { children: rsx!("Item 1") }
            JoinItem { children: rsx!("Item 2") }
        ),
        id: None,
        class: None,
        orientation: Some(JoinOrientation::Vertical),
    };

    let result = dioxus_ssr::render_element(Join(props));
    assert!(result.contains(r#"class="join join-vertical""#));
}

#[test]
fn test_join_with_custom_class() {
    let props = JoinProps {
        children: rsx!(
            JoinItem { children: rsx!("Item 1") }
            JoinItem { children: rsx!("Item 2") }
        ),
        id: None,
        class: Some("custom-class".to_string()),
        orientation: None,
    };

    let result = dioxus_ssr::render_element(Join(props));
    assert!(result.contains(r#"class="join join-horizontal custom-class""#));
}

#[test]
fn test_join_item_with_custom_class() {
    let props = JoinItemProps {
        children: rsx!("Item"),
        class: Some("custom-item-class".to_string()),
    };

    let result = dioxus_ssr::render_element(JoinItem(props));
    assert!(result.contains(r#"class="join-item custom-item-class""#));
}

#[test]
fn test_join_with_id() {
    let props = JoinProps {
        children: rsx!(
            JoinItem { children: rsx!("Item 1") }
            JoinItem { children: rsx!("Item 2") }
        ),
        id: Some("test-join".to_string()),
        class: None,
        orientation: None,
    };

    let result = dioxus_ssr::render_element(Join(props));
    assert!(result.contains(r#"id="test-join""#));
}
