#![allow(non_snake_case)]
use std::fmt::Display;
use dioxus::prelude::*;

/// A Calendar component for date picker and calendar display.
///
/// # Examples
///
/// Basic usage:
///
/// ```text
/// use daisy_rsx::{Calendar, CalendarHeader, CalendarBody, CalendarDay, CalendarWeekday};
///
/// Calendar {
///     children: rsx!(
///         CalendarHeader { month: "December 2025" }
///         CalendarBody { children: rsx!(
///             CalendarWeekday { children: rsx!("Sun") }
///             CalendarWeekday { children: rsx!("Mon") }
///             CalendarWeekday { children: rsx!("Tue") }
///             CalendarWeekday { children: rsx!("Wed") }
///             CalendarWeekday { children: rsx!("Thu") }
///             CalendarWeekday { children: rsx!("Fri") }
///             CalendarWeekday { children: rsx!("Sat") }
///             CalendarDay { day: 1, children: rsx!("1") }
///             CalendarDay { day: 2, children: rsx!("2") }
///         )}
///     )
/// }
/// ```

/// Color scheme options for Calendar component
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CalendarColorScheme {
    /// Primary color
    Primary,
    /// Secondary color
    Secondary,
    /// Accent color
    Accent,
    /// Info color
    Info,
    /// Success color
    Success,
    /// Warning color
    Warning,
    /// Error color
    Error,
}

impl Display for CalendarColorScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalendarColorScheme::Primary => write!(f, "calendar-primary"),
            CalendarColorScheme::Secondary => write!(f, "calendar-secondary"),
            CalendarColorScheme::Accent => write!(f, "calendar-accent"),
            CalendarColorScheme::Info => write!(f, "calendar-info"),
            CalendarColorScheme::Success => write!(f, "calendar-success"),
            CalendarColorScheme::Warning => write!(f, "calendar-warning"),
            CalendarColorScheme::Error => write!(f, "calendar-error"),
        }
    }
}

/// Size options for Calendar component
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CalendarSize {
    /// Small size
    Small,
    /// Medium size
    Medium,
    /// Large size
    Large,
}

