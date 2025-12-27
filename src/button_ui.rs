#![allow(non_snake_case)]
use std::fmt::Display;
use dioxus::prelude::*;

/// An enhanced button component that provides comprehensive styling options based on DaisyUI button component.
///
/// # Examples
///
/// Basic usage:
///
/// ```text
/// use daisy_rsx::{ButtonUI, ButtonUIColorScheme, ButtonUISize};
///
/// ButtonUI {
///     color_scheme: ButtonUIColorScheme::Primary,
///     size: ButtonUISize::Large,
///     "Click me"
/// }
/// ```
///
/// With all styling options:
///
/// ```text
/// use daisy_rsx::{ButtonUI, ButtonUIColorScheme, ButtonUISize, ButtonUIShape, ButtonUIVariant};
///
/// ButtonUI {
///     color_scheme: ButtonUIColorScheme::Success,
///     size: ButtonUISize::Small,
///     shape: ButtonUIShape::Circle,
///     variant: ButtonUIVariant::Outline,
///     "Save"
/// }
/// ```

/// Color scheme options for ButtonUI component
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonUIColorScheme {
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
    /// Ghost/transparent color scheme
    Ghost,
    /// Link-style button
    Link,
}

impl Display for ButtonUIColorScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ButtonUIColorScheme::Neutral => write!(f, "btn-neutral"),
            ButtonUIColorScheme::Primary => write!(f, "btn-primary"),
            ButtonUIColorScheme::Secondary => write!(f, "btn-secondary"),
            ButtonUIColorScheme::Accent => write!(f, "btn-accent"),
            ButtonUIColorScheme::Info => write!(f, "btn-info"),
            ButtonUIColorScheme::Success => write!(f, "btn-success"),
            ButtonUIColorScheme::Warning => write!(f, "btn-warning"),
            ButtonUIColorScheme::Error => write!(f, "btn-error"),
            ButtonUIColorScheme::Ghost => write!(f, "btn-ghost"),
            ButtonUIColorScheme::Link => write!(f, "btn-link"),
        }
    }
}

/// Size options for ButtonUI component
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonUISize {
    #[default]
    /// Default size (equivalent to Small)
    Default,
    /// Large button size
    Large,
    /// Medium button size
    Medium,
    /// Small button size
    Small,
    /// Extra small button size
    ExtraSmall,
    /// Tiny button size
    Tiny,
}

impl Display for ButtonUISize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ButtonUISize::Default => write!(f, ""),
            ButtonUISize::Large => write!(f, "btn-lg"),
            ButtonUISize::Medium => write!(f, "btn-md"),
            ButtonUISize::Small => write!(f, "btn-sm"),
            ButtonUISize::ExtraSmall => write!(f, "btn-xs"),
            ButtonUISize::Tiny => write!(f, "btn-tiny"),
        }
    }
}

/// Shape options for ButtonUI component
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonUIShape {
    #[default]
    /// Default rectangular shape
    None,
    /// Circular button shape
    Circle,
    /// Square button shape
    Square,
}

impl Display for ButtonUIShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ButtonUIShape::None => write!(f, ""),
            ButtonUIShape::Circle => write!(f, "btn-circle"),
            ButtonUIShape::Square => write!(f, "btn-square"),
        }
    }
}

/// Visual variant options for ButtonUI component
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonUIVariant {
    #[default]
    /// Default solid button style
    None,
    /// Outline button style
    Outline,
    /// Soft/light button style
    Soft,
    /// Wide button style
    Wide,
    /// Full-width block button style
    Block,
    /// Glass morphism effect
    Glass,
}

impl Display for ButtonUIVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ButtonUIVariant::None => write!(f, ""),
            ButtonUIVariant::Outline => write!(f, "btn-outline"),
            ButtonUIVariant::Soft => write!(f, "btn-soft"),
            ButtonUIVariant::Wide => write!(f, "btn-wide"),
            ButtonUIVariant::Block => write!(f, "btn-block"),
            ButtonUIVariant::Glass => write!(f, "glass"),
        }
    }
}

