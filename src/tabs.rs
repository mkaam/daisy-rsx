#![allow(non_snake_case)]
use std::fmt::Display;
use dioxus::prelude::*;

/// A Tabs component that creates tabbed interfaces.
///
/// # Examples
///
/// Basic usage:
///
/// ```text
/// use daisy_rsx::{Tabs, Tab, TabPanel, TabsOrientation};
///
/// Tabs {
///     orientation: TabsOrientation::Vertical,
///     Tab { value: "tab1".to_string(), children: rsx!("Tab 1") }
///     Tab { value: "tab2".to_string(), children: rsx!("Tab 2") }
///     TabPanel { value: "tab1".to_string(), children: rsx!("Content 1") }
///     TabPanel { value: "tab2".to_string(), children: rsx!("Content 2") }
/// }
/// ```

/// Orientation options for Tabs component
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum TabsOrientation {
    #[default]
    /// Vertical orientation (default)
    Vertical,
    /// Horizontal orientation
    Horizontal,
}

impl Display for TabsOrientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TabsOrientation::Vertical => write!(f, "tabs-vertical"),
            TabsOrientation::Horizontal => write!(f, "tabs-horizontal"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TabsProps {
    /// The content to display inside tabs (Tab and TabPanel children)
    children: Element,
    /// Optional ID for tabs element
    id: Option<String>,
    /// Additional CSS classes to apply to tabs
    class: Option<String>,
    /// Orientation of tabs (vertical or horizontal)
    orientation: Option<TabsOrientation>,
}

#[component]
pub fn Tabs(props: TabsProps) -> Element {
    let orientation = props.orientation.unwrap_or_default();
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["tabs".to_string()];
    classes.push(orientation.to_string());
    
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
pub struct TabProps {
    /// The content to display inside tab
    children: Element,
    /// Optional ID for tab element
    id: Option<String>,
    /// Additional CSS classes to apply to tab
    class: Option<String>,
    /// Value of tab (must match TabPanel value)
    value: String,
    /// Whether tab is disabled
    disabled: Option<bool>,
}

#[component]
pub fn Tab(props: TabProps) -> Element {
    let class = props.class.unwrap_or_default();
    let disabled = props.disabled.filter(|&x| x);

    // Build CSS classes
    let mut classes = vec!["tab".to_string()];
    
    if disabled.is_some() {
        classes.push("tab-disabled".to_string());
    }
    
    if !class.is_empty() {
        classes.push(class);
    }

    let class_string = classes.join(" ");

    rsx!(
        a {
            class: "{class_string}",
            id: props.id,
            "data-value": "{props.value}",
            {props.children}
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct TabPanelProps {
    /// The content to display inside tab panel
    children: Element,
    /// Optional ID for tab panel element
    id: Option<String>,
    /// Additional CSS classes to apply to tab panel
    class: Option<String>,
    /// Value of tab panel (must match Tab value)
    value: String,
}

#[component]
pub fn TabPanel(props: TabPanelProps) -> Element {
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["tab-content".to_string()];
    
    if !class.is_empty() {
        classes.push(class);
    }

    let class_string = classes.join(" ");

    rsx!(
        div {
            class: "{class_string}",
            id: props.id,
            "data-value": "{props.value}",
            {props.children}
        }
    )
}

#[test]
fn test_tabs_basic() {
    let props = TabsProps {
        children: rsx!(
            Tab { value: "tab1".to_string(), children: rsx!("Tab 1") }
            Tab { value: "tab2".to_string(), children: rsx!("Tab 2") }
            TabPanel { value: "tab1".to_string(), children: rsx!("Content 1") }
            TabPanel { value: "tab2".to_string(), children: rsx!("Content 2") }
        ),
        id: None,
        class: None,
        orientation: None,
    };

    let result = dioxus_ssr::render_element(Tabs(props));
    assert!(result.contains(r#"class="tabs tabs-vertical""#));
}

#[test]
fn test_tabs_horizontal() {
    let props = TabsProps {
        children: rsx!(
            Tab { value: "tab1".to_string(), children: rsx!("Tab 1") }
            TabPanel { value: "tab1".to_string(), children: rsx!("Content") }
        ),
        id: None,
        class: None,
        orientation: Some(TabsOrientation::Horizontal),
    };

    let result = dioxus_ssr::render_element(Tabs(props));
    assert!(result.contains(r#"class="tabs tabs-horizontal""#));
}

#[test]
fn test_tab_disabled() {
    let props = TabProps {
        children: rsx!("Disabled Tab"),
        id: None,
        class: None,
        value: "tab1".to_string(),
        disabled: Some(true),
    };

    let result = dioxus_ssr::render_element(Tab(props));
    assert!(result.contains(r#"class="tab tab-disabled""#));
}

#[test]
fn test_tab_with_custom_class() {
    let props = TabsProps {
        children: rsx!(
            Tab { value: "tab1".to_string(), children: rsx!("Tab 1") }
        ),
        id: None,
        class: Some("custom-class".to_string()),
        orientation: None,
    };

    let result = dioxus_ssr::render_element(Tabs(props));
    assert!(result.contains(r#"class="tabs tabs-vertical custom-class""#));
}

#[test]
fn test_tab_with_id() {
    let props = TabProps {
        children: rsx!("Tab"),
        id: Some("test-tab".to_string()),
        class: None,
        value: "tab1".to_string(),
        disabled: None,
    };

    let result = dioxus_ssr::render_element(Tab(props));
    assert!(result.contains(r#"id="test-tab""#));
}