impl Display for CalendarSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalendarSize::Small => write!(f, "calendar-sm"),
            CalendarSize::Medium => write!(f, "calendar-md"),
            CalendarSize::Large => write!(f, "calendar-lg"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CalendarProps {
    /// The content to display inside calendar (CalendarHeader, CalendarBody children)
    children: Element,
    /// Optional ID for calendar element
    id: Option<String>,
    /// Additional CSS classes to apply to calendar
    class: Option<String>,
    /// Color scheme for calendar
    color_scheme: Option<CalendarColorScheme>,
    /// Size of calendar
    size: Option<CalendarSize>,
}

#[component]
pub fn Calendar(props: CalendarProps) -> Element {
    let class = props.class.unwrap_or_default();
    let color_scheme = props.color_scheme;
    let size = props.size;

    // Build CSS classes
    let mut classes = vec!["calendar".to_string()];
    
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

    rsx!(
        div {
            class: "{class_string}",
            id: props.id,
            {props.children}
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct CalendarHeaderProps {
    /// The content to display inside calendar header
    children: Element,
    /// Optional ID for calendar header element
    id: Option<String>,
    /// Additional CSS classes to apply to calendar header
    class: Option<String>,
}

#[component]
pub fn CalendarHeader(props: CalendarHeaderProps) -> Element {
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["calendar-header".to_string()];
    
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
pub struct CalendarBodyProps {
    /// The content to display inside calendar body (CalendarWeekday, CalendarDay children)
    children: Element,
    /// Optional ID for calendar body element
    id: Option<String>,
    /// Additional CSS classes to apply to calendar body
    class: Option<String>,
}

#[component]
pub fn CalendarBody(props: CalendarBodyProps) -> Element {
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["calendar-body".to_string()];
    
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
pub struct CalendarWeekdayProps {
    /// The content to display inside calendar weekday
    children: Element,
    /// Optional ID for calendar weekday element
    id: Option<String>,
    /// Additional CSS classes to apply to calendar weekday
    class: Option<String>,
}

#[component]
pub fn CalendarWeekday(props: CalendarWeekdayProps) -> Element {
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["calendar-weekday".to_string()];
    
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
pub struct CalendarDayProps {
    /// The content to display inside calendar day
    children: Element,
    /// Optional ID for calendar day element
    id: Option<String>,
    /// Additional CSS classes to apply to calendar day
    class: Option<String>,
    /// Day number
    day: i32,
    /// Whether day is selected
    selected: Option<bool>,
    /// Whether day is today
    today: Option<bool>,
    /// Whether day is disabled
    disabled: Option<bool>,
}

#[component]
pub fn CalendarDay(props: CalendarDayProps) -> Element {
    let class = props.class.unwrap_or_default();
    let selected = props.selected.filter(|&x| x);
    let today = props.today.filter(|&x| x);
    let disabled = props.disabled.filter(|&x| x);

    // Build CSS classes
    let mut classes = vec!["calendar-day".to_string()];
    
    if selected.is_some() {
        classes.push("calendar-day-selected".to_string());
    }
    
    if today.is_some() {
        classes.push("calendar-day-today".to_string());
    }
    
    if disabled.is_some() {
        classes.push("calendar-day-disabled".to_string());
    }
    
    if !class.is_empty() {
        classes.push(class);
    }

    let class_string = classes.join(" ");

    rsx!(
        div {
            class: "{class_string}",
            id: props.id,
            "data-day": "{props.day}",
            {props.children}
        }
    )
}

#[test]
fn test_calendar_basic() {
    let props = CalendarProps {
        children: rsx!(
            CalendarHeader { children: rsx!("December 2025") }
            CalendarBody { children: rsx!(
                CalendarWeekday { children: rsx!("Sun") }
                CalendarDay { day: 1, children: rsx!("1") }
            )}
        ),
        id: None,
        class: None,
        color_scheme: None,
        size: None,
    };

    let result = dioxus_ssr::render_element(Calendar(props));
    assert!(result.contains("calendar"));
}

#[test]
fn test_calendar_day() {
    let props = CalendarDayProps {
        children: rsx!("15"),
        id: None,
        class: None,
        day: 15,
        selected: None,
        today: None,
        disabled: None,
    };

    let result = dioxus_ssr::render_element(CalendarDay(props));
    assert!(result.contains("calendar-day"));
    assert!(result.contains(r#"data-day="15""#));
}

#[test]
fn test_calendar_day_selected() {
    let props = CalendarDayProps {
        children: rsx!("15"),
        id: None,
        class: None,
        day: 15,
        selected: Some(true),
        today: None,
        disabled: None,
    };

    let result = dioxus_ssr::render_element(CalendarDay(props));
    assert!(result.contains("calendar-day-selected"));
}

#[test]
fn test_calendar_day_today() {
    let props = CalendarDayProps {
        children: rsx!("15"),
        id: None,
        class: None,
        day: 15,
        selected: None,
        today: Some(true),
        disabled: None,
    };

    let result = dioxus_ssr::render_element(CalendarDay(props));
    assert!(result.contains("calendar-day-today"));
}

#[test]
fn test_calendar_day_disabled() {
    let props = CalendarDayProps {
        children: rsx!("15"),
        id: None,
        class: None,
        day: 15,
        selected: None,
        today: None,
        disabled: Some(true),
    };

    let result = dioxus_ssr::render_element(CalendarDay(props));
    assert!(result.contains("calendar-day-disabled"));
}

#[test]
fn test_calendar_with_color_scheme() {
    let props = CalendarProps {
        children: rsx!(CalendarHeader { children: rsx!("December 2025") }),
        id: None,
        class: None,
        color_scheme: Some(CalendarColorScheme::Primary),
        size: None,
    };

    let result = dioxus_ssr::render_element(Calendar(props));
    assert!(result.contains("calendar-primary"));
}

#[test]
fn test_calendar_with_size() {
    let props = CalendarProps {
        children: rsx!(CalendarHeader { children: rsx!("December 2025") }),
        id: None,
        class: None,
        color_scheme: None,
        size: Some(CalendarSize::Large),
    };

    let result = dioxus_ssr::render_element(Calendar(props));
    assert!(result.contains("calendar-lg"));
}

#[test]
fn test_calendar_custom_class() {
    let props = CalendarProps {
        children: rsx!(CalendarHeader { children: rsx!("December 2025") }),
        id: None,
        class: Some("custom-class".to_string()),
        color_scheme: None,
        size: None,
    };

    let result = dioxus_ssr::render_element(Calendar(props));
    assert!(result.contains("calendar") && result.contains("custom-class"));
}

#[test]
fn test_calendar_with_id() {
    let props = CalendarProps {
        children: rsx!(CalendarHeader { children: rsx!("December 2025") }),
        id: Some("test-calendar".to_string()),
        class: None,
        color_scheme: None,
        size: None,
    };

    let result = dioxus_ssr::render_element(Calendar(props));
    assert!(result.contains(r#"id="test-calendar""#));
}
