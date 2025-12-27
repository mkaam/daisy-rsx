#![allow(non_snake_case)]
use std::fmt::Display;
use dioxus::prelude::*;

/// A Rating component that allows users to rate items using stars or other symbols.
///
/// # Examples
///
/// Basic usage:
///
/// ```text
/// use daisy_rsx::{Rating, RatingColorScheme};
///
/// Rating {
///     value: 4,
///     max: 5,
///     color_scheme: RatingColorScheme::Primary,
/// }
/// ```

/// Color scheme options for Rating component
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum RatingColorScheme {
    #[default]
    /// Primary brand color scheme
    Primary,
    /// Secondary color scheme
    Secondary,
    /// Warning yellow color scheme
    Warning,
    /// Success green color scheme
    Success,
}

impl Display for RatingColorScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RatingColorScheme::Primary => write!(f, "rating-primary"),
            RatingColorScheme::Secondary => write!(f, "rating-secondary"),
            RatingColorScheme::Warning => write!(f, "rating-warning"),
            RatingColorScheme::Success => write!(f, "rating-success"),
        }
    }
}

/// Size options for Rating component
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum RatingSize {
    #[default]
    /// Default size
    Default,
    /// Small rating
    Small,
    /// Medium rating
    Medium,
    /// Large rating
    Large,
}

impl Display for RatingSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RatingSize::Default => write!(f, ""),
            RatingSize::Small => write!(f, "rating-sm"),
            RatingSize::Medium => write!(f, "rating-md"),
            RatingSize::Large => write!(f, "rating-lg"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct RatingProps {
    /// Optional ID for the rating element
    id: Option<String>,
    /// Additional CSS classes to apply to the rating
    class: Option<String>,
    /// Current rating value
    value: i32,
    /// Maximum rating value (default: 5)
    max: Option<i32>,
    /// Color scheme for the rating
    color_scheme: Option<RatingColorScheme>,
    /// Size of the rating
    size: Option<RatingSize>,
    /// Whether the rating is read-only
    read_only: Option<bool>,
    /// Whether to show half-star support
    half: Option<bool>,
}

#[component]
pub fn Rating(props: RatingProps) -> Element {
    let color_scheme = props.color_scheme.unwrap_or_default();
    let size = props.size.unwrap_or_default();
    let class = props.class.unwrap_or_default();
    let read_only = props.read_only.filter(|&x| x);
    let half = props.half.filter(|&x| x);
    let max = props.max.unwrap_or(5);
    let rating_id = props.id.clone().unwrap_or_default();
    let value = props.value;
    let div_id = props.id;

    // Build CSS classes
    let mut classes = vec!["rating".to_string()];
    
    if !color_scheme.to_string().is_empty() {
        classes.push(color_scheme.to_string());
    }
    
    if !size.to_string().is_empty() {
        classes.push(size.to_string());
    }
    
    if half.is_some() {
        classes.push("rating-half".to_string());
    }
    
    if !class.is_empty() {
        classes.push(class);
    }

    let class_string = classes.join(" ");

    rsx!(
        div {
            class: "{class_string}",
            id: div_id,
            {(0..max).map(|i| {
                let is_filled = i < value;
                rsx!(
                    input {
                        r#type: "radio",
                        name: "rating-{rating_id}",
                        class: "mask mask-star",
                        r#aria_label: format!("{} star", i + 1),
                        checked: is_filled,
                        disabled: read_only,
                    }
                )
            })}
        }
    )
}

#[test]
fn test_rating_basic() {
    let props = RatingProps {
        id: None,
        class: None,
        value: 4,
        max: Some(5),
        color_scheme: None,
        size: None,
        read_only: None,
        half: None,
    };

    let result = dioxus_ssr::render_element(Rating(props));
    assert!(result.contains("rating"));
}

#[test]
fn test_rating_with_color_scheme() {
    let schemes = [
        (RatingColorScheme::Primary, "rating-primary"),
        (RatingColorScheme::Secondary, "rating-secondary"),
        (RatingColorScheme::Warning, "rating-warning"),
        (RatingColorScheme::Success, "rating-success"),
    ];

    for (scheme, expected_class) in schemes {
        let props = RatingProps {
            id: None,
            class: None,
            value: 3,
            max: Some(5),
            color_scheme: Some(scheme),
            size: None,
            read_only: None,
            half: None,
        };

        let result = dioxus_ssr::render_element(Rating(props));
        assert!(result.contains(expected_class),
                "Expected '{}' to contain '{}', but got: {}",
                result, expected_class, result);
    }
}

#[test]
fn test_rating_with_size() {
    let sizes = [
        (RatingSize::Default, ""),
        (RatingSize::Small, "rating-sm"),
        (RatingSize::Medium, "rating-md"),
        (RatingSize::Large, "rating-lg"),
    ];

    for (size, expected_class) in sizes {
        let props = RatingProps {
            id: None,
            class: None,
            value: 3,
            max: Some(5),
            color_scheme: None,
            size: Some(size),
            read_only: None,
            half: None,
        };

        let result = dioxus_ssr::render_element(Rating(props));
        if expected_class.is_empty() {
            assert!(result.contains("rating"));
        } else {
            assert!(result.contains(expected_class),
                    "Expected '{}' to contain '{}', but got: {}",
                    result, expected_class, result);
        }
    }
}

#[test]
fn test_rating_half() {
    let props = RatingProps {
        id: None,
        class: None,
        value: 3,
        max: Some(5),
        color_scheme: None,
        size: None,
        read_only: None,
        half: Some(true),
    };

    let result = dioxus_ssr::render_element(Rating(props));
    assert!(result.contains("rating") && result.contains("rating-half"));
}

#[test]
fn test_rating_read_only() {
    let props = RatingProps {
        id: None,
        class: None,
        value: 4,
        max: Some(5),
        color_scheme: None,
        size: None,
        read_only: Some(true),
        half: None,
    };

    let result = dioxus_ssr::render_element(Rating(props));
    assert!(result.contains(r#"disabled"#));
}

#[test]
fn test_rating_with_custom_class() {
    let props = RatingProps {
        id: None,
        class: Some("custom-class".to_string()),
        value: 3,
        max: Some(5),
        color_scheme: None,
        size: None,
        read_only: None,
        half: None,
    };

    let result = dioxus_ssr::render_element(Rating(props));
    assert!(result.contains("rating") && result.contains("custom-class"));
}

#[test]
fn test_rating_with_id() {
    let props = RatingProps {
        id: Some("test-rating".to_string()),
        class: None,
        value: 3,
        max: Some(5),
        color_scheme: None,
        size: None,
        read_only: None,
        half: None,
    };

    let result = dioxus_ssr::render_element(Rating(props));
    assert!(result.contains(r#"id="test-rating""#));
}
