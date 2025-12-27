#![allow(non_snake_case)]
use std::fmt::Display;
use dioxus::prelude::*;

/// A Footer component for website footers with links, social icons, and branding.
///
/// # Examples
///
/// Basic usage:
///
/// ```text
/// use daisy_rsx::{Footer, FooterSection, FooterLink, FooterCopyright};
///
/// Footer {
///     children: rsx!(
///         FooterSection { title: "Product", children: rsx!(
///             FooterLink { href: "/features", children: rsx!("Features") }
///             FooterLink { href: "/pricing", children: rsx!("Pricing") }
///         )}
///     ),
///     copyright: Some("© 2025 My Company"),
/// }
/// ```

/// Color scheme options for Footer component
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FooterColorScheme {
    /// Neutral color
    Neutral,
    /// Primary color
    Primary,
    /// Secondary color
    Secondary,
}

impl Display for FooterColorScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FooterColorScheme::Neutral => write!(f, "footer-neutral"),
            FooterColorScheme::Primary => write!(f, "footer-primary"),
            FooterColorScheme::Secondary => write!(f, "footer-secondary"),
        }
    }
}

/// Size options for Footer component
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FooterSize {
    /// Small size
    Small,
    /// Medium size
    Medium,
    /// Large size
    Large,
}

