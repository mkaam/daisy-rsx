#![allow(non_snake_case)]
use std::fmt::Display;
use dioxus::prelude::*;

/// A Swap component that allows swapping between two elements on hover or click.
///
/// # Examples
///
/// Basic usage:
///
/// ```text
/// use daisy_rsx::{Swap, SwapAnimation, SwapSize};
///
/// Swap {
///     animation: SwapAnimation::Flip,
///     SwapItem { children: rsx!("Element 1") }
///     SwapItem { children: rsx!("Element 2") }
/// }
/// ```

/// Animation options for Swap component
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum SwapAnimation {
    #[default]
    /// Fade animation
    Fade,
    /// Flip animation
    Flip,
    /// Rotate animation
    Rotate,
}

impl Display for SwapAnimation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SwapAnimation::Fade => write!(f, "swap-fade"),
            SwapAnimation::Flip => write!(f, "swap-flip"),
            SwapAnimation::Rotate => write!(f, "swap-rotate"),
        }
    }
}

/// Size options for Swap component
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum SwapSize {
    #[default]
    /// Default size
    Default,
    /// Small swap
    Small,
    /// Medium swap
    Medium,
    /// Large swap
    Large,
}

impl Display for SwapSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SwapSize::Default => write!(f, ""),
            SwapSize::Small => write!(f, "swap-sm"),
            SwapSize::Medium => write!(f, "swap-md"),
            SwapSize::Large => write!(f, "swap-lg"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SwapProps {
    /// The content to display inside swap (must be exactly 2 SwapItem children)
    children: Element,
    /// Optional ID for swap element
    id: Option<String>,
    /// Additional CSS classes to apply to swap
    class: Option<String>,
    /// Animation for swap
    animation: Option<SwapAnimation>,
    /// Size of swap
    size: Option<SwapSize>,
    /// Whether to activate on click instead of hover
    click: Option<bool>,
}

#[component]
pub fn Swap(props: SwapProps) -> Element {
    let animation = props.animation.unwrap_or_default();
    let size = props.size.unwrap_or_default();
    let class = props.class.unwrap_or_default();
    let click = props.click.filter(|&x| x);

    // Build CSS classes
    let mut classes = vec!["swap".to_string()];
    
    if !animation.to_string().is_empty() {
        classes.push(animation.to_string());
    }
    
    if click.is_some() {
        classes.push("swap-active".to_string());
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
            id: props.id,
            {props.children}
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct SwapItemProps {
    /// The content to display inside swap item
    children: Element,
    /// Additional CSS classes to apply to swap item
    class: Option<String>,
}

#[component]
pub fn SwapItem(props: SwapItemProps) -> Element {
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["swap-item".to_string()];
    
    if !class.is_empty() {
        classes.push(class);
    }

    let class_string = classes.join(" ");

    rsx!(
        div {
            class: "{class_string}",
            {props.children}
        }
    )
}

#[test]
fn test_swap_basic() {
    let props = SwapProps {
        children: rsx!(
            SwapItem { children: rsx!("Element 1") }
            SwapItem { children: rsx!("Element 2") }
        ),
        id: None,
        class: None,
        animation: None,
        size: None,
        click: None,
    };

    let result = dioxus_ssr::render_element(Swap(props));
    assert!(result.contains("swap"));
}

#[test]
fn test_swap_with_animation() {
    let props = SwapProps {
        children: rsx!(
            SwapItem { children: rsx!("Element 1") }
            SwapItem { children: rsx!("Element 2") }
        ),
        id: None,
        class: None,
        animation: Some(SwapAnimation::Flip),
        size: None,
        click: None,
    };

    let result = dioxus_ssr::render_element(Swap(props));
    assert!(result.contains(r#"class="swap swap-flip""#));
}

#[test]
fn test_swap_click() {
    let props = SwapProps {
        children: rsx!(
            SwapItem { children: rsx!("Element 1") }
            SwapItem { children: rsx!("Element 2") }
        ),
        id: None,
        class: None,
        animation: None,
        size: None,
        click: Some(true),
    };

    let result = dioxus_ssr::render_element(Swap(props));
    assert!(result.contains("swap") && result.contains("swap-active"));
}

#[test]
fn test_swap_with_size() {
    let sizes = [
        (SwapSize::Default, ""),
        (SwapSize::Small, "swap-sm"),
        (SwapSize::Medium, "swap-md"),
        (SwapSize::Large, "swap-lg"),
    ];

    for (size, expected_class) in sizes {
        let props = SwapProps {
            children: rsx!(
                SwapItem { children: rsx!("Element 1") }
                SwapItem { children: rsx!("Element 2") }
            ),
            id: None,
            class: None,
            animation: None,
            size: Some(size),
            click: None,
        };

        let result = dioxus_ssr::render_element(Swap(props));
        if expected_class.is_empty() {
            assert!(result.contains("swap"));
        } else {
            assert!(result.contains(expected_class),
                    "Expected '{}' to contain '{}', but got: {}",
                    result, expected_class, result);
        }
    }
}

#[test]
fn test_swap_with_custom_class() {
    let props = SwapProps {
        children: rsx!(
            SwapItem { children: rsx!("Element 1") }
            SwapItem { children: rsx!("Element 2") }
        ),
        id: None,
        class: Some("custom-class".to_string()),
        animation: None,
        size: None,
        click: None,
    };

    let result = dioxus_ssr::render_element(Swap(props));
    assert!(result.contains("swap") && result.contains("custom-class"));
}

#[test]
fn test_swap_with_id() {
    let props = SwapProps {
        children: rsx!(
            SwapItem { children: rsx!("Element 1") }
            SwapItem { children: rsx!("Element 2") }
        ),
        id: Some("test-swap".to_string()),
        class: None,
        animation: None,
        size: None,
        click: None,
    };

    let result = dioxus_ssr::render_element(Swap(props));
    assert!(result.contains(r#"id="test-swap""#));
}
