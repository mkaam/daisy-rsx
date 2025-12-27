#![allow(non_snake_case)]
use std::fmt::Display;
use dioxus::prelude::*;

/// A Stats component for displaying statistics and metrics.
///
/// # Examples
///
/// Basic usage:
///
/// ```text
/// use daisy_rsx::{Stats, StatsItem, StatsTitle, StatsValue, StatsDescription, StatsColorScheme};
///
/// Stats {
///     children: rsx!(
///         StatsItem {
///             StatsTitle { children: rsx!("Total Users") }
///             StatsValue { children: rsx!("10,543") }
///             StatsDescription { children: rsx!("+12% from last month") }
///         }
///     )
/// }
/// ```

/// Color scheme options for Stats component
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum StatsColorScheme {
    /// Primary color
    Primary,
    /// Secondary color
    Secondary,
    /// Accent color
    Accent,
    /// Info color
    Info,
    /// Success color
    Success,
    /// Warning color
    Warning,
    /// Error color
    Error,
}

impl Display for StatsColorScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatsColorScheme::Primary => write!(f, "stats-primary"),
            StatsColorScheme::Secondary => write!(f, "stats-secondary"),
            StatsColorScheme::Accent => write!(f, "stats-accent"),
            StatsColorScheme::Info => write!(f, "stats-info"),
            StatsColorScheme::Success => write!(f, "stats-success"),
            StatsColorScheme::Warning => write!(f, "stats-warning"),
            StatsColorScheme::Error => write!(f, "stats-error"),
        }
    }
}

/// Size options for Stats component
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum StatsSize {
    /// Small size
    Small,
    /// Medium size
    Medium,
    /// Large size
    Large,
}

impl Display for StatsSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatsSize::Small => write!(f, "stats-sm"),
            StatsSize::Medium => write!(f, "stats-md"),
            StatsSize::Large => write!(f, "stats-lg"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct StatsProps {
    /// The content to display inside stats (StatsItem children)
    children: Element,
    /// Optional ID for stats element
    id: Option<String>,
    /// Additional CSS classes to apply to stats
    class: Option<String>,
    /// Color scheme for stats
    color_scheme: Option<StatsColorScheme>,
    /// Size of stats
    size: Option<StatsSize>,
}

#[component]
pub fn Stats(props: StatsProps) -> Element {
    let class = props.class.unwrap_or_default();
    let color_scheme = props.color_scheme;
    let size = props.size;

    // Build CSS classes
    let mut classes = vec!["stats".to_string()];
    
    if let Some(color) = color_scheme {
        classes.push(color.to_string());
    }
    
    if let Some(s) = size {
        classes.push(s.to_string());
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

#[derive(Props, Clone, PartialEq)]
pub struct StatsItemProps {
    /// The content to display inside stats item
    children: Element,
    /// Optional ID for stats item element
    id: Option<String>,
    /// Additional CSS classes to apply to stats item
    class: Option<String>,
    /// Color scheme for stats item
    color_scheme: Option<StatsColorScheme>,
}

#[component]
pub fn StatsItem(props: StatsItemProps) -> Element {
    let class = props.class.unwrap_or_default();
    let color_scheme = props.color_scheme;

    // Build CSS classes
    let mut classes = vec!["stat".to_string()];
    
    if let Some(color) = color_scheme {
        classes.push(color.to_string());
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

#[derive(Props, Clone, PartialEq)]
pub struct StatsTitleProps {
    /// The content to display inside stats title
    children: Element,
    /// Optional ID for stats title element
    id: Option<String>,
    /// Additional CSS classes to apply to stats title
    class: Option<String>,
}

#[component]
pub fn StatsTitle(props: StatsTitleProps) -> Element {
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["stat-title".to_string()];
    
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
pub struct StatsValueProps {
    /// The content to display inside stats value
    children: Element,
    /// Optional ID for stats value element
    id: Option<String>,
    /// Additional CSS classes to apply to stats value
    class: Option<String>,
}

#[component]
pub fn StatsValue(props: StatsValueProps) -> Element {
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["stat-value".to_string()];
    
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
pub struct StatsDescriptionProps {
    /// The content to display inside stats description
    children: Element,
    /// Optional ID for stats description element
    id: Option<String>,
    /// Additional CSS classes to apply to stats description
    class: Option<String>,
}

#[component]
pub fn StatsDescription(props: StatsDescriptionProps) -> Element {
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["stat-desc".to_string()];
    
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
fn test_stats_basic() {
    let props = StatsProps {
        children: rsx!(
            StatsItem {
                StatsTitle { children: rsx!("Total Users") }
                StatsValue { children: rsx!("10,543") }
                StatsDescription { children: rsx!("+12% from last month") }
            }
        ),
        id: None,
        class: None,
        color_scheme: None,
        size: None,
    };

    let result = dioxus_ssr::render_element(Stats(props));
    assert!(result.contains("stats"));
}

#[test]
fn test_stats_item() {
    let props = StatsItemProps {
        children: rsx!(
            StatsTitle { children: rsx!("Title") }
            StatsValue { children: rsx!("100") }
            StatsDescription { children: rsx!("Description") }
        ),
        id: None,
        class: None,
        color_scheme: None,
    };

    let result = dioxus_ssr::render_element(StatsItem(props));
    assert!(result.contains("stat"));
}

#[test]
fn test_stats_with_color_scheme() {
    let props = StatsProps {
        children: rsx!(StatsItem { children: rsx!(StatsValue { children: rsx!("100") }) }),
        id: None,
        class: None,
        color_scheme: Some(StatsColorScheme::Primary),
        size: None,
    };

    let result = dioxus_ssr::render_element(Stats(props));
    assert!(result.contains("stats-primary"));
}

#[test]
fn test_stats_with_size() {
    let props = StatsProps {
        children: rsx!(StatsItem { children: rsx!(StatsValue { children: rsx!("100") }) }),
        id: None,
        class: None,
        color_scheme: None,
        size: Some(StatsSize::Large),
    };

    let result = dioxus_ssr::render_element(Stats(props));
    assert!(result.contains("stats-lg"));
}

#[test]
fn test_stats_custom_class() {
    let props = StatsProps {
        children: rsx!(StatsItem { children: rsx!(StatsValue { children: rsx!("100") }) }),
        id: None,
        class: Some("custom-class".to_string()),
        color_scheme: None,
        size: None,
    };

    let result = dioxus_ssr::render_element(Stats(props));
    assert!(result.contains("stats") && result.contains("custom-class"));
}

#[test]
fn test_stats_with_id() {
    let props = StatsProps {
        children: rsx!(StatsItem { children: rsx!(StatsValue { children: rsx!("100") }) }),
        id: Some("test-stats".to_string()),
        class: None,
        color_scheme: None,
        size: None,
    };

    let result = dioxus_ssr::render_element(Stats(props));
    assert!(result.contains(r#"id="test-stats""#));
}
