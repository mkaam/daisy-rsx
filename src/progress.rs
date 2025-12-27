#![allow(non_snake_case)]
use std::fmt::Display;
use dioxus::prelude::*;

/// A Progress component that displays progress indicators.
///
/// # Examples
///
/// Basic usage:
///
/// ```text
/// use daisy_rsx::{Progress, ProgressColorScheme};
///
/// Progress {
///     value: 50,
///     max: 100,
///     color_scheme: ProgressColorScheme::Primary,
/// }
/// ```
///
/// Indeterminate state:
///
/// ```text
/// Progress {
///     indeterminate: true,
///     color_scheme: ProgressColorScheme::Success,
/// }
/// ```

/// Color scheme options for Progress component
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ProgressColorScheme {
    #[default]
    /// Primary brand color scheme
    Primary,
    /// Secondary color scheme
    Secondary,
    /// Accent color scheme
    Accent,
    /// Informational blue color scheme
    Info,
    /// Success green color scheme
    Success,
    /// Warning yellow color scheme
    Warning,
    /// Error red color scheme
    Error,
}

impl Display for ProgressColorScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgressColorScheme::Primary => write!(f, "progress-primary"),
            ProgressColorScheme::Secondary => write!(f, "progress-secondary"),
            ProgressColorScheme::Accent => write!(f, "progress-accent"),
            ProgressColorScheme::Info => write!(f, "progress-info"),
            ProgressColorScheme::Success => write!(f, "progress-success"),
            ProgressColorScheme::Warning => write!(f, "progress-warning"),
            ProgressColorScheme::Error => write!(f, "progress-error"),
        }
    }
}

/// Size options for Progress component
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ProgressSize {
    #[default]
    /// Default size
    Default,
    /// Small progress bar
    Small,
    /// Medium progress bar
    Medium,
    /// Large progress bar
    Large,
}

impl Display for ProgressSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgressSize::Default => write!(f, ""),
            ProgressSize::Small => write!(f, "progress-sm"),
            ProgressSize::Medium => write!(f, "progress-md"),
            ProgressSize::Large => write!(f, "progress-lg"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ProgressProps {
    /// Optional ID for the progress element
    id: Option<String>,
    /// Additional CSS classes to apply to the progress bar
    class: Option<String>,
    /// Current value of the progress (0-100)
    value: Option<f64>,
    /// Maximum value of the progress (default: 100)
    max: Option<f64>,
    /// Color scheme for the progress bar
    color_scheme: Option<ProgressColorScheme>,
    /// Size of the progress bar
    size: Option<ProgressSize>,
    /// Whether the progress is in indeterminate state
    indeterminate: Option<bool>,
}

#[component]
pub fn Progress(props: ProgressProps) -> Element {
    let color_scheme = props.color_scheme.unwrap_or_default();
    let size = props.size.unwrap_or_default();
    let class = props.class.unwrap_or_default();
    let indeterminate = props.indeterminate.filter(|&x| x);
    let value = props.value.unwrap_or(0.0);
    let max = props.max.unwrap_or(100.0);

    // Build CSS classes
    let mut classes = vec!["progress".to_string()];
    
    if !color_scheme.to_string().is_empty() {
        classes.push(color_scheme.to_string());
    }
    
    if !size.to_string().is_empty() {
        classes.push(size.to_string());
    }
    
    if indeterminate.is_some() {
        classes.push("progress-indeterminate".to_string());
    }
    
    if !class.is_empty() {
        classes.push(class);
    }

    let class_string = classes.join(" ");

    // Calculate percentage for determinate progress
    let percentage = if indeterminate.is_none() {
        let pct = (value / max * 100.0).min(100.0).max(0.0);
        Some(format!("{}%", pct))
    } else {
        None
    };

    rsx!(
        div {
            class: "{class_string}",
            r#role: "progressbar",
            id: props.id,
            "aria-valuenow": "{value}",
            "aria-valuemin": "0",
            "aria-valuemax": "{max}",
            style: if percentage.is_some() {
                format!("width: {}", percentage.unwrap())
            } else {
                "".to_string()
            },
        }
    )
}

#[test]
fn test_progress_basic() {
    let props = ProgressProps {
        id: None,
        class: None,
        value: Some(50.0),
        max: Some(100.0),
        color_scheme: None,
        size: None,
        indeterminate: None,
    };

    let result = dioxus_ssr::render_element(Progress(props));
    assert!(result.contains("progress"));
}

#[test]
fn test_progress_with_color_scheme() {
    let schemes = [
        (ProgressColorScheme::Primary, "progress-primary"),
        (ProgressColorScheme::Secondary, "progress-secondary"),
        (ProgressColorScheme::Accent, "progress-accent"),
        (ProgressColorScheme::Info, "progress-info"),
        (ProgressColorScheme::Success, "progress-success"),
        (ProgressColorScheme::Warning, "progress-warning"),
        (ProgressColorScheme::Error, "progress-error"),
    ];

    for (scheme, expected_class) in schemes {
        let props = ProgressProps {
            id: None,
            class: None,
            value: Some(50.0),
            max: Some(100.0),
            color_scheme: Some(scheme),
            size: None,
            indeterminate: None,
        };

        let result = dioxus_ssr::render_element(Progress(props));
        assert!(result.contains(expected_class),
                "Expected '{}' to contain '{}', but got: {}",
                result, expected_class, result);
    }
}

#[test]
fn test_progress_with_size() {
    let sizes = [
        (ProgressSize::Default, ""),
        (ProgressSize::Small, "progress-sm"),
        (ProgressSize::Medium, "progress-md"),
        (ProgressSize::Large, "progress-lg"),
    ];

    for (size, expected_class) in sizes {
        let props = ProgressProps {
            id: None,
            class: None,
            value: Some(50.0),
            max: Some(100.0),
            color_scheme: None,
            size: Some(size),
            indeterminate: None,
        };

        let result = dioxus_ssr::render_element(Progress(props));
        if expected_class.is_empty() {
            assert!(result.contains("progress"));
        } else {
            assert!(result.contains(expected_class),
                    "Expected '{}' to contain '{}', but got: {}",
                    result, expected_class, result);
        }
    }
}

#[test]
fn test_progress_indeterminate() {
    let props = ProgressProps {
        id: None,
        class: None,
        value: None,
        max: None,
        color_scheme: None,
        size: None,
        indeterminate: Some(true),
    };

    let result = dioxus_ssr::render_element(Progress(props));
    assert!(result.contains("progress") && result.contains("progress-indeterminate"));
    assert!(result.contains(r#"role="progressbar""#));
}

#[test]
fn test_progress_with_custom_class() {
    let props = ProgressProps {
        id: None,
        class: Some("custom-class".to_string()),
        value: Some(50.0),
        max: Some(100.0),
        color_scheme: None,
        size: None,
        indeterminate: None,
    };

    let result = dioxus_ssr::render_element(Progress(props));
    assert!(result.contains("progress") && result.contains("custom-class"));
}

#[test]
fn test_progress_with_id() {
    let props = ProgressProps {
        id: Some("test-progress".to_string()),
        class: None,
        value: Some(50.0),
        max: Some(100.0),
        color_scheme: None,
        size: None,
        indeterminate: None,
    };

    let result = dioxus_ssr::render_element(Progress(props));
    assert!(result.contains(r#"id="test-progress""#));
}
