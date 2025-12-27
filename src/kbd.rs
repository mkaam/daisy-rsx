#![allow(non_snake_case)]
use dioxus::prelude::*;

/// A Kbd component for displaying keyboard shortcuts.
///
/// # Examples
///
/// Basic usage:
///
/// ```text
/// use daisy_rsx::Kbd;
///
/// Kbd {
///     children: rsx!("Ctrl")
/// }
/// ```

#[derive(Props, Clone, PartialEq)]
pub struct KbdProps {
    /// The content to display inside kbd
    children: Element,
    /// Optional ID for kbd element
    id: Option<String>,
    /// Additional CSS classes to apply to kbd
    class: Option<String>,
}

#[component]
pub fn Kbd(props: KbdProps) -> Element {
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["kbd".to_string()];
    
    if !class.is_empty() {
        classes.push(class);
    }

    let class_string = classes.join(" ");

    rsx!(
        kbd {
            class: "{class_string}",
            id: props.id,
            {props.children}
        }
    )
}

#[test]
fn test_kbd_basic() {
    let props = KbdProps {
        children: rsx!("Ctrl"),
        id: None,
        class: None,
    };

    let result = dioxus_ssr::render_element(Kbd(props));
    assert!(result.contains(r#"class="kbd""#));
}

#[test]
fn test_kbd_custom_class() {
    let props = KbdProps {
        children: rsx!("Cmd"),
        id: None,
        class: Some("custom-class".to_string()),
    };

    let result = dioxus_ssr::render_element(Kbd(props));
    assert!(result.contains(r#"class="kbd custom-class""#));
}

#[test]
fn test_kbd_with_id() {
    let props = KbdProps {
        children: rsx!("Shift"),
        id: Some("test-kbd".to_string()),
        class: None,
    };

    let result = dioxus_ssr::render_element(Kbd(props));
    assert!(result.contains(r#"id="test-kbd""#));
}

#[test]
fn test_kbd_multiple_keys() {
    let props = KbdProps {
        children: rsx!("Ctrl"),
        id: None,
        class: None,
    };

    let result = dioxus_ssr::render_element(Kbd(props));
    assert!(result.contains("Ctrl"));
}
