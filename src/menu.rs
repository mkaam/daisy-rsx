#![allow(non_snake_case)]
use std::fmt::Display;
use dioxus::prelude::*;

/// A Menu component that creates vertical and horizontal navigation menus with nested items.
///
/// # Examples
///
/// Basic usage:
///
/// ```text
/// use daisy_rsx::{Menu, MenuItem, MenuTitle, MenuOrientation};
///
/// Menu {
///     orientation: MenuOrientation::Vertical,
///     MenuTitle { "Navigation" }
///     MenuItem { href: "/home", "Home" }
///     MenuItem { href: "/about", "About" }
/// }
/// ```

/// Orientation options for Menu component
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum MenuOrientation {
    #[default]
    /// Vertical orientation (default)
    Vertical,
    /// Horizontal orientation
    Horizontal,
}

impl Display for MenuOrientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MenuOrientation::Vertical => write!(f, "menu-vertical"),
            MenuOrientation::Horizontal => write!(f, "menu-horizontal"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct MenuProps {
    /// The content to display inside the menu
    children: Element,
    /// Optional ID for the menu element
    id: Option<String>,
    /// Additional CSS classes to apply to the menu
    class: Option<String>,
    /// Orientation of the menu (vertical or horizontal)
    orientation: Option<MenuOrientation>,
}

#[component]
pub fn Menu(props: MenuProps) -> Element {
    let orientation = props.orientation.unwrap_or_default();
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["menu".to_string()];
    classes.push(orientation.to_string());
    
    if !class.is_empty() {
        classes.push(class);
    }

    let class_string = classes.join(" ");

    rsx!(
        ul {
            class: "{class_string}",
            id: props.id,
            {props.children}
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct MenuItemProps {
    /// The content to display inside the menu item
    children: Element,
    /// Optional ID for the menu item
    id: Option<String>,
    /// Additional CSS classes to apply to the menu item
    class: Option<String>,
    /// Optional href to render as a link
    href: Option<String>,
    /// Whether the menu item is active
    active: Option<bool>,
    /// Whether the menu item is disabled
    disabled: Option<bool>,
}

#[component]
pub fn MenuItem(props: MenuItemProps) -> Element {
    let class = props.class.unwrap_or_default();
    let active = props.active.filter(|&x| x);
    let disabled = props.disabled.filter(|&x| x);

    // Build CSS classes
    let mut classes = vec!["menu-item".to_string()];
    
    if active.is_some() {
        classes.push("active".to_string());
    }
    
    if disabled.is_some() {
        classes.push("disabled".to_string());
    }
    
    if !class.is_empty() {
        classes.push(class);
    }

    let class_string = classes.join(" ");

    if let Some(href) = props.href {
        rsx!(
            li {
                class: "{class_string}",
                id: props.id,
                a {
                    href: "{href}",
                    {props.children}
                }
            }
        )
    } else {
        rsx!(
            li {
                class: "{class_string}",
                id: props.id,
                {props.children}
            }
        )
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct MenuTitleProps {
    /// The content to display inside the menu title
    children: Element,
    /// Optional ID for the menu title
    id: Option<String>,
    /// Additional CSS classes to apply to the menu title
    class: Option<String>,
}

#[component]
pub fn MenuTitle(props: MenuTitleProps) -> Element {
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["menu-title".to_string()];
    
    if !class.is_empty() {
        classes.push(class);
    }

    let class_string = classes.join(" ");

    rsx!(
        li {
            class: "{class_string}",
            id: props.id,
            {props.children}
        }
    )
}

#[test]
fn test_menu_basic() {
    let props = MenuProps {
        children: rsx!(
            MenuTitle { children: rsx!("Navigation") }
            MenuItem { children: rsx!("Home") }
            MenuItem { children: rsx!("About") }
        ),
        id: None,
        class: None,
        orientation: None,
    };

    let result = dioxus_ssr::render_element(Menu(props));
    assert!(result.contains(r#"class="menu menu-vertical""#));
}

#[test]
fn test_menu_horizontal() {
    let props = MenuProps {
        children: rsx!(
            MenuItem { children: rsx!("Home") }
            MenuItem { children: rsx!("About") }
        ),
        id: None,
        class: None,
        orientation: Some(MenuOrientation::Horizontal),
    };

    let result = dioxus_ssr::render_element(Menu(props));
    assert!(result.contains(r#"class="menu menu-horizontal""#));
}

#[test]
fn test_menu_item_active() {
    let props = MenuItemProps {
        children: rsx!("Active Item"),
        id: None,
        class: None,
        href: None,
        active: Some(true),
        disabled: None,
    };

    let result = dioxus_ssr::render_element(MenuItem(props));
    assert!(result.contains(r#"class="menu-item active""#));
}

#[test]
fn test_menu_item_disabled() {
    let props = MenuItemProps {
        children: rsx!("Disabled Item"),
        id: None,
        class: None,
        href: None,
        active: None,
        disabled: Some(true),
    };

    let result = dioxus_ssr::render_element(MenuItem(props));
    assert!(result.contains(r#"class="menu-item disabled""#));
}

#[test]
fn test_menu_item_with_href() {
    let props = MenuItemProps {
        children: rsx!("Link"),
        id: None,
        class: None,
        href: Some("/home".to_string()),
        active: None,
        disabled: None,
    };

    let result = dioxus_ssr::render_element(MenuItem(props));
    assert!(result.contains(r#"href="/home""#));
}

#[test]
fn test_menu_with_custom_class() {
    let props = MenuProps {
        children: rsx!(
            MenuItem { children: rsx!("Home") }
        ),
        id: None,
        class: Some("custom-class".to_string()),
        orientation: None,
    };

    let result = dioxus_ssr::render_element(Menu(props));
    assert!(result.contains(r#"class="menu menu-vertical custom-class""#));
}
