#![allow(non_snake_case)]
use std::fmt::Display;
use dioxus::prelude::*;

/// An Artboard component for device mockup frames.
///
/// # Examples
///
/// Basic usage:
///
/// ```text
/// use daisy_rsx::{Artboard, ArtboardDevice};
///
/// Artboard {
///     device: Some(ArtboardDevice::Phone),
///     children: rsx!(
///         ArtboardContent {
///             div { "Phone content here" }
///         }
///     )
/// }
/// ```

/// Device type options for Artboard component
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ArtboardDevice {
    /// Phone device frame
    Phone,
    /// Tablet device frame
    Tablet,
    /// Laptop device frame
    Laptop,
    /// Desktop device frame
    Desktop,
}

impl Display for ArtboardDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArtboardDevice::Phone => write!(f, "artboard-phone"),
            ArtboardDevice::Tablet => write!(f, "artboard-tablet"),
            ArtboardDevice::Laptop => write!(f, "artboard-laptop"),
            ArtboardDevice::Desktop => write!(f, "artboard-desktop"),
        }
    }
}

/// Border radius options for Artboard component
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ArtboardBorderRadius {
    /// No border radius
    None,
    /// Small border radius
    Small,
    /// Medium border radius
    Medium,
    /// Large border radius
    Large,
    /// Extra large border radius
    ExtraLarge,
}

impl Display for ArtboardBorderRadius {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArtboardBorderRadius::None => write!(f, ""),
            ArtboardBorderRadius::Small => write!(f, "rounded-sm"),
            ArtboardBorderRadius::Medium => write!(f, "rounded-md"),
            ArtboardBorderRadius::Large => write!(f, "rounded-lg"),
            ArtboardBorderRadius::ExtraLarge => write!(f, "rounded-xl"),
        }
    }
}

/// Shadow options for Artboard component
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ArtboardShadow {
    /// No shadow
    None,
    /// Small shadow
    Small,
    /// Medium shadow
    Medium,
    /// Large shadow
    Large,
}

impl Display for ArtboardShadow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArtboardShadow::None => write!(f, ""),
            ArtboardShadow::Small => write!(f, "shadow-sm"),
            ArtboardShadow::Medium => write!(f, "shadow-md"),
            ArtboardShadow::Large => write!(f, "shadow-lg"),
        }
    }
}

/// Color scheme options for Artboard component
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ArtboardColorScheme {
    /// Neutral color
    Neutral,
    /// Primary color
    Primary,
    /// Secondary color
    Secondary,
}

impl Display for ArtboardColorScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArtboardColorScheme::Neutral => write!(f, "artboard-neutral"),
            ArtboardColorScheme::Primary => write!(f, "artboard-primary"),
            ArtboardColorScheme::Secondary => write!(f, "artboard-secondary"),
        }
    }
}

/// Size options for Artboard component
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ArtboardSize {
    /// Small size
    Small,
    /// Medium size
    Medium,
    /// Large size
    Large,
}

