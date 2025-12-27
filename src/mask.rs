#![allow(non_snake_case)]
use std::fmt::Display;
use dioxus::prelude::*;

/// A Mask component that applies shape masks to images and other elements.
///
/// # Examples
///
/// Basic usage:
///
/// ```text
/// use daisy_rsx::{Mask, MaskVariant};
///
/// Mask {
///     variant: MaskVariant::Circle,
///     img { src: "avatar.jpg", alt: "Avatar" }
/// }
/// ```
///
/// With custom dimensions:
///
/// ```text
/// Mask {
///     variant: MaskVariant::Square,
///     width: "100px",
///     height: "100px",
///     img { src: "photo.jpg", alt: "Photo" }
/// }
/// ```

/// Shape variant options for Mask component
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum MaskVariant {
    #[default]
    /// No mask (default)
    None,
    /// Circular mask
    Circle,
    /// Square mask
    Square,
    /// Rounded square (squircle) mask
    Squircle,
    /// Hexagonal mask
    Hexagon,
    /// Triangular mask
    Triangle,
    /// Diamond mask
    Diamond,
}

impl Display for MaskVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MaskVariant::None => write!(f, ""),
            MaskVariant::Circle => write!(f, "mask-circle"),
            MaskVariant::Square => write!(f, "mask-square"),
            MaskVariant::Squircle => write!(f, "mask-squircle"),
            MaskVariant::Hexagon => write!(f, "mask-hexagon"),
            MaskVariant::Triangle => write!(f, "mask-triangle"),
            MaskVariant::Diamond => write!(f, "mask-diamond"),
        }
    }
}

/// Size options for Mask component
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum MaskSize {
    #[default]
    /// Default size
    Default,
    /// Small size
    Small,
    /// Medium size
    Medium,
    /// Large size
    Large,
}

impl Display for MaskSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MaskSize::Default => write!(f, ""),
            MaskSize::Small => write!(f, "mask-sm"),
            MaskSize::Medium => write!(f, "mask-md"),
            MaskSize::Large => write!(f, "mask-lg"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct MaskProps {
    /// The content to display inside the mask
    children: Element,
    /// Optional ID for the mask element
    id: Option<String>,
    /// Additional CSS classes to apply to the mask
    class: Option<String>,
    /// Shape variant for the mask
    variant: Option<MaskVariant>,
    /// Size of the mask
    size: Option<MaskSize>,
    /// Custom width for the mask
    width: Option<String>,
    /// Custom height for the mask
    height: Option<String>,
}

#[component]
pub fn Mask(props: MaskProps) -> Element {
    let variant = props.variant.unwrap_or_default();
    let size = props.size.unwrap_or_default();
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["mask".to_string()];
    
    if !variant.to_string().is_empty() {
        classes.push(variant.to_string());
    }
    
    if !size.to_string().is_empty() {
        classes.push(size.to_string());
    }
    
    if !class.is_empty() {
        classes.push(class);
    }

    let class_string = classes.join(" ");

    // Build style attribute for custom dimensions
    let mut style_parts = Vec::new();
    if let Some(width) = &props.width {
        style_parts.push(format!("width: {}", width));
    }
    if let Some(height) = &props.height {
        style_parts.push(format!("height: {}", height));
    }
    let style = if !style_parts.is_empty() {
        Some(style_parts.join("; "))
    } else {
        None
    };

    rsx!(
        div {
            class: "{class_string}",
            id: props.id,
            style: style,
            {props.children}
        }
    )
}

#[test]
fn test_mask_basic() {
    let props = MaskProps {
        children: rsx!("Content"),
        id: None,
        class: None,
        variant: None,
        size: None,
        width: None,
        height: None,
    };

    let result = dioxus_ssr::render_element(Mask(props));
    assert!(result.contains(r#"class="mask""#));
}

#[test]
fn test_mask_circle() {
    let props = MaskProps {
        children: rsx!("Content"),
        id: None,
        class: None,
        variant: Some(MaskVariant::Circle),
        size: None,
        width: None,
        height: None,
    };

    let result = dioxus_ssr::render_element(Mask(props));
    assert!(result.contains(r#"class="mask mask-circle""#));
}

#[test]
fn test_mask_square() {
    let props = MaskProps {
        children: rsx!("Content"),
        id: None,
        class: None,
        variant: Some(MaskVariant::Square),
        size: None,
        width: None,
        height: None,
    };

    let result = dioxus_ssr::render_element(Mask(props));
    assert!(result.contains(r#"class="mask mask-square""#));
}

#[test]
fn test_mask_with_size() {
    let sizes = [
        (MaskSize::Default, ""),
        (MaskSize::Small, "mask-sm"),
        (MaskSize::Medium, "mask-md"),
        (MaskSize::Large, "mask-lg"),
    ];

    for (size, expected_class) in sizes {
        let props = MaskProps {
            children: rsx!("Content"),
            id: None,
            class: None,
            variant: None,
            size: Some(size),
            width: None,
            height: None,
        };

        let result = dioxus_ssr::render_element(Mask(props));
        if expected_class.is_empty() {
            assert!(result.contains(r#"class="mask""#));
        } else {
            assert!(result.contains(expected_class),
                    "Expected '{}' to contain '{}', but got: {}",
                    result, expected_class, result);
        }
    }
}

#[test]
fn test_mask_with_custom_dimensions() {
    let props = MaskProps {
        children: rsx!("Content"),
        id: None,
        class: None,
        variant: None,
        size: None,
        width: Some("100px".to_string()),
        height: Some("100px".to_string()),
    };

    let result = dioxus_ssr::render_element(Mask(props));
    assert!(result.contains(r#"style="width: 100px; height: 100px""#));
}

#[test]
fn test_mask_with_custom_class() {
    let props = MaskProps {
        children: rsx!("Content"),
        id: None,
        class: Some("custom-class".to_string()),
        variant: None,
        size: None,
        width: None,
        height: None,
    };

    let result = dioxus_ssr::render_element(Mask(props));
    assert!(result.contains(r#"class="mask custom-class""#));
}

#[test]
fn test_mask_with_id() {
    let props = MaskProps {
        children: rsx!("Content"),
        id: Some("test-mask".to_string()),
        class: None,
        variant: None,
        size: None,
        width: None,
        height: None,
    };

    let result = dioxus_ssr::render_element(Mask(props));
    assert!(result.contains(r#"id="test-mask""#));
}