impl Display for FooterSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FooterSize::Small => write!(f, "footer-sm"),
            FooterSize::Medium => write!(f, "footer-md"),
            FooterSize::Large => write!(f, "footer-lg"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct FooterProps {
    /// The content to display inside footer (FooterSection children)
    children: Element,
    /// Optional ID for footer element
    id: Option<String>,
    /// Additional CSS classes to apply to footer
    class: Option<String>,
    /// Logo/branding element
    logo: Option<Element>,
    /// Footer title
    title: Option<String>,
    /// Footer description
    description: Option<String>,
    /// Copyright notice
    copyright: Option<String>,
    /// Copyright year
    year: Option<i32>,
    /// Color scheme for footer
    color_scheme: Option<FooterColorScheme>,
    /// Size of footer
    size: Option<FooterSize>,
}

#[component]
pub fn Footer(props: FooterProps) -> Element {
    let class = props.class.unwrap_or_default();
    let color_scheme = props.color_scheme;
    let size = props.size;
    let year = props.year.unwrap_or(2025);

    // Build CSS classes
    let mut classes = vec!["footer".to_string()];
    
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

    // Build copyright text
    let copyright_text = if let Some(copyright) = &props.copyright {
        copyright.replace("{year}", &year.to_string())
    } else {
        format!("© {} My Company", year)
    };

    rsx!(
        footer {
            class: "{class_string}",
            id: props.id,
            {props.logo}
            {props.title.as_ref().map(|title| rsx!(div { class: "footer-title", "{title}" }))}
            {props.description.as_ref().map(|description| rsx!(div { class: "footer-description", "{description}" }))}
            {props.children}
            div { class: "footer-copyright", "{copyright_text}" }
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct FooterSectionProps {
    /// The content to display inside footer section (FooterLink children)
    children: Element,
    /// Optional ID for footer section element
    id: Option<String>,
    /// Additional CSS classes to apply to footer section
    class: Option<String>,
    /// Section title
    title: String,
}

#[component]
pub fn FooterSection(props: FooterSectionProps) -> Element {
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["footer-section".to_string()];
    
    if !class.is_empty() {
        classes.push(class);
    }

    let class_string = classes.join(" ");

    rsx!(
        div {
            class: "{class_string}",
            id: props.id,
            h4 { class: "footer-title", "{props.title}" }
            {props.children}
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct FooterLinkProps {
    /// The content to display inside footer link
    children: Element,
    /// Optional ID for footer link element
    id: Option<String>,
    /// Additional CSS classes to apply to footer link
    class: Option<String>,
    /// Link href
    href: String,
    /// Whether link is external
    external: Option<bool>,
}

#[component]
pub fn FooterLink(props: FooterLinkProps) -> Element {
    let class = props.class.unwrap_or_default();
    let external = props.external.filter(|&x| x);

    // Build CSS classes
    let mut classes = vec!["link".to_string()];
    classes.push("link-hover".to_string());
    
    if !class.is_empty() {
        classes.push(class);
    }

    let class_string = classes.join(" ");

    rsx!(
        a {
            class: "{class_string}",
            id: props.id,
            href: "{props.href}",
            r#rel: if external.is_some() { Some("noopener noreferrer") } else { None },
            {props.children}
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct FooterCopyrightProps {
    /// Copyright text
    text: String,
    /// Copyright year
    year: Option<i32>,
    /// Optional ID for footer copyright element
    id: Option<String>,
    /// Additional CSS classes to apply to footer copyright
    class: Option<String>,
}

#[component]
pub fn FooterCopyright(props: FooterCopyrightProps) -> Element {
    let class = props.class.unwrap_or_default();
    let year = props.year.unwrap_or(2025);

    // Build CSS classes
    let mut classes = vec!["footer-copyright".to_string()];
    
    if !class.is_empty() {
        classes.push(class);
    }

    let class_string = classes.join(" ");
    let copyright_text = props.text.replace("{year}", &year.to_string());

    rsx!(
        div {
            class: "{class_string}",
            id: props.id,
            "{copyright_text}"
        }
    )
}

#[test]
fn test_footer_basic() {
    let props = FooterProps {
        children: rsx!(
            FooterSection { title: "Product", children: rsx!(
                FooterLink { href: "/features", children: rsx!("Features") }
            )}
        ),
        id: None,
        class: None,
        logo: None,
        title: None,
        description: None,
        copyright: Some("© {year} My Company".to_string()),
        year: None,
        color_scheme: None,
        size: None,
    };

    let result = dioxus_ssr::render_element(Footer(props));
    assert!(result.contains("footer"));
}

#[test]
fn test_footer_section() {
    let props = FooterSectionProps {
        children: rsx!(
            FooterLink { href: "/about", children: rsx!("About") }
        ),
        id: None,
        class: None,
        title: "Company".to_string(),
    };

    let result = dioxus_ssr::render_element(FooterSection(props));
    assert!(result.contains("footer-section"));
}

#[test]
fn test_footer_link() {
    let props = FooterLinkProps {
        children: rsx!("Link"),
        id: None,
        class: None,
        href: "/page".to_string(),
        external: None,
    };

    let result = dioxus_ssr::render_element(FooterLink(props));
    assert!(result.contains(r#"href="/page""#));
}

#[test]
fn test_footer_link_external() {
    let props = FooterLinkProps {
        children: rsx!("External Link"),
        id: None,
        class: None,
        href: "https://example.com".to_string(),
        external: Some(true),
    };

    let result = dioxus_ssr::render_element(FooterLink(props));
    assert!(result.contains(r#"rel="noopener noreferrer""#));
}

#[test]
fn test_footer_with_color_scheme() {
    let props = FooterProps {
        children: rsx!(FooterSection { title: "Test", children: rsx!() }),
        id: None,
        class: None,
        logo: None,
        title: None,
        description: None,
        copyright: None,
        year: None,
        color_scheme: Some(FooterColorScheme::Primary),
        size: None,
    };

    let result = dioxus_ssr::render_element(Footer(props));
    assert!(result.contains("footer-primary"));
}

#[test]
fn test_footer_custom_class() {
    let props = FooterProps {
        children: rsx!(FooterSection { title: "Test", children: rsx!() }),
        id: None,
        class: Some("custom-class".to_string()),
        logo: None,
        title: None,
        description: None,
        copyright: None,
        year: None,
        color_scheme: None,
        size: None,
    };

    let result = dioxus_ssr::render_element(Footer(props));
    assert!(result.contains("footer") && result.contains("custom-class"));
}
