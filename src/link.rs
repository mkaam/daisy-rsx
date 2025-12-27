#![allow(non_snake_case)]
use std::fmt::Display;
use dioxus::prelude::*;

/// A Link component that renders styled anchor links with hover effects.
///
/// # Examples
///
/// Basic usage:
///
/// ```text
/// use daisy_rsx::{Link, LinkColorScheme};
///
/// Link {
///     href: "https://example.com",
///     color_scheme: LinkColorScheme::Primary,
///     "Click me"
/// }
/// ```
///
/// With target attribute:
///
/// ```text
/// Link {
///     href: "https://example.com",
///     target: "_blank",
///     color_scheme: LinkColorScheme::Secondary,
///     "Open in new tab"
/// }
/// ```

/// Color scheme options for Link component
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum LinkColorScheme {
    #[default]
    /// Neutral gray color scheme
    Neutral,
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

impl Display for LinkColorScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LinkColorScheme::Neutral => write!(f, "link-neutral"),
            LinkColorScheme::Primary => write!(f, "link-primary"),
            LinkColorScheme::Secondary => write!(f, "link-secondary"),
            LinkColorScheme::Accent => write!(f, "link-accent"),
            LinkColorScheme::Info => write!(f, "link-info"),
            LinkColorScheme::Success => write!(f, "link-success"),
            LinkColorScheme::Warning => write!(f, "link-warning"),
            LinkColorScheme::Error => write!(f, "link-error"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct LinkProps {
    /// The content to display inside the link
    children: Element,
    /// Optional ID for the link element
    id: Option<String>,
    /// The URL the link points to
    href: String,
    /// Target attribute for the link (e.g., "_blank" for new tab)
    target: Option<String>,
    /// Additional CSS classes to apply to the link
    class: Option<String>,
    /// Color scheme for the link
    color_scheme: Option<LinkColorScheme>,
    /// Whether to add rel="noopener noreferrer" for external links
    external: Option<bool>,
}

#[component]
pub fn Link(props: LinkProps) -> Element {
    let color_scheme = props.color_scheme.unwrap_or_default();
    let class = props.class.unwrap_or_default();
    let external = props.external.filter(|&x| x);

    // Build CSS classes
    let mut classes = vec!["link".to_string()];
    
    if !color_scheme.to_string().is_empty() {
        classes.push(color_scheme.to_string());
    }
    
    if !class.is_empty() {
        classes.push(class);
    }

    let class_string = classes.join(" ");

    // Build rel attribute for external links
    let rel = if external.is_some() && props.target.as_deref() == Some("_blank") {
        Some("noopener noreferrer".to_string())
    } else {
        None
    };

    rsx!(
        a {
            class: "{class_string}",
            id: props.id,
            href: "{props.href}",
            target: props.target,
            rel: rel,
            {props.children}
        }
    )
}

#[test]
fn test_link_basic() {
    let props = LinkProps {
        children: rsx!("Test Link"),
        id: None,
        href: "https://example.com".to_string(),
        target: None,
        class: None,
        color_scheme: None,
        external: None,
    };

    let result = dioxus_ssr::render_element(Link(props));
    assert!(result.contains(r#"class="link link-neutral""#));
    assert!(result.contains(r#"href="https://example.com""#));
    assert!(result.contains(">Test Link</a>"));
}

#[test]
fn test_link_with_color_scheme() {
    let schemes = [
        (LinkColorScheme::Neutral, "link-neutral"),
        (LinkColorScheme::Primary, "link-primary"),
        (LinkColorScheme::Secondary, "link-secondary"),
        (LinkColorScheme::Accent, "link-accent"),
        (LinkColorScheme::Info, "link-info"),
        (LinkColorScheme::Success, "link-success"),
        (LinkColorScheme::Warning, "link-warning"),
        (LinkColorScheme::Error, "link-error"),
    ];

    for (scheme, expected_class) in schemes {
        let props = LinkProps {
            children: rsx!("Test"),
            id: None,
            href: "https://example.com".to_string(),
            target: None,
            class: None,
            color_scheme: Some(scheme),
            external: None,
        };

        let result = dioxus_ssr::render_element(Link(props));
        assert!(result.contains(expected_class),
                "Expected '{}' to contain '{}', but got: {}",
                result, expected_class, result);
    }
}

#[test]
fn test_link_with_target() {
    let props = LinkProps {
        children: rsx!("Test Link"),
        id: None,
        href: "https://example.com".to_string(),
        target: Some("_blank".to_string()),
        class: None,
        color_scheme: None,
        external: None,
    };

    let result = dioxus_ssr::render_element(Link(props));
    assert!(result.contains(r#"target="_blank""#));
}

#[test]
fn test_link_external_with_rel() {
    let props = LinkProps {
        children: rsx!("Test Link"),
        id: None,
        href: "https://example.com".to_string(),
        target: Some("_blank".to_string()),
        class: None,
        color_scheme: None,
        external: Some(true),
    };

    let result = dioxus_ssr::render_element(Link(props));
    assert!(result.contains(r#"rel="noopener noreferrer""#));
}

#[test]
fn test_link_with_custom_class() {
    let props = LinkProps {
        children: rsx!("Test Link"),
        id: None,
        href: "https://example.com".to_string(),
        target: None,
        class: Some("custom-class".to_string()),
        color_scheme: None,
        external: None,
    };

    let result = dioxus_ssr::render_element(Link(props));
    assert!(result.contains(r#"class="link link-neutral custom-class""#));
}

#[test]
fn test_link_with_id() {
    let props = LinkProps {
        children: rsx!("Test Link"),
        id: Some("test-link".to_string()),
        href: "https://example.com".to_string(),
        target: None,
        class: None,
        color_scheme: None,
        external: None,
    };

    let result = dioxus_ssr::render_element(Link(props));
    assert!(result.contains(r#"id="test-link""#));
}
