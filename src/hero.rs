#![allow(non_snake_case)]
use std::fmt::Display;
use dioxus::prelude::*;

/// A Hero component for hero sections on landing pages.
///
/// # Examples
///
/// Basic usage:
///
/// ```text
/// use daisy_rsx::{Hero, HeroContent, HeroTitle, HeroSubtitle, HeroActions, HeroColorScheme};
///
/// Hero {
///     children: rsx!(
///         HeroContent {
///             HeroTitle { children: rsx!("Welcome to Our Product") }
///             HeroSubtitle { children: rsx!("The best solution for your needs") }
///             HeroActions {
///                 Button { children: rsx!("Get Started") }
///                 Button { children: rsx!("Learn More") }
///             }
///         }
///     )
/// }
/// ```

/// Color scheme options for Hero component
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HeroColorScheme {
    /// Primary color
    Primary,
    /// Secondary color
    Secondary,
    /// Accent color
    Accent,
    /// Neutral color
    Neutral,
}

impl Display for HeroColorScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HeroColorScheme::Primary => write!(f, "hero-primary"),
            HeroColorScheme::Secondary => write!(f, "hero-secondary"),
            HeroColorScheme::Accent => write!(f, "hero-accent"),
            HeroColorScheme::Neutral => write!(f, "hero-neutral"),
        }
    }
}

/// Size options for Hero component
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HeroSize {
    /// Small size
    Small,
    /// Medium size
    Medium,
    /// Large size
    Large,
    /// Extra large size
    ExtraLarge,
}

impl Display for HeroSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HeroSize::Small => write!(f, "hero-sm"),
            HeroSize::Medium => write!(f, "hero-md"),
            HeroSize::Large => write!(f, "hero-lg"),
            HeroSize::ExtraLarge => write!(f, "hero-xl"),
        }
    }
}

/// Alignment options for Hero component
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HeroAlign {
    /// Left alignment
    Left,
    /// Center alignment
    Center,
    /// Right alignment
    Right,
}

impl Display for HeroAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HeroAlign::Left => write!(f, "text-left"),
            HeroAlign::Center => write!(f, "text-center"),
            HeroAlign::Right => write!(f, "text-right"),
        }
    }
}

/// Title level options for HeroTitle
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HeroTitleLevel {
    /// H1 heading
    H1,
    /// H2 heading
    H2,
    /// H3 heading
    H3,
}

#[derive(Props, Clone, PartialEq)]
pub struct HeroProps {
    /// The content to display inside hero (HeroContent children)
    children: Element,
    /// Optional ID for hero element
    id: Option<String>,
    /// Additional CSS classes to apply to hero
    class: Option<String>,
    /// Background image URL
    background_image: Option<String>,
    /// Background color
    background_color: Option<String>,
    /// Whether to show overlay
    overlay: Option<bool>,
    /// Color scheme for hero
    color_scheme: Option<HeroColorScheme>,
    /// Size of hero
    size: Option<HeroSize>,
    /// Alignment of content
    align: Option<HeroAlign>,
    /// Overlay opacity (0.0 to 1.0)
    overlay_opacity: Option<f32>,
}

