#![allow(non_snake_case)]
use std::fmt::Display;
use dioxus::prelude::*;

/// A Toast component for displaying notifications.
///
/// # Examples
///
/// Basic usage:
///
/// ```text
/// use daisy_rsx::{Toast, ToastType};
///
/// Toast {
///     r#type: ToastType::Success,
///     children: rsx!("Operation completed successfully!")
/// }
/// ```

/// Toast type variants
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ToastType {
    /// Success toast
    Success,
    /// Info toast
    Info,
    /// Warning toast
    Warning,
    /// Error toast
    Error,
}

impl Display for ToastType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToastType::Success => write!(f, "alert-success"),
            ToastType::Info => write!(f, "alert-info"),
            ToastType::Warning => write!(f, "alert-warning"),
            ToastType::Error => write!(f, "alert-error"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ToastProps {
    /// The content to display inside toast
    children: Element,
    /// Type of toast (success, info, warning, error)
    r#type: ToastType,
    /// Optional ID for toast element
    id: Option<String>,
    /// Additional CSS classes to apply to toast
    class: Option<String>,
}

#[component]
pub fn Toast(props: ToastProps) -> Element {
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["alert".to_string()];
    classes.push(props.r#type.to_string());
    
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
fn test_toast_success() {
    let props = ToastProps {
        children: rsx!("Success message"),
        r#type: ToastType::Success,
        id: None,
        class: None,
    };

    let result = dioxus_ssr::render_element(Toast(props));
    assert!(result.contains(r#"class="alert alert-success""#));
}

#[test]
fn test_toast_info() {
    let props = ToastProps {
        children: rsx!("Info message"),
        r#type: ToastType::Info,
        id: None,
        class: None,
    };

    let result = dioxus_ssr::render_element(Toast(props));
    assert!(result.contains(r#"class="alert alert-info""#));
}

#[test]
fn test_toast_warning() {
    let props = ToastProps {
        children: rsx!("Warning message"),
        r#type: ToastType::Warning,
        id: None,
        class: None,
    };

    let result = dioxus_ssr::render_element(Toast(props));
    assert!(result.contains(r#"class="alert alert-warning""#));
}

#[test]
fn test_toast_error() {
    let props = ToastProps {
        children: rsx!("Error message"),
        r#type: ToastType::Error,
        id: None,
        class: None,
    };

    let result = dioxus_ssr::render_element(Toast(props));
    assert!(result.contains(r#"class="alert alert-error""#));
}

#[test]
fn test_toast_custom_class() {
    let props = ToastProps {
        children: rsx!("Custom toast"),
        r#type: ToastType::Success,
        id: None,
        class: Some("custom-class".to_string()),
    };

    let result = dioxus_ssr::render_element(Toast(props));
    assert!(result.contains(r#"class="alert alert-success custom-class""#));
}

#[test]
fn test_toast_with_id() {
    let props = ToastProps {
        children: rsx!("Toast with id"),
        r#type: ToastType::Info,
        id: Some("test-toast".to_string()),
        class: None,
    };

    let result = dioxus_ssr::render_element(Toast(props));
    assert!(result.contains(r#"id="test-toast""#));
}
