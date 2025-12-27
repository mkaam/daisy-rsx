#![allow(non_snake_case)]
use std::fmt::Display;
use dioxus::prelude::*;

/// A Steps component that displays step-by-step progress indicators.
///
/// # Examples
///
/// Basic usage:
///
/// ```text
/// use daisy_rsx::{Steps, Step, StepsOrientation};
///
/// Steps {
///     orientation: StepsOrientation::Vertical,
///     current_step: 2,
///     Step { value: 1, children: rsx!("Step 1") }
///     Step { value: 2, children: rsx!("Step 2") }
///     Step { value: 3, children: rsx!("Step 3") }
/// }
/// ```

/// Orientation options for Steps component
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum StepsOrientation {
    #[default]
    /// Vertical orientation (default)
    Vertical,
    /// Horizontal orientation
    Horizontal,
}

impl Display for StepsOrientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StepsOrientation::Vertical => write!(f, "steps-vertical"),
            StepsOrientation::Horizontal => write!(f, "steps-horizontal"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct StepsProps {
    /// The content to display inside the steps
    children: Element,
    /// Optional ID for the steps element
    id: Option<String>,
    /// Additional CSS classes to apply to the steps
    class: Option<String>,
    /// Orientation of the steps (vertical or horizontal)
    orientation: Option<StepsOrientation>,
    /// Current step number (1-indexed)
    current_step: Option<i32>,
}

#[component]
pub fn Steps(props: StepsProps) -> Element {
    let orientation = props.orientation.unwrap_or_default();
    let class = props.class.unwrap_or_default();
    let current_step = props.current_step.unwrap_or(0);

    // Build CSS classes
    let mut classes = vec!["steps".to_string()];
    classes.push(orientation.to_string());
    
    if !class.is_empty() {
        classes.push(class);
    }

    let class_string = classes.join(" ");

    // Provide context for child steps
    let steps_context = StepsContext { current_step };

    rsx!(
        ul {
            class: "{class_string}",
            id: props.id,
            {props.children}
        }
    )
}

#[derive(Clone, Copy)]
pub struct StepsContext {
    pub current_step: i32,
}

#[derive(Props, Clone, PartialEq)]
pub struct StepProps {
    /// The content to display inside the step
    children: Element,
    /// Optional ID for the step element
    id: Option<String>,
    /// Additional CSS classes to apply to the step
    class: Option<String>,
    /// Step number
    value: i32,
}

#[component]
pub fn Step(props: StepProps) -> Element {
    let class = props.class.unwrap_or_default();

    // Determine step state based on current step
    let state = if props.value < 0 {
        "step-completed".to_string()
    } else if props.value == 0 {
        "step-current".to_string()
    } else {
        "step-pending".to_string()
    };

    // Build CSS classes
    let mut classes = vec!["step".to_string(), state];
    
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
fn test_steps_basic() {
    let props = StepsProps {
        children: rsx!(
            Step { value: 1, children: rsx!("Step 1") }
            Step { value: 2, children: rsx!("Step 2") }
            Step { value: 3, children: rsx!("Step 3") }
        ),
        id: None,
        class: None,
        orientation: None,
        current_step: None,
    };

    let result = dioxus_ssr::render_element(Steps(props));
    assert!(result.contains(r#"class="steps steps-vertical""#));
}

#[test]
fn test_steps_horizontal() {
    let props = StepsProps {
        children: rsx!(
            Step { value: 1, children: rsx!("Step 1") }
            Step { value: 2, children: rsx!("Step 2") }
        ),
        id: None,
        class: None,
        orientation: Some(StepsOrientation::Horizontal),
        current_step: None,
    };

    let result = dioxus_ssr::render_element(Steps(props));
    assert!(result.contains(r#"class="steps steps-horizontal""#));
}

#[test]
fn test_steps_with_custom_class() {
    let props = StepsProps {
        children: rsx!(
            Step { value: 1, children: rsx!("Step 1") }
        ),
        id: None,
        class: Some("custom-class".to_string()),
        orientation: None,
        current_step: None,
    };

    let result = dioxus_ssr::render_element(Steps(props));
    assert!(result.contains(r#"class="steps steps-vertical custom-class""#));
}

#[test]
fn test_step_basic() {
    let props = StepProps {
        children: rsx!("Step 1"),
        id: None,
        class: None,
        value: 1,
    };

    let result = dioxus_ssr::render_element(Step(props));
    assert!(result.contains(r#"class="step step-pending""#));
}

#[test]
fn test_step_with_custom_class() {
    let props = StepProps {
        children: rsx!("Step 1"),
        id: None,
        class: Some("custom-step-class".to_string()),
        value: 1,
    };

    let result = dioxus_ssr::render_element(Step(props));
    assert!(result.contains(r#"class="step step-pending custom-step-class""#));
}

#[test]
fn test_step_with_id() {
    let props = StepProps {
        children: rsx!("Step 1"),
        id: Some("test-step".to_string()),
        class: None,
        value: 1,
    };

    let result = dioxus_ssr::render_element(Step(props));
    assert!(result.contains(r#"id="test-step""#));
}