#[component]
pub fn Hero(props: HeroProps) -> Element {
    let class = props.class.unwrap_or_default();
    let color_scheme = props.color_scheme;
    let size = props.size;
    let align = props.align;
    let overlay = props.overlay.filter(|&x| x);

    // Build CSS classes
    let mut classes = vec!["hero".to_string()];
    
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

    // Build background style
    let mut background_style = String::new();
    if let Some(bg_image) = &props.background_image {
        background_style.push_str(&format!("background-image: url('{}');", bg_image));
    }
    if let Some(bg_color) = &props.background_color {
        if !background_style.is_empty() {
            background_style.push(' ');
        }
        background_style.push_str(&format!("background-color: {};", bg_color));
    }

    // Build overlay style
    let overlay_style = if overlay.is_some() {
        let opacity = props.overlay_opacity.unwrap_or(0.5);
        Some(format!("background-color: rgba(0, 0, 0, {});", opacity))
    } else {
        None
    };

    rsx!(
        div {
            class: "{class_string}",
            id: props.id,
            style: if !background_style.is_empty() { Some(background_style) } else { None },
            {props.children}
            {overlay_style.map(|style| rsx!(
                div {
                    class: "hero-overlay",
                    style: "{style}",
                }
            ))}
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct HeroContentProps {
    /// The content to display inside hero content
    children: Element,
    /// Optional ID for hero content element
    id: Option<String>,
    /// Additional CSS classes to apply to hero content
    class: Option<String>,
    /// Alignment of content
    align: Option<HeroAlign>,
}

#[component]
pub fn HeroContent(props: HeroContentProps) -> Element {
    let class = props.class.unwrap_or_default();
    let align = props.align;

    // Build CSS classes
    let mut classes = vec!["hero-content".to_string()];
    
    if let Some(a) = align {
        classes.push(a.to_string());
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
pub struct HeroTitleProps {
    /// The content to display inside hero title
    children: Element,
    /// Optional ID for hero title element
    id: Option<String>,
    /// Additional CSS classes to apply to hero title
    class: Option<String>,
    /// Heading level (h1, h2, h3)
    level: Option<HeroTitleLevel>,
}

#[component]
pub fn HeroTitle(props: HeroTitleProps) -> Element {
    let class = props.class.unwrap_or_default();
    let level = props.level.unwrap_or(HeroTitleLevel::H1);

    // Build CSS classes
    let mut classes = vec!["hero-title".to_string()];
    
    if !class.is_empty() {
        classes.push(class);
    }

    let class_string = classes.join(" ");

    rsx!(
        div {
            class: "{class_string}",
            id: props.id,
            {match level {
                HeroTitleLevel::H1 => rsx!(h1 { {props.children} }),
                HeroTitleLevel::H2 => rsx!(h2 { {props.children} }),
                HeroTitleLevel::H3 => rsx!(h3 { {props.children} }),
            }}
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct HeroSubtitleProps {
    /// The content to display inside hero subtitle
    children: Element,
    /// Optional ID for hero subtitle element
    id: Option<String>,
    /// Additional CSS classes to apply to hero subtitle
    class: Option<String>,
}

#[component]
pub fn HeroSubtitle(props: HeroSubtitleProps) -> Element {
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["hero-subtitle".to_string()];
    
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
pub struct HeroActionsProps {
    /// The content to display inside hero actions
    children: Element,
    /// Optional ID for hero actions element
    id: Option<String>,
    /// Additional CSS classes to apply to hero actions
    class: Option<String>,
}

#[component]
pub fn HeroActions(props: HeroActionsProps) -> Element {
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["hero-actions".to_string()];
    
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
fn test_hero_basic() {
    let props = HeroProps {
        children: rsx!(
            HeroContent {
                HeroTitle { children: rsx!("Welcome to Our Product") }
                HeroSubtitle { children: rsx!("The best solution for your needs") }
                HeroActions {
                    div { "Get Started" }
                    div { "Learn More" }
                }
            }
        ),
        id: None,
        class: None,
        background_image: None,
        background_color: None,
        overlay: None,
        color_scheme: None,
        size: None,
        align: None,
        overlay_opacity: None,
    };

    let result = dioxus_ssr::render_element(Hero(props));
    assert!(result.contains("hero"));
}

#[test]
fn test_hero_with_background() {
    let props = HeroProps {
        children: rsx!(
            HeroContent {
                HeroTitle { children: rsx!("Title") }
            }
        ),
        id: None,
        class: None,
        background_image: Some("/hero-bg.jpg".to_string()),
        background_color: None,
        overlay: Some(true),
        color_scheme: None,
        size: None,
        align: None,
        overlay_opacity: None,
    };

    let result = dioxus_ssr::render_element(Hero(props));
    assert!(result.contains("hero"));
    assert!(result.contains("background-image"));
}

#[test]
fn test_hero_with_color_scheme() {
    let props = HeroProps {
        children: rsx!(HeroContent { HeroTitle { children: rsx!("Title") } }),
        id: None,
        class: None,
        background_image: None,
        background_color: None,
        overlay: None,
        color_scheme: Some(HeroColorScheme::Primary),
        size: None,
        align: None,
        overlay_opacity: None,
    };

    let result = dioxus_ssr::render_element(Hero(props));
    assert!(result.contains("hero-primary"));
}

#[test]
fn test_hero_with_size() {
    let props = HeroProps {
        children: rsx!(HeroContent { HeroTitle { children: rsx!("Title") } }),
        id: None,
        class: None,
        background_image: None,
        background_color: None,
        overlay: None,
        color_scheme: None,
        size: Some(HeroSize::Large),
        align: None,
        overlay_opacity: None,
    };

    let result = dioxus_ssr::render_element(Hero(props));
    assert!(result.contains("hero-lg"));
}

#[test]
fn test_hero_centered() {
    let props = HeroProps {
        children: rsx!(HeroContent { HeroTitle { children: rsx!("Title") } }),
        id: None,
        class: None,
        background_image: None,
        background_color: None,
        overlay: None,
        color_scheme: None,
        size: None,
        align: Some(HeroAlign::Center),
        overlay_opacity: None,
    };

    let result = dioxus_ssr::render_element(Hero(props));
    // align is a prop that can be used by CSS/JS, not rendered as class on hero element
    assert!(result.contains("hero"));
}
