#![allow(non_snake_case)]
use std::fmt::Display;
use dioxus::prelude::*;

/// A Toggle component for switch-like controls.
///
/// # Examples
///
/// Basic usage:
///
/// ```text
/// use daisy_rsx::{Toggle, ToggleColorScheme, ToggleSize};
///
/// Toggle {
///     color_scheme: ToggleColorScheme::Primary,
///     size: ToggleSize::Default,
///     checked: None,
/// }
/// ```

/// Color scheme options for Toggle component
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ToggleColorScheme {
    /// Primary color
    Primary,
    /// Secondary color
    Secondary,
    /// Accent color
    Accent,
    /// Success color
    Success,
    /// Warning color
    Warning,
    /// Error color
    Error,
    /// Info color
    Info,
}

impl Display for ToggleColorScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToggleColorScheme::Primary => write!(f, "toggle-primary"),
            ToggleColorScheme::Secondary => write!(f, "toggle-secondary"),
            ToggleColorScheme::Accent => write!(f, "toggle-accent"),
            ToggleColorScheme::Success => write!(f, "toggle-success"),
            ToggleColorScheme::Warning => write!(f, "toggle-warning"),
            ToggleColorScheme::Error => write!(f, "toggle-error"),
            ToggleColorScheme::Info => write!(f, "toggle-info"),
        }
    }
}

/// Size options for Toggle component
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ToggleSize {
    /// Default size
    Default,
    /// Small size
    Small,
    /// Medium size
    Medium,
    /// Large size
    Large,
}

impl Display for ToggleSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToggleSize::Default => write!(f, ""),
            ToggleSize::Small => write!(f, "toggle-sm"),
            ToggleSize::Medium => write!(f, "toggle-md"),
            ToggleSize::Large => write!(f, "toggle-lg"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ToggleProps {
    /// Optional ID for toggle element
    id: Option<String>,
    /// Additional CSS classes to apply to toggle
    class: Option<String>,
    /// Color scheme of toggle
    color_scheme: Option<ToggleColorScheme>,
    /// Size of toggle
    size: Option<ToggleSize>,
    /// Whether toggle is checked
    checked: Option<bool>,
    /// Whether toggle is disabled
    disabled: Option<bool>,
    /// Optional name attribute
    name: Option<String>,
}

#[component]
pub fn Toggle(props: ToggleProps) -> Element {
    let class = props.class.unwrap_or_default();
    let color_scheme = props.color_scheme;
    let size = props.size;
    let checked = props.checked.filter(|&x| x);
    let disabled = props.disabled.filter(|&x| x);

    // Build CSS classes
    let mut classes = vec!["toggle".to_string()];
    
    if let Some(color) = color_scheme {
        classes.push(color.to_string());
    }
    
    if let Some(s) = size {
        let size_class = s.to_string();
        if !size_class.is_empty() {
            classes.push(size_class);
        }
    }
    
    if !class.is_empty() {
        classes.push(class);
    }

    let class_string = classes.join(" ");

    rsx!(
        input {
            class: "{class_string}",
            id: props.id,
            r#type: "checkbox",
            checked: checked,
            disabled: disabled,
            name: props.name,
        }
    )
}

#[test]
fn test_toggle_basic() {
    let props = ToggleProps {
        id: None,
        class: None,
        color_scheme: None,
        size: None,
        checked: None,
        disabled: None,
        name: None,
    };

    let result = dioxus_ssr::render_element(Toggle(props));
    assert!(result.contains(r#"class="toggle""#));
}

#[test]
fn test_toggle_checked() {
    let props = ToggleProps {
        id: None,
        class: None,
        color_scheme: None,
        size: None,
        checked: Some(true),
        disabled: None,
        name: None,
    };

    let result = dioxus_ssr::render_element(Toggle(props));
    assert!(result.contains(r#"checked"#));
}

#[test]
fn test_toggle_disabled() {
    let props = ToggleProps {
        id: None,
        class: None,
        color_scheme: None,
        size: None,
        checked: None,
        disabled: Some(true),
        name: None,
    };

    let result = dioxus_ssr::render_element(Toggle(props));
    assert!(result.contains(r#"disabled"#));
}

#[test]
fn test_toggle_color_schemes() {
    let color_schemes = vec![
        ToggleColorScheme::Primary,
        ToggleColorScheme::Secondary,
        ToggleColorScheme::Accent,
        ToggleColorScheme::Success,
        ToggleColorScheme::Warning,
        ToggleColorScheme::Error,
        ToggleColorScheme::Info,
    ];

    for color in color_schemes {
        let props = ToggleProps {
            id: None,
            class: None,
            color_scheme: Some(color),
            size: None,
            checked: None,
            disabled: None,
            name: None,
        };

        let result = dioxus_ssr::render_element(Toggle(props));
        assert!(result.contains(&format!("class=\"toggle {}\"", color.to_string())));
    }
}

#[test]
fn test_toggle_sizes() {
    let sizes = vec![
        ToggleSize::Default,
        ToggleSize::Small,
        ToggleSize::Medium,
        ToggleSize::Large,
    ];

    for size in sizes {
        let props = ToggleProps {
            id: None,
            class: None,
            color_scheme: None,
            size: Some(size),
            checked: None,
            disabled: None,
            name: None,
        };

        let result = dioxus_ssr::render_element(Toggle(props));
        let size_class = size.to_string();
        if !size_class.is_empty() {
            assert!(result.contains(&format!("class=\"toggle {}\"", size_class)));
        } else {
            assert!(result.contains(r#"class="toggle""#));
        }
    }
}

#[test]
fn test_toggle_custom_class() {
    let props = ToggleProps {
        id: None,
        class: Some("custom-class".to_string()),
        color_scheme: Some(ToggleColorScheme::Primary),
        size: None,
        checked: None,
        disabled: None,
        name: None,
    };

    let result = dioxus_ssr::render_element(Toggle(props));
    assert!(result.contains(r#"class="toggle toggle-primary custom-class""#));
}

#[test]
fn test_toggle_with_id() {
    let props = ToggleProps {
        id: Some("test-toggle".to_string()),
        class: None,
        color_scheme: None,
        size: None,
        checked: None,
        disabled: None,
        name: None,
    };

    let result = dioxus_ssr::render_element(Toggle(props));
    assert!(result.contains(r#"id="test-toggle""#));
}

#[test]
fn test_toggle_with_name() {
    let props = ToggleProps {
        id: None,
        class: None,
        color_scheme: None,
        size: None,
        checked: None,
        disabled: None,
        name: Some("toggle-name".to_string()),
    };

    let result = dioxus_ssr::render_element(Toggle(props));
    assert!(result.contains(r#"name="toggle-name""#));
}
