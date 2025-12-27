#![allow(non_snake_case)]
use std::fmt::Display;
use dioxus::prelude::*;

/// A Skeleton component that displays placeholder content while data is loading.
///
/// # Examples
///
/// Basic usage:
///
/// ```text
/// use daisy_rsx::{Skeleton, SkeletonVariant};
///
/// Skeleton {
///     variant: SkeletonVariant::Text,
/// }
/// ```

/// Variant options for Skeleton component
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum SkeletonVariant {
    #[default]
    /// Text variant
    Text,
    /// Avatar variant
    Avatar,
    /// Image variant
    Image,
    /// Card variant
    Card,
}

impl Display for SkeletonVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SkeletonVariant::Text => write!(f, "skeleton-text"),
            SkeletonVariant::Avatar => write!(f, "skeleton-avatar"),
            SkeletonVariant::Image => write!(f, "skeleton-image"),
            SkeletonVariant::Card => write!(f, "skeleton-card"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SkeletonProps {
    /// Optional ID for the skeleton element
    id: Option<String>,
    /// Additional CSS classes to apply to the skeleton
    class: Option<String>,
    /// Variant for the skeleton
    variant: Option<SkeletonVariant>,
}

#[component]
pub fn Skeleton(props: SkeletonProps) -> Element {
    let variant = props.variant.unwrap_or_default();
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["skeleton".to_string()];
    classes.push(variant.to_string());
    
    if !class.is_empty() {
        classes.push(class);
    }

    let class_string = classes.join(" ");

    rsx!(
        div {
            class: "{class_string}",
            id: props.id,
        }
    )
}

#[test]
fn test_skeleton_basic() {
    let props = SkeletonProps {
        id: None,
        class: None,
        variant: None,
    };

    let result = dioxus_ssr::render_element(Skeleton(props));
    assert!(result.contains(r#"class="skeleton skeleton-text""#));
}

#[test]
fn test_skeleton_avatar() {
    let props = SkeletonProps {
        id: None,
        class: None,
        variant: Some(SkeletonVariant::Avatar),
    };

    let result = dioxus_ssr::render_element(Skeleton(props));
    assert!(result.contains(r#"class="skeleton skeleton-avatar""#));
}

#[test]
fn test_skeleton_with_custom_class() {
    let props = SkeletonProps {
        id: None,
        class: Some("custom-class".to_string()),
        variant: None,
    };

    let result = dioxus_ssr::render_element(Skeleton(props));
    assert!(result.contains(r#"class="skeleton skeleton-text custom-class""#));
}

#[test]
fn test_skeleton_with_id() {
    let props = SkeletonProps {
        id: Some("test-skeleton".to_string()),
        class: None,
        variant: None,
    };

    let result = dioxus_ssr::render_element(Skeleton(props));
    assert!(result.contains(r#"id="test-skeleton""#));
}
