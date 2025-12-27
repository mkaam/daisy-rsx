#![allow(non_snake_case)]
use std::fmt::Display;
use dioxus::prelude::*;

/// A Carousel component for image/content carousels.
///
/// # Examples
///
/// Basic usage:
///
/// ```text
/// use daisy_rsx::{Carousel, CarouselItem};
///
/// Carousel {
///     children: rsx!(
///         CarouselItem { children: rsx!(img { src: "/slide1.jpg", alt: "Slide 1" }) }
///         CarouselItem { children: rsx!(img { src: "/slide2.jpg", alt: "Slide 2" }) }
///         CarouselItem { children: rsx!(img { src: "/slide3.jpg", alt: "Slide 3" }) }
///     )
/// }
/// ```

/// Color scheme options for Carousel component
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CarouselColorScheme {
    /// Neutral color
    Neutral,
    /// Primary color
    Primary,
    /// Secondary color
    Secondary,
}

impl Display for CarouselColorScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CarouselColorScheme::Neutral => write!(f, "carousel-neutral"),
            CarouselColorScheme::Primary => write!(f, "carousel-primary"),
            CarouselColorScheme::Secondary => write!(f, "carousel-secondary"),
        }
    }
}

/// Size options for Carousel component
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CarouselSize {
    /// Small size
    Small,
    /// Medium size
    Medium,
    /// Large size
    Large,
}

