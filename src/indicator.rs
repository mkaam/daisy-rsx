#![allow(non_snake_case)]
use std::fmt::Display;
use dioxus::prelude::*;

/// An Indicator component for displaying badges/indicators on elements.
///
/// # Examples
///
/// Basic usage:
///
/// ```text
/// use daisy_rsx::{Indicator, IndicatorItem};
///
/// Indicator {
///     children: rsx!(
///         IndicatorItem { children: rsx!("3") }
///         div { "Content" }
///     )
/// }
/// ```

#[derive(Props, Clone, PartialEq)]
pub struct IndicatorProps {
    /// The content to display inside indicator (IndicatorItem and other content)
    children: Element,
    /// Optional ID for indicator element
    id: Option<String>,
    /// Additional CSS classes to apply to indicator
    class: Option<String>,
}

#[component]
pub fn Indicator(props: IndicatorProps) -> Element {
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["indicator".to_string()];
    
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
pub struct IndicatorItemProps {
    /// The content to display inside indicator item
    children: Element,
    /// Optional ID for indicator item element
    id: Option<String>,
    /// Additional CSS classes to apply to indicator item
    class: Option<String>,
}

#[component]
pub fn IndicatorItem(props: IndicatorItemProps) -> Element {
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["indicator-item".to_string()];
    
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
fn test_indicator_basic() {
    let props = IndicatorProps {
        children: rsx!(
            IndicatorItem { children: rsx!("3") }
            div { "Content" }
        ),
        id: None,
        class: None,
    };

    let result = dioxus_ssr::render_element(Indicator(props));
    assert!(result.contains(r#"class="indicator""#));
}

#[test]
fn test_indicator_item() {
    let props = IndicatorItemProps {
        children: rsx!("5"),
        id: None,
        class: None,
    };

    let result = dioxus_ssr::render_element(IndicatorItem(props));
    assert!(result.contains(r#"class="indicator-item""#));
}

#[test]
fn test_indicator_custom_class() {
    let props = IndicatorProps {
        children: rsx!(
            IndicatorItem { children: rsx!("3") }
            div { "Content" }
        ),
        id: None,
        class: Some("custom-class".to_string()),
    };

    let result = dioxus_ssr::render_element(Indicator(props));
    assert!(result.contains(r#"class="indicator custom-class""#));
}

#[test]
fn test_indicator_with_id() {
    let props = IndicatorProps {
        children: rsx!(
            IndicatorItem { children: rsx!("3") }
            div { "Content" }
        ),
        id: Some("test-indicator".to_string()),
        class: None,
    };

    let result = dioxus_ssr::render_element(Indicator(props));
    assert!(result.contains(r#"id="test-indicator""#));
}

#[test]
fn test_indicator_item_with_id() {
    let props = IndicatorItemProps {
        children: rsx!("10"),
        id: Some("test-item".to_string()),
        class: None,
    };

    let result = dioxus_ssr::render_element(IndicatorItem(props));
    assert!(result.contains(r#"id="test-item""#));
}

#[test]
fn test_indicator_item_custom_class() {
    let props = IndicatorItemProps {
        children: rsx!("99+"),
        id: None,
        class: Some("custom-class".to_string()),
    };

    let result = dioxus_ssr::render_element(IndicatorItem(props));
    assert!(result.contains(r#"class="indicator-item custom-class""#));
}