/// State options for ButtonUI component
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonUIState {
    #[default]
    /// Default state
    None,
    /// Active/pressed state
    Active,
    /// Disabled state
    Disabled,
    /// Loading state with spinner
    Loading,
    /// Focus state
    Focus,
}

impl Display for ButtonUIState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ButtonUIState::None => write!(f, ""),
            ButtonUIState::Active => write!(f, "btn-active"),
            ButtonUIState::Disabled => write!(f, "btn-disabled"),
            ButtonUIState::Loading => write!(f, "loading"),
            ButtonUIState::Focus => write!(f, "btn-focus"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ButtonUIProps {
    /// The content to display inside the button
    children: Element,
    /// Optional ID for the button element
    id: Option<String>,
    /// Additional CSS classes to apply to the button
    class: Option<String>,
    /// Whether the button should be disabled
    disabled: Option<bool>,
    /// If provided, renders as an anchor tag with this href
    href: Option<String>,
    /// Target attribute for anchor tag (when href is provided)
    target: Option<String>,
    /// Color scheme for the button
    color_scheme: Option<ButtonUIColorScheme>,
    /// Size of the button
    size: Option<ButtonUISize>,
    /// Shape of the button
    shape: Option<ButtonUIShape>,
    /// Visual variant/style of the button
    variant: Option<ButtonUIVariant>,
    /// State of the button
    state: Option<ButtonUIState>,
    /// Whether to show loading state
    loading: Option<bool>,
    /// HTML string for icon to show before the button text
    prefix_icon: Option<String>,
    /// HTML string for icon to show after the button text
    suffix_icon: Option<String>,
}

#[component]
pub fn ButtonUI(props: ButtonUIProps) -> Element {
    let color_scheme = props.color_scheme.unwrap_or_default();
    let size = props.size.unwrap_or_default();
    let shape = props.shape.unwrap_or_default();
    let variant = props.variant.unwrap_or_default();
    let state = props.state.unwrap_or_default();
    let class = props.class.unwrap_or_default();
    let disabled = props.disabled.filter(|&x| x);
    let loading = props.loading.filter(|&x| x);

    // Determine if button should be in loading state
    let is_loading = loading.is_some() || matches!(props.state, Some(ButtonUIState::Loading));
    let final_state = if is_loading { ButtonUIState::Loading } else { state };

    // Build CSS classes
    let mut classes = vec!["btn".to_string()];
    
    if !color_scheme.to_string().is_empty() {
        classes.push(color_scheme.to_string());
    }
    if !size.to_string().is_empty() {
        classes.push(size.to_string());
    }
    if !shape.to_string().is_empty() {
        classes.push(shape.to_string());
    }
    if !variant.to_string().is_empty() {
        classes.push(variant.to_string());
    }
    if !final_state.to_string().is_empty() {
        classes.push(final_state.to_string());
    }
    
    if !class.is_empty() {
        classes.push(class);
    }

    let class_string = classes.join(" ");

    // Render as link if href is provided
    if let Some(href) = props.href {
        rsx!(
            a {
                class: "{class_string}",
                id: props.id,
                href: "{href}",
                target: props.target,
                aria_disabled: disabled.map(|_| "true"),
                if let Some(icon) = props.prefix_icon {
                    span { class: "icon", dangerous_inner_html: "{icon}" }
                }
                {props.children}
                if let Some(icon) = props.suffix_icon {
                    span { class: "icon", dangerous_inner_html: "{icon}" }
                }
            }
        )
    } else {
        rsx!(
            button {
                class: "{class_string}",
                id: props.id,
                disabled,
                if let Some(icon) = props.prefix_icon {
                    span { class: "icon", dangerous_inner_html: "{icon}" }
                }
                {props.children}
                if let Some(icon) = props.suffix_icon {
                    span { class: "icon", dangerous_inner_html: "{icon}" }
                }
            }
        )
    }
}

#[test]
fn test_button_ui_basic() {
    let props = ButtonUIProps {
        children: rsx!("Test Button"),
        id: None,
        class: None,
        disabled: None,
        href: None,
        target: None,
        color_scheme: None,
        size: None,
        shape: None,
        variant: None,
        state: None,
        loading: None,
        prefix_icon: None,
        suffix_icon: None,
    };

    let result = dioxus_ssr::render_element(ButtonUI(props));
    assert!(result.contains(r#"<button class="btn btn-neutral""#));
    assert!(result.contains(">Test Button</button>"));
}

#[test]
fn test_button_ui_with_all_props() {
    let props = ButtonUIProps {
        children: rsx!("Complete Button"),
        id: Some("test-button".to_string()),
        class: Some("custom-class".to_string()),
        disabled: Some(false),
        href: Some("https://example.com".to_string()),
        target: Some("_blank".to_string()),
        color_scheme: Some(ButtonUIColorScheme::Primary),
        size: Some(ButtonUISize::Large),
        shape: Some(ButtonUIShape::Circle),
        variant: Some(ButtonUIVariant::Outline),
        state: Some(ButtonUIState::Active),
        loading: None,
        prefix_icon: Some("<svg>...</svg>".to_string()),
        suffix_icon: Some("<svg>...</svg>".to_string()),
    };

    let result = dioxus_ssr::render_element(ButtonUI(props));
    assert!(result.contains(r#"<a class="btn btn-primary btn-lg btn-circle btn-outline btn-active custom-class""#));
    assert!(result.contains(r#"id="test-button""#));
    assert!(result.contains(r#"href="https://example.com""#));
    assert!(result.contains(r#"target="_blank""#));
    assert!(result.contains(r#"<span class="icon"><svg>...</svg></span>"#));
    assert!(result.contains("Complete Button"));
}

#[test]
fn test_button_ui_loading_state() {
    let props = ButtonUIProps {
        children: rsx!("Loading Button"),
        id: None,
        class: None,
        disabled: None,
        href: None,
        target: None,
        color_scheme: None,
        size: None,
        shape: None,
        variant: None,
        state: None,
        loading: Some(true),
        prefix_icon: None,
        suffix_icon: None,
    };

    let result = dioxus_ssr::render_element(ButtonUI(props));
    assert!(result.contains(r#"class="btn btn-neutral loading""#));
    assert!(result.contains(">Loading Button</button>"));
}

#[test]
fn test_all_button_ui_color_schemes() {
    let schemes = [
        (ButtonUIColorScheme::Neutral, "btn-neutral"),
        (ButtonUIColorScheme::Primary, "btn-primary"),
        (ButtonUIColorScheme::Secondary, "btn-secondary"),
        (ButtonUIColorScheme::Accent, "btn-accent"),
        (ButtonUIColorScheme::Info, "btn-info"),
        (ButtonUIColorScheme::Success, "btn-success"),
        (ButtonUIColorScheme::Warning, "btn-warning"),
        (ButtonUIColorScheme::Error, "btn-error"),
        (ButtonUIColorScheme::Ghost, "btn-ghost"),
        (ButtonUIColorScheme::Link, "btn-link"),
    ];

    for (scheme, expected_class) in schemes {
        let props = ButtonUIProps {
            children: rsx!("Test"),
            id: None,
            class: None,
            disabled: None,
            href: None,
            target: None,
            color_scheme: Some(scheme),
            size: None,
            shape: None,
            variant: None,
            state: None,
            loading: None,
            prefix_icon: None,
            suffix_icon: None,
        };

        let result = dioxus_ssr::render_element(ButtonUI(props));
        assert!(result.contains(expected_class),
                "Expected '{}' to contain '{}', but got: {}",
                result, expected_class, result);
    }
}

#[test]
fn test_all_button_ui_sizes() {
    let sizes = [
        (ButtonUISize::Default, ""),
        (ButtonUISize::Large, "btn-lg"),
        (ButtonUISize::Medium, "btn-md"),
        (ButtonUISize::Small, "btn-sm"),
        (ButtonUISize::ExtraSmall, "btn-xs"),
        (ButtonUISize::Tiny, "btn-tiny"),
    ];

    for (size, expected_class) in sizes {
        let props = ButtonUIProps {
            children: rsx!("Test"),
            id: None,
            class: None,
            disabled: None,
            href: None,
            target: None,
            color_scheme: None,
            size: Some(size),
            shape: None,
            variant: None,
            state: None,
            loading: None,
            prefix_icon: None,
            suffix_icon: None,
        };

        let result = dioxus_ssr::render_element(ButtonUI(props));
        if expected_class.is_empty() {
            // Default size should not add any size class, but other classes might be present
            assert!(result.contains("btn btn-neutral"), "Expected basic button classes, but got: {}", result);
        } else {
            assert!(result.contains(expected_class),
                    "Expected '{}' to contain '{}', but got: {}",
                    result, expected_class, result);
        }
    }
}

#[test]
fn test_all_button_ui_shapes() {
    let shapes = [
        (ButtonUIShape::None, ""),
        (ButtonUIShape::Circle, "btn-circle"),
        (ButtonUIShape::Square, "btn-square"),
    ];

    for (shape, expected_class) in shapes {
        let props = ButtonUIProps {
            children: rsx!("Test"),
            id: None,
            class: None,
            disabled: None,
            href: None,
            target: None,
            color_scheme: None,
            size: None,
            shape: Some(shape),
            variant: None,
            state: None,
            loading: None,
            prefix_icon: None,
            suffix_icon: None,
        };

        let result = dioxus_ssr::render_element(ButtonUI(props));
        if expected_class.is_empty() {
            assert!(!result.contains("btn-circle") && !result.contains("btn-square"),
                    "Expected no shape class, but got: {}", result);
        } else {
            assert!(result.contains(expected_class),
                    "Expected '{}' to contain '{}', but got: {}",
                    result, expected_class, result);
        }
    }
}

#[test]
fn test_all_button_ui_variants() {
    let variants = [
        (ButtonUIVariant::None, ""),
        (ButtonUIVariant::Outline, "btn-outline"),
        (ButtonUIVariant::Soft, "btn-soft"),
        (ButtonUIVariant::Wide, "btn-wide"),
        (ButtonUIVariant::Block, "btn-block"),
        (ButtonUIVariant::Glass, "glass"),
    ];

    for (variant, expected_class) in variants {
        let props = ButtonUIProps {
            children: rsx!("Test"),
            id: None,
            class: None,
            disabled: None,
            href: None,
            target: None,
            color_scheme: None,
            size: None,
            shape: None,
            variant: Some(variant),
            state: None,
            loading: None,
            prefix_icon: None,
            suffix_icon: None,
        };

        let result = dioxus_ssr::render_element(ButtonUI(props));
        if expected_class.is_empty() {
            assert!(!result.contains("btn-outline") && !result.contains("btn-soft") &&
                    !result.contains("btn-wide") && !result.contains("btn-block") && !result.contains("glass"),
                    "Expected no variant class, but got: {}", result);
        } else {
            assert!(result.contains(expected_class),
                    "Expected '{}' to contain '{}', but got: {}",
                    result, expected_class, result);
        }
    }
}

#[test]
fn test_all_button_ui_states() {
    let states = [
        (ButtonUIState::None, ""),
        (ButtonUIState::Active, "btn-active"),
        (ButtonUIState::Disabled, "btn-disabled"),
        (ButtonUIState::Loading, "loading"),
        (ButtonUIState::Focus, "btn-focus"),
    ];

    for (state, expected_class) in states {
        let props = ButtonUIProps {
            children: rsx!("Test"),
            id: None,
            class: None,
            disabled: None,
            href: None,
            target: None,
            color_scheme: None,
            size: None,
            shape: None,
            variant: None,
            state: Some(state),
            loading: None,
            prefix_icon: None,
            suffix_icon: None,
        };

        let result = dioxus_ssr::render_element(ButtonUI(props));
        if expected_class.is_empty() {
            assert!(!result.contains("btn-active") && !result.contains("btn-disabled") &&
                    !result.contains("loading") && !result.contains("btn-focus"),
                    "Expected no state class, but got: {}", result);
        } else {
            assert!(result.contains(expected_class),
                    "Expected '{}' to contain '{}', but got: {}",
                    result, expected_class, result);
        }
    }
}