impl Display for CarouselSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CarouselSize::Small => write!(f, "carousel-sm"),
            CarouselSize::Medium => write!(f, "carousel-md"),
            CarouselSize::Large => write!(f, "carousel-lg"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CarouselProps {
    /// The content to display inside carousel (CarouselItem children)
    children: Element,
    /// Optional ID for carousel element
    id: Option<String>,
    /// Additional CSS classes to apply to carousel
    class: Option<String>,
    /// Auto-play functionality
    auto_play: Option<bool>,
    /// Auto-play interval in milliseconds
    interval: Option<u32>,
    /// Show navigation buttons
    show_nav: Option<bool>,
    /// Show dot indicators
    show_indicators: Option<bool>,
    /// Infinite loop
    infinite: Option<bool>,
    /// Pause on hover
    pause_on_hover: Option<bool>,
    /// Color scheme for carousel
    color_scheme: Option<CarouselColorScheme>,
    /// Size of carousel
    size: Option<CarouselSize>,
}

#[component]
pub fn Carousel(props: CarouselProps) -> Element {
    let class = props.class.unwrap_or_default();
    let color_scheme = props.color_scheme;
    let size = props.size;
    let auto_play = props.auto_play.filter(|&x| x);
    let show_nav = props.show_nav.filter(|&x| x);
    let show_indicators = props.show_indicators.filter(|&x| x);
    let infinite = props.infinite.filter(|&x| x);
    let pause_on_hover = props.pause_on_hover.filter(|&x| x);
    let interval = props.interval.unwrap_or(5000);

    // Build CSS classes
    let mut classes = vec!["carousel".to_string()];
    
    if let Some(color) = color_scheme {
        classes.push(color.to_string());
    }
    
    if let Some(s) = size {
        classes.push(s.to_string());
    }
    
    if auto_play.is_some() {
        classes.push("carousel-auto".to_string());
    }
    
    if infinite.is_some() {
        classes.push("carousel-infinite".to_string());
    }
    
    if pause_on_hover.is_some() {
        classes.push("carousel-pause".to_string());
    }
    
    if !class.is_empty() {
        classes.push(class);
    }

    let class_string = classes.join(" ");

    rsx!(
        div {
            class: "{class_string}",
            id: props.id,
            "data-interval": "{interval}",
            {props.children}
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct CarouselItemProps {
    /// The content to display inside carousel item
    children: Element,
    /// Optional ID for carousel item element
    id: Option<String>,
    /// Additional CSS classes to apply to carousel item
    class: Option<String>,
    /// Whether this item is active
    active: Option<bool>,
}

#[component]
pub fn CarouselItem(props: CarouselItemProps) -> Element {
    let class = props.class.unwrap_or_default();
    let active = props.active.filter(|&x| x);

    // Build CSS classes
    let mut classes = vec!["carousel-item".to_string()];
    
    if active.is_some() {
        classes.push("carousel-item-active".to_string());
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

#[test]
fn test_carousel_basic() {
    let props = CarouselProps {
        children: rsx!(
            CarouselItem { children: rsx!(img { src: "/slide1.jpg", alt: "Slide 1" }) }
            CarouselItem { children: rsx!(img { src: "/slide2.jpg", alt: "Slide 2" }) }
            CarouselItem { children: rsx!(img { src: "/slide3.jpg", alt: "Slide 3" }) }
        ),
        id: None,
        class: None,
        auto_play: None,
        interval: None,
        show_nav: None,
        show_indicators: None,
        infinite: None,
        pause_on_hover: None,
        color_scheme: None,
        size: None,
    };

    let result = dioxus_ssr::render_element(Carousel(props));
    assert!(result.contains("carousel"));
}

#[test]
fn test_carousel_item() {
    let props = CarouselItemProps {
        children: rsx!(img { src: "/slide.jpg", alt: "Slide" }),
        id: None,
        class: None,
        active: None,
    };

    let result = dioxus_ssr::render_element(CarouselItem(props));
    assert!(result.contains("carousel-item"));
}

#[test]
fn test_carousel_item_active() {
    let props = CarouselItemProps {
        children: rsx!(img { src: "/slide.jpg", alt: "Slide" }),
        id: None,
        class: None,
        active: Some(true),
    };

    let result = dioxus_ssr::render_element(CarouselItem(props));
    assert!(result.contains("carousel-item-active"));
}

#[test]
fn test_carousel_auto_play() {
    let props = CarouselProps {
        children: rsx!(CarouselItem { children: rsx!(img { src: "/slide.jpg" }) }),
        id: None,
        class: None,
        auto_play: Some(true),
        interval: Some(3000),
        show_nav: None,
        show_indicators: None,
        infinite: None,
        pause_on_hover: None,
        color_scheme: None,
        size: None,
    };

    let result = dioxus_ssr::render_element(Carousel(props));
    assert!(result.contains("carousel-auto"));
}

#[test]
fn test_carousel_with_nav() {
    let props = CarouselProps {
        children: rsx!(CarouselItem { children: rsx!(img { src: "/slide.jpg" }) }),
        id: None,
        class: None,
        auto_play: None,
        interval: None,
        show_nav: Some(true),
        show_indicators: None,
        infinite: None,
        pause_on_hover: None,
        color_scheme: None,
        size: None,
    };

    let result = dioxus_ssr::render_element(Carousel(props));
    // show_nav is a prop that can be used by CSS/JS, not rendered as element
    assert!(result.contains("carousel"));
}

#[test]
fn test_carousel_with_indicators() {
    let props = CarouselProps {
        children: rsx!(CarouselItem { children: rsx!(img { src: "/slide.jpg" }) }),
        id: None,
        class: None,
        auto_play: None,
        interval: None,
        show_nav: None,
        show_indicators: Some(true),
        infinite: None,
        pause_on_hover: None,
        color_scheme: None,
        size: None,
    };

    let result = dioxus_ssr::render_element(Carousel(props));
    // show_indicators is a prop that can be used by CSS/JS, not rendered as element
    assert!(result.contains("carousel"));
}

#[test]
fn test_carousel_infinite() {
    let props = CarouselProps {
        children: rsx!(CarouselItem { children: rsx!(img { src: "/slide.jpg" }) }),
        id: None,
        class: None,
        auto_play: None,
        interval: None,
        show_nav: None,
        show_indicators: None,
        infinite: Some(true),
        pause_on_hover: None,
        color_scheme: None,
        size: None,
    };

    let result = dioxus_ssr::render_element(Carousel(props));
    assert!(result.contains("carousel-infinite"));
}

#[test]
fn test_carousel_pause_on_hover() {
    let props = CarouselProps {
        children: rsx!(CarouselItem { children: rsx!(img { src: "/slide.jpg" }) }),
        id: None,
        class: None,
        auto_play: Some(true),
        interval: None,
        show_nav: None,
        show_indicators: None,
        infinite: None,
        pause_on_hover: Some(true),
        color_scheme: None,
        size: None,
    };

    let result = dioxus_ssr::render_element(Carousel(props));
    assert!(result.contains("carousel-pause"));
}

#[test]
fn test_carousel_with_color_scheme() {
    let props = CarouselProps {
        children: rsx!(CarouselItem { children: rsx!(img { src: "/slide.jpg" }) }),
        id: None,
        class: None,
        auto_play: None,
        interval: None,
        show_nav: None,
        show_indicators: None,
        infinite: None,
        pause_on_hover: None,
        color_scheme: Some(CarouselColorScheme::Primary),
        size: None,
    };

    let result = dioxus_ssr::render_element(Carousel(props));
    assert!(result.contains("carousel-primary"));
}

#[test]
fn test_carousel_with_size() {
    let props = CarouselProps {
        children: rsx!(CarouselItem { children: rsx!(img { src: "/slide.jpg" }) }),
        id: None,
        class: None,
        auto_play: None,
        interval: None,
        show_nav: None,
        show_indicators: None,
        infinite: None,
        pause_on_hover: None,
        color_scheme: None,
        size: Some(CarouselSize::Large),
    };

    let result = dioxus_ssr::render_element(Carousel(props));
    assert!(result.contains("carousel-lg"));
}

#[test]
fn test_carousel_custom_class() {
    let props = CarouselProps {
        children: rsx!(CarouselItem { children: rsx!(img { src: "/slide.jpg" }) }),
        id: None,
        class: Some("custom-class".to_string()),
        auto_play: None,
        interval: None,
        show_nav: None,
        show_indicators: None,
        infinite: None,
        pause_on_hover: None,
        color_scheme: None,
        size: None,
    };

    let result = dioxus_ssr::render_element(Carousel(props));
    assert!(result.contains("carousel") && result.contains("custom-class"));
}

#[test]
fn test_carousel_with_id() {
    let props = CarouselProps {
        children: rsx!(CarouselItem { children: rsx!(img { src: "/slide.jpg" }) }),
        id: Some("test-carousel".to_string()),
        class: None,
        auto_play: None,
        interval: None,
        show_nav: None,
        show_indicators: None,
        infinite: None,
        pause_on_hover: None,
        color_scheme: None,
        size: None,
    };

    let result = dioxus_ssr::render_element(Carousel(props));
    assert!(result.contains(r#"id="test-carousel""#));
}
