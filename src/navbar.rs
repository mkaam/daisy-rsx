#![allow(non_snake_case)]
use std::fmt::Display;
use dioxus::prelude::*;

/// A Navbar component that creates responsive navigation bars.
///
/// # Examples
///
/// Basic usage:
///
/// ```text
/// use daisy_rsx::{Navbar, NavbarStart, NavbarCenter, NavbarEnd};
///
/// Navbar {
///     NavbarStart { "Brand" }
///     NavbarCenter { "Center" }
///     NavbarEnd { "End" }
/// }
/// ```

#[derive(Props, Clone, PartialEq)]
pub struct NavbarProps {
    /// The content to display inside the navbar
    children: Element,
    /// Optional ID for the navbar element
    id: Option<String>,
    /// Additional CSS classes to apply to the navbar
    class: Option<String>,
}

#[component]
pub fn Navbar(props: NavbarProps) -> Element {
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["navbar".to_string()];
    
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
pub struct NavbarStartProps {
    /// The content to display in the start section
    children: Element,
    /// Optional ID for the navbar start element
    id: Option<String>,
    /// Additional CSS classes to apply
    class: Option<String>,
}

#[component]
pub fn NavbarStart(props: NavbarStartProps) -> Element {
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["navbar-start".to_string()];
    
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
pub struct NavbarCenterProps {
    /// The content to display in the center section
    children: Element,
    /// Optional ID for the navbar center element
    id: Option<String>,
    /// Additional CSS classes to apply
    class: Option<String>,
}

#[component]
pub fn NavbarCenter(props: NavbarCenterProps) -> Element {
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["navbar-center".to_string()];
    
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
pub struct NavbarEndProps {
    /// The content to display in the end section
    children: Element,
    /// Optional ID for the navbar end element
    id: Option<String>,
    /// Additional CSS classes to apply
    class: Option<String>,
}

#[component]
pub fn NavbarEnd(props: NavbarEndProps) -> Element {
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["navbar-end".to_string()];
    
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
fn test_navbar_basic() {
    let props = NavbarProps {
        children: rsx!(
            NavbarStart { children: rsx!("Brand") }
            NavbarCenter { children: rsx!("Center") }
            NavbarEnd { children: rsx!("End") }
        ),
        id: None,
        class: None,
    };

    let result = dioxus_ssr::render_element(Navbar(props));
    assert!(result.contains(r#"class="navbar""#));
}

#[test]
fn test_navbar_with_custom_class() {
    let props = NavbarProps {
        children: rsx!(
            NavbarStart { children: rsx!("Brand") }
        ),
        id: None,
        class: Some("custom-class".to_string()),
    };

    let result = dioxus_ssr::render_element(Navbar(props));
    assert!(result.contains(r#"class="navbar custom-class""#));
}

#[test]
fn test_navbar_with_id() {
    let props = NavbarProps {
        children: rsx!(
            NavbarStart { children: rsx!("Brand") }
        ),
        id: Some("test-navbar".to_string()),
        class: None,
    };

    let result = dioxus_ssr::render_element(Navbar(props));
    assert!(result.contains(r#"id="test-navbar""#));
}
