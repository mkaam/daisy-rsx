#![allow(non_snake_case)]
use std::fmt::Display;
use dioxus::prelude::*;

/// A Theme component for applying daisyUI themes.
///
/// # Examples
///
/// Basic usage:
///
/// ```text
/// use daisy_rsx::{Theme, ThemeName};
///
/// Theme {
///     name: ThemeName::Light,
///     children: rsx!(
///         div { "Content with light theme" }
///     )
/// }
/// ```

/// Theme names supported by daisyUI
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ThemeName {
    /// Light theme
    Light,
    /// Dark theme
    Dark,
    /// Cupcake theme
    Cupcake,
    /// Bumblebee theme
    Bumblebee,
    /// Emerald theme
    Emerald,
    /// Corporate theme
    Corporate,
    /// Synthwave theme
    Synthwave,
    /// Retro theme
    Retro,
    /// Cyberpunk theme
    Cyberpunk,
    /// Valentine theme
    Valentine,
    /// Halloween theme
    Halloween,
    /// Garden theme
    Garden,
    /// Forest theme
    Forest,
    /// Aqua theme
    Aqua,
    /// Lo-fi theme
    Lofi,
    /// Pastel theme
    Pastel,
    /// Fantasy theme
    Fantasy,
    /// Wireframe theme
    Wireframe,
    /// Black theme
    Black,
    /// Luxury theme
    Luxury,
    /// Dracula theme
    Dracula,
    /// Cmyk theme
    Cmyk,
    /// Autumn theme
    Autumn,
    /// Business theme
    Business,
    /// Acid theme
    Acid,
    /// Lemonade theme
    Lemonade,
    /// Night theme
    Night,
    /// Coffee theme
    Coffee,
    /// Winter theme
    Winter,
}

impl Display for ThemeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThemeName::Light => write!(f, "light"),
            ThemeName::Dark => write!(f, "dark"),
            ThemeName::Cupcake => write!(f, "cupcake"),
            ThemeName::Bumblebee => write!(f, "bumblebee"),
            ThemeName::Emerald => write!(f, "emerald"),
            ThemeName::Corporate => write!(f, "corporate"),
            ThemeName::Synthwave => write!(f, "synthwave"),
            ThemeName::Retro => write!(f, "retro"),
            ThemeName::Cyberpunk => write!(f, "cyberpunk"),
            ThemeName::Valentine => write!(f, "valentine"),
            ThemeName::Halloween => write!(f, "halloween"),
            ThemeName::Garden => write!(f, "garden"),
            ThemeName::Forest => write!(f, "forest"),
            ThemeName::Aqua => write!(f, "aqua"),
            ThemeName::Lofi => write!(f, "lofi"),
            ThemeName::Pastel => write!(f, "pastel"),
            ThemeName::Fantasy => write!(f, "fantasy"),
            ThemeName::Wireframe => write!(f, "wireframe"),
            ThemeName::Black => write!(f, "black"),
            ThemeName::Luxury => write!(f, "luxury"),
            ThemeName::Dracula => write!(f, "dracula"),
            ThemeName::Cmyk => write!(f, "cmyk"),
            ThemeName::Autumn => write!(f, "autumn"),
            ThemeName::Business => write!(f, "business"),
            ThemeName::Acid => write!(f, "acid"),
            ThemeName::Lemonade => write!(f, "lemonade"),
            ThemeName::Night => write!(f, "night"),
            ThemeName::Coffee => write!(f, "coffee"),
            ThemeName::Winter => write!(f, "winter"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ThemeProps {
    /// The content to display with theme applied
    children: Element,
    /// Theme name to apply
    name: ThemeName,
    /// Optional ID for theme element
    id: Option<String>,
    /// Additional CSS classes to apply
    class: Option<String>,
}

#[component]
pub fn Theme(props: ThemeProps) -> Element {
    let class = props.class.unwrap_or_default();
    let theme_class = format!("data-theme={}", props.name.to_string());

    // Build CSS classes
    let mut classes = vec![];
    
    if !class.is_empty() {
        classes.push(class);
    }

    let class_string = classes.join(" ");

    rsx!(
        div {
            class: "{class_string}",
            id: props.id,
            {theme_class},
            {props.children}
        }
    )
}

#[test]
fn test_theme_light() {
    let props = ThemeProps {
        children: rsx!(div { "Content" }),
        name: ThemeName::Light,
        id: None,
        class: None,
    };

    let result = dioxus_ssr::render_element(Theme(props));
    assert!(result.contains(r#"data-theme=light"#));
}

#[test]
fn test_theme_dark() {
    let props = ThemeProps {
        children: rsx!(div { "Content" }),
        name: ThemeName::Dark,
        id: None,
        class: None,
    };

    let result = dioxus_ssr::render_element(Theme(props));
    assert!(result.contains(r#"data-theme=dark"#));
}

#[test]
fn test_theme_custom_class() {
    let props = ThemeProps {
        children: rsx!(div { "Content" }),
        name: ThemeName::Emerald,
        id: None,
        class: Some("custom-class".to_string()),
    };

    let result = dioxus_ssr::render_element(Theme(props));
    assert!(result.contains(r#"data-theme=emerald"#));
    assert!(result.contains(r#"class="custom-class""#));
}

#[test]
fn test_theme_with_id() {
    let props = ThemeProps {
        children: rsx!(div { "Content" }),
        name: ThemeName::Dracula,
        id: Some("test-theme".to_string()),
        class: None,
    };

    let result = dioxus_ssr::render_element(Theme(props));
    assert!(result.contains(r#"id="test-theme""#));
    assert!(result.contains(r#"data-theme=dracula"#));
}

#[test]
fn test_theme_various_themes() {
    let themes = vec![
        ThemeName::Cupcake,
        ThemeName::Bumblebee,
        ThemeName::Emerald,
        ThemeName::Corporate,
        ThemeName::Synthwave,
        ThemeName::Retro,
        ThemeName::Cyberpunk,
        ThemeName::Valentine,
        ThemeName::Halloween,
        ThemeName::Garden,
    ];

    for theme in themes {
        let props = ThemeProps {
            children: rsx!(div { "Content" }),
            name: theme,
            id: None,
            class: None,
        };

        let result = dioxus_ssr::render_element(Theme(props));
        assert!(result.contains(&format!("data-theme={}", theme.to_string())));
    }
}
