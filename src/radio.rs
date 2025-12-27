#![allow(non_snake_case)]
use std::fmt::Display;
use dioxus::prelude::*;

/// A Radio component that allows users to select one option from a set of choices.
///
/// # Examples
///
/// Basic usage:
///
/// ```text
/// use daisy_rsx::{Radio, RadioColorScheme};
///
/// Radio {
///     name: "option",
///     value: "1",
///     color_scheme: RadioColorScheme::Primary,
///     "Option 1"
/// }
/// ```

/// Color scheme options for Radio component
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum RadioColorScheme {
    #[default]
    /// Primary brand color scheme
    Primary,
    /// Secondary color scheme
    Secondary,
    /// Accent color scheme
    Accent,
    /// Success green color scheme
    Success,
    /// Warning yellow color scheme
    Warning,
    /// Error red color scheme
    Error,
}

impl Display for RadioColorScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RadioColorScheme::Primary => write!(f, "radio-primary"),
            RadioColorScheme::Secondary => write!(f, "radio-secondary"),
            RadioColorScheme::Accent => write!(f, "radio-accent"),
            RadioColorScheme::Success => write!(f, "radio-success"),
            RadioColorScheme::Warning => write!(f, "radio-warning"),
            RadioColorScheme::Error => write!(f, "radio-error"),
        }
    }
}

/// Size options for Radio component
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum RadioSize {
    #[default]
    /// Default size
    Default,
    /// Small radio button
    Small,
    /// Medium radio button
    Medium,
    /// Large radio button
    Large,
}

impl Display for RadioSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RadioSize::Default => write!(f, ""),
            RadioSize::Small => write!(f, "radio-sm"),
            RadioSize::Medium => write!(f, "radio-md"),
            RadioSize::Large => write!(f, "radio-lg"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct RadioProps {
    /// The content to display inside the radio label
    children: Element,
    /// Optional ID for the radio element
    id: Option<String>,
    /// Additional CSS classes to apply to the radio
    class: Option<String>,
    /// Name of the radio group
    name: String,
    /// Value of the radio option
    value: String,
    /// Color scheme for the radio
    color_scheme: Option<RadioColorScheme>,
    /// Size of the radio
    size: Option<RadioSize>,
    /// Whether the radio is checked
    checked: Option<bool>,
    /// Whether the radio is disabled
    disabled: Option<bool>,
    /// Whether the radio is required
    required: Option<bool>,
}

#[component]
pub fn Radio(props: RadioProps) -> Element {
    let color_scheme = props.color_scheme.unwrap_or_default();
    let size = props.size.unwrap_or_default();
    let class = props.class.unwrap_or_default();
    let checked = props.checked.filter(|&x| x);
    let disabled = props.disabled.filter(|&x| x);
    let required = props.required.filter(|&x| x);

    // Build CSS classes
    let mut classes = vec!["radio".to_string()];
    
    if !color_scheme.to_string().is_empty() {
        classes.push(color_scheme.to_string());
    }
    
    if !size.to_string().is_empty() {
        classes.push(size.to_string());
    }
    
    if !class.is_empty() {
        classes.push(class);
    }

    let class_string = classes.join(" ");

    rsx!(
        label {
            class: "{class_string}",
            input {
                r#type: "radio",
                name: "{props.name}",
                value: "{props.value}",
                checked: checked,
                disabled: disabled,
                required: required,
                id: props.id.clone(),
            }
            {props.children}
        }
    )
}

#[test]
fn test_radio_basic() {
    let props = RadioProps {
        children: rsx!("Option 1"),
        id: None,
        class: None,
        name: "option".to_string(),
        value: "1".to_string(),
        color_scheme: None,
        size: None,
        checked: None,
        disabled: None,
        required: None,
    };

    let result = dioxus_ssr::render_element(Radio(props));
    assert!(result.contains("radio"));
    assert!(result.contains(r#"name="option""#));
    assert!(result.contains(r#"value="1""#));
}

#[test]
fn test_radio_checked() {
    let props = RadioProps {
        children: rsx!("Option 1"),
        id: None,
        class: None,
        name: "option".to_string(),
        value: "1".to_string(),
        color_scheme: None,
        size: None,
        checked: Some(true),
        disabled: None,
        required: None,
    };

    let result = dioxus_ssr::render_element(Radio(props));
    assert!(result.contains(r#"checked"#));
}

#[test]
fn test_radio_disabled() {
    let props = RadioProps {
        children: rsx!("Option 1"),
        id: None,
        class: None,
        name: "option".to_string(),
        value: "1".to_string(),
        color_scheme: None,
        size: None,
        checked: None,
        disabled: Some(true),
        required: None,
    };

    let result = dioxus_ssr::render_element(Radio(props));
    assert!(result.contains(r#"disabled"#));
}

#[test]
fn test_radio_with_color_scheme() {
    let schemes = [
        (RadioColorScheme::Primary, "radio-primary"),
        (RadioColorScheme::Secondary, "radio-secondary"),
        (RadioColorScheme::Accent, "radio-accent"),
        (RadioColorScheme::Success, "radio-success"),
        (RadioColorScheme::Warning, "radio-warning"),
        (RadioColorScheme::Error, "radio-error"),
    ];

    for (scheme, expected_class) in schemes {
        let props = RadioProps {
            children: rsx!("Option"),
            id: None,
            class: None,
            name: "option".to_string(),
            value: "1".to_string(),
            color_scheme: Some(scheme),
            size: None,
            checked: None,
            disabled: None,
            required: None,
        };

        let result = dioxus_ssr::render_element(Radio(props));
        assert!(result.contains(expected_class),
                "Expected '{}' to contain '{}', but got: {}",
                result, expected_class, result);
    }
}

#[test]
fn test_radio_with_size() {
    let sizes = [
        (RadioSize::Default, ""),
        (RadioSize::Small, "radio-sm"),
        (RadioSize::Medium, "radio-md"),
        (RadioSize::Large, "radio-lg"),
    ];

    for (size, expected_class) in sizes {
        let props = RadioProps {
            children: rsx!("Option"),
            id: None,
            class: None,
            name: "option".to_string(),
            value: "1".to_string(),
            color_scheme: None,
            size: Some(size),
            checked: None,
            disabled: None,
            required: None,
        };

        let result = dioxus_ssr::render_element(Radio(props));
        if expected_class.is_empty() {
            assert!(result.contains("radio"));
        } else {
            assert!(result.contains(expected_class),
                    "Expected '{}' to contain '{}', but got: {}",
                    result, expected_class, result);
        }
    }
}

#[test]
fn test_radio_with_custom_class() {
    let props = RadioProps {
        children: rsx!("Option"),
        id: None,
        class: Some("custom-class".to_string()),
        name: "option".to_string(),
        value: "1".to_string(),
        color_scheme: None,
        size: None,
        checked: None,
        disabled: None,
        required: None,
    };

    let result = dioxus_ssr::render_element(Radio(props));
    assert!(result.contains("radio") && result.contains("custom-class"));
}

#[test]
fn test_radio_with_id() {
    let props = RadioProps {
        children: rsx!("Option"),
        id: Some("test-radio".to_string()),
        class: None,
        name: "option".to_string(),
        value: "1".to_string(),
        color_scheme: None,
        size: None,
        checked: None,
        disabled: None,
        required: None,
    };

    let result = dioxus_ssr::render_element(Radio(props));
    assert!(result.contains(r#"id="test-radio""#));
}