impl Display for ArtboardSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArtboardSize::Small => write!(f, "artboard-sm"),
            ArtboardSize::Medium => write!(f, "artboard-md"),
            ArtboardSize::Large => write!(f, "artboard-lg"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ArtboardProps {
    /// The content to display inside artboard (ArtboardContent children)
    children: Element,
    /// Optional ID for artboard element
    id: Option<String>,
    /// Additional CSS classes to apply to artboard
    class: Option<String>,
    /// Device type
    device: Option<ArtboardDevice>,
    /// Border radius
    border_radius: Option<ArtboardBorderRadius>,
    /// Shadow effect
    shadow: Option<ArtboardShadow>,
    /// Color scheme
    color_scheme: Option<ArtboardColorScheme>,
    /// Size
    size: Option<ArtboardSize>,
}

#[component]
pub fn Artboard(props: ArtboardProps) -> Element {
    let class = props.class.unwrap_or_default();
    let device = props.device.unwrap_or(ArtboardDevice::Phone);
    let border_radius = props.border_radius;
    let shadow = props.shadow;
    let color_scheme = props.color_scheme;
    let size = props.size;

    // Build CSS classes
    let mut classes = vec!["artboard".to_string()];
    classes.push(device.to_string());
    
    if let Some(color) = color_scheme {
        classes.push(color.to_string());
    }
    
    if let Some(s) = size {
        classes.push(s.to_string());
    }
    
    if let Some(br) = border_radius {
        let br_class = br.to_string();
        if !br_class.is_empty() {
            classes.push(br_class);
        }
    }
    
    if let Some(sh) = shadow {
        let sh_class = sh.to_string();
        if !sh_class.is_empty() {
            classes.push(sh_class);
        }
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
pub struct ArtboardContentProps {
    /// The content to display inside artboard content
    children: Element,
    /// Optional ID for artboard content element
    id: Option<String>,
    /// Additional CSS classes to apply to artboard content
    class: Option<String>,
}

#[component]
pub fn ArtboardContent(props: ArtboardContentProps) -> Element {
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["artboard-content".to_string()];
    
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
fn test_artboard_basic() {
    let props = ArtboardProps {
        children: rsx!(
            ArtboardContent {
                div { "Content" }
            }
        ),
        id: None,
        class: None,
        device: None,
        border_radius: None,
        shadow: None,
        color_scheme: None,
        size: None,
    };

    let result = dioxus_ssr::render_element(Artboard(props));
    assert!(result.contains("artboard"));
}

#[test]
fn test_artboard_phone() {
    let props = ArtboardProps {
        children: rsx!(
            ArtboardContent {
                div { "Phone content" }
            }
        ),
        id: None,
        class: None,
        device: Some(ArtboardDevice::Phone),
        border_radius: None,
        shadow: None,
        color_scheme: None,
        size: None,
    };

    let result = dioxus_ssr::render_element(Artboard(props));
    assert!(result.contains("artboard-phone"));
}

#[test]
fn test_artboard_tablet() {
    let props = ArtboardProps {
        children: rsx!(
            ArtboardContent {
                div { "Tablet content" }
            }
        ),
        id: None,
        class: None,
        device: Some(ArtboardDevice::Tablet),
        border_radius: None,
        shadow: None,
        color_scheme: None,
        size: None,
    };

    let result = dioxus_ssr::render_element(Artboard(props));
    assert!(result.contains("artboard-tablet"));
}

#[test]
fn test_artboard_laptop() {
    let props = ArtboardProps {
        children: rsx!(
            ArtboardContent {
                div { "Laptop content" }
            }
        ),
        id: None,
        class: None,
        device: Some(ArtboardDevice::Laptop),
        border_radius: None,
        shadow: None,
        color_scheme: None,
        size: None,
    };

    let result = dioxus_ssr::render_element(Artboard(props));
    assert!(result.contains("artboard-laptop"));
}

#[test]
fn test_artboard_desktop() {
    let props = ArtboardProps {
        children: rsx!(
            ArtboardContent {
                div { "Desktop content" }
            }
        ),
        id: None,
        class: None,
        device: Some(ArtboardDevice::Desktop),
        border_radius: None,
        shadow: None,
        color_scheme: None,
        size: None,
    };

    let result = dioxus_ssr::render_element(Artboard(props));
    assert!(result.contains("artboard-desktop"));
}

#[test]
fn test_artboard_with_shadow() {
    let props = ArtboardProps {
        children: rsx!(
            ArtboardContent {
                div { "Content" }
            }
        ),
        id: None,
        class: None,
        device: None,
        border_radius: None,
        shadow: Some(ArtboardShadow::Large),
        color_scheme: None,
        size: None,
    };

    let result = dioxus_ssr::render_element(Artboard(props));
    assert!(result.contains("shadow-lg"));
}

#[test]
fn test_artboard_custom_class() {
    let props = ArtboardProps {
        children: rsx!(
            ArtboardContent {
                div { "Content" }
            }
        ),
        id: None,
        class: Some("custom-class".to_string()),
        device: None,
        border_radius: None,
        shadow: None,
        color_scheme: None,
        size: None,
    };

    let result = dioxus_ssr::render_element(Artboard(props));
    assert!(result.contains("artboard") && result.contains("custom-class"));
}

#[test]
fn test_artboard_with_id() {
    let props = ArtboardProps {
        children: rsx!(
            ArtboardContent {
                div { "Content" }
            }
        ),
        id: Some("test-artboard".to_string()),
        class: None,
        device: None,
        border_radius: None,
        shadow: None,
        color_scheme: None,
        size: None,
    };

    let result = dioxus_ssr::render_element(Artboard(props));
    assert!(result.contains(r#"id="test-artboard""#));
}
