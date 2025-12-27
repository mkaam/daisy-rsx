#![allow(non_snake_case)]
use std::fmt::Display;
use dioxus::prelude::*;

/// A Countdown component for countdown timers.
///
/// # Examples
///
/// Basic usage:
///
/// ```text
/// use daisy_rsx::{Countdown, CountdownValue};
///
/// Countdown {
///     children: rsx!(
///         CountdownValue { value: 10 }
///         CountdownValue { value: 20 }
///         CountdownValue { value: 30 }
///     )
/// }
/// ```

#[derive(Props, Clone, PartialEq)]
pub struct CountdownProps {
    /// The content to display inside countdown (CountdownValue children)
    children: Element,
    /// Optional ID for countdown element
    id: Option<String>,
    /// Additional CSS classes to apply to countdown
    class: Option<String>,
}

#[component]
pub fn Countdown(props: CountdownProps) -> Element {
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["countdown".to_string()];
    
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
pub struct CountdownValueProps {
    /// Value for this countdown digit
    value: i32,
    /// Optional ID for countdown value element
    id: Option<String>,
    /// Additional CSS classes to apply to countdown value
    class: Option<String>,
}

#[component]
pub fn CountdownValue(props: CountdownValueProps) -> Element {
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec![];
    
    if !class.is_empty() {
        classes.push(class);
    }

    let class_string = classes.join(" ");

    rsx!(
        span {
            class: "{class_string}",
            id: props.id,
            "data-value": "{props.value}",
            "{props.value}"
        }
    )
}

#[test]
fn test_countdown_basic() {
    let props = CountdownProps {
        children: rsx!(
            CountdownValue { value: 10 }
            CountdownValue { value: 20 }
            CountdownValue { value: 30 }
        ),
        id: None,
        class: None,
    };

    let result = dioxus_ssr::render_element(Countdown(props));
    assert!(result.contains(r#"class="countdown""#));
}

#[test]
fn test_countdown_value() {
    let props = CountdownValueProps {
        value: 42,
        id: None,
        class: None,
    };

    let result = dioxus_ssr::render_element(CountdownValue(props));
    assert!(result.contains(r#"data-value="42""#));
    assert!(result.contains("42"));
}

#[test]
fn test_countdown_custom_class() {
    let props = CountdownProps {
        children: rsx!(
            CountdownValue { value: 10 }
        ),
        id: None,
        class: Some("custom-class".to_string()),
    };

    let result = dioxus_ssr::render_element(Countdown(props));
    assert!(result.contains(r#"class="countdown custom-class""#));
}

#[test]
fn test_countdown_with_id() {
    let props = CountdownProps {
        children: rsx!(
            CountdownValue { value: 10 }
        ),
        id: Some("test-countdown".to_string()),
        class: None,
    };

    let result = dioxus_ssr::render_element(Countdown(props));
    assert!(result.contains(r#"id="test-countdown""#));
}

#[test]
fn test_countdown_value_with_id() {
    let props = CountdownValueProps {
        value: 99,
        id: Some("test-value".to_string()),
        class: None,
    };

    let result = dioxus_ssr::render_element(CountdownValue(props));
    assert!(result.contains(r#"id="test-value""#));
}

#[test]
fn test_countdown_value_custom_class() {
    let props = CountdownValueProps {
        value: 100,
        id: None,
        class: Some("custom-class".to_string()),
    };

    let result = dioxus_ssr::render_element(CountdownValue(props));
    assert!(result.contains(r#"class="custom-class""#));
}
