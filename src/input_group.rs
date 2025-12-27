#![allow(non_snake_case)]
use std::fmt::Display;
use dioxus::prelude::*;

/// An Input Group component for grouping inputs with buttons, selects, or icons.
///
/// # Examples
///
/// Basic usage with button:
///
/// ```text
/// use daisy_rsx::{InputGroup, InputGroupButton, InputGroupInput};
///
/// InputGroup {
///     children: rsx!(
///         InputGroupInput {
///             input_type: "text".to_string(),
///             placeholder: "Search...".to_string()
///         }
///         InputGroupButton {
///             button_type: "submit".to_string(),
///             children: rsx!("Search")
///         }
///     )
/// }
/// ```
///
/// With select:
///
/// ```text
/// use daisy_rsx::{InputGroup, InputGroupInput, InputGroupSelect, InputGroupOption};
///
/// InputGroup {
///     children: rsx!(
///         InputGroupSelect { children: rsx!(
///             InputGroupOption { value: "1", children: rsx!("Option 1") }
///             InputGroupOption { value: "2", children: rsx!("Option 2") }
///         )}
///         InputGroupInput {
///             input_type: "text".to_string(),
///             placeholder: "Enter value...".to_string()
///         }
///     )
/// }
/// ```

/// Size options for Input Group component
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InputGroupSize {
    /// Small size
    Small,
    /// Medium size
    Medium,
    /// Large size
    Large,
}

impl Display for InputGroupSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputGroupSize::Small => write!(f, "input-group-sm"),
            InputGroupSize::Medium => write!(f, "input-group-md"),
            InputGroupSize::Large => write!(f, "input-group-lg"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct InputGroupProps {
    /// The content to display inside input group (InputGroupInput, InputGroupButton, InputGroupSelect, InputGroupIcon children)
    children: Element,
    /// Optional ID for input group element
    id: Option<String>,
    /// Additional CSS classes to apply to input group
    class: Option<String>,
    /// Size of input group
    size: Option<InputGroupSize>,
    /// Vertical layout
    vertical: Option<bool>,
}

#[component]
pub fn InputGroup(props: InputGroupProps) -> Element {
    let class = props.class.unwrap_or_default();
    let size = props.size;
    let vertical = props.vertical.filter(|&x| x);

    // Build CSS classes
    let mut classes = vec!["input-group".to_string()];
    
    if let Some(s) = size {
        classes.push(s.to_string());
    }
    
    if vertical.is_some() {
        classes.push("input-group-vertical".to_string());
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
pub struct InputGroupInputProps {
    /// Input type (text, password, email, number, etc.)
    input_type: String,
    /// Placeholder text
    placeholder: String,
    /// Optional ID for input element
    id: Option<String>,
    /// Additional CSS classes to apply to input
    class: Option<String>,
    /// Input name
    name: Option<String>,
    /// Input value
    value: Option<String>,
    /// Disabled state
    disabled: Option<bool>,
    /// Required state
    required: Option<bool>,
    /// Read-only state
    readonly: Option<bool>,
}

#[component]
pub fn InputGroupInput(props: InputGroupInputProps) -> Element {
    let class = props.class.unwrap_or_default();
    let disabled = props.disabled.filter(|&x| x);
    let required = props.required.filter(|&x| x);
    let readonly = props.readonly.filter(|&x| x);

    // Build CSS classes
    let mut classes = vec!["input-group-input".to_string()];
    
    if !class.is_empty() {
        classes.push(class);
    }

    let class_string = classes.join(" ");

    rsx!(
        input {
            class: "{class_string}",
            id: props.id,
            type: "{props.input_type}",
            placeholder: "{props.placeholder}",
            name: props.name,
            value: props.value,
            disabled: disabled,
            required: required,
            readonly: readonly,
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct InputGroupButtonProps {
    /// Button type (button, submit, reset)
    button_type: String,
    /// The content to display inside button
    children: Element,
    /// Optional ID for button element
    id: Option<String>,
    /// Additional CSS classes to apply to button
    class: Option<String>,
    /// Disabled state
    disabled: Option<bool>,
}

#[component]
pub fn InputGroupButton(props: InputGroupButtonProps) -> Element {
    let class = props.class.unwrap_or_default();
    let disabled = props.disabled.filter(|&x| x);

    // Build CSS classes
    let mut classes = vec!["input-group-button".to_string()];
    
    if !class.is_empty() {
        classes.push(class);
    }

    let class_string = classes.join(" ");

    rsx!(
        button {
            class: "{class_string}",
            id: props.id,
            type: "{props.button_type}",
            disabled: disabled,
            {props.children}
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct InputGroupSelectProps {
    /// The content to display inside select (InputGroupOption children)
    children: Element,
    /// Optional ID for select element
    id: Option<String>,
    /// Additional CSS classes to apply to select
    class: Option<String>,
    /// Select name
    name: Option<String>,
    /// Disabled state
    disabled: Option<bool>,
    /// Required state
    required: Option<bool>,
}

#[component]
pub fn InputGroupSelect(props: InputGroupSelectProps) -> Element {
    let class = props.class.unwrap_or_default();
    let disabled = props.disabled.filter(|&x| x);
    let required = props.required.filter(|&x| x);

    // Build CSS classes
    let mut classes = vec!["input-group-select".to_string()];
    
    if !class.is_empty() {
        classes.push(class);
    }

    let class_string = classes.join(" ");

    rsx!(
        select {
            class: "{class_string}",
            id: props.id,
            name: props.name,
            disabled: disabled,
            required: required,
            {props.children}
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct InputGroupOptionProps {
    /// Option value
    value: String,
    /// The content to display inside option
    children: Element,
    /// Optional ID for option element
    id: Option<String>,
    /// Additional CSS classes to apply to option
    class: Option<String>,
    /// Selected state
    selected: Option<bool>,
    /// Disabled state
    disabled: Option<bool>,
}

#[component]
pub fn InputGroupOption(props: InputGroupOptionProps) -> Element {
    let class = props.class.unwrap_or_default();
    let selected = props.selected.filter(|&x| x);
    let disabled = props.disabled.filter(|&x| x);

    // Build CSS classes
    let mut classes = vec!["input-group-option".to_string()];
    
    if !class.is_empty() {
        classes.push(class);
    }

    let class_string = classes.join(" ");

    rsx!(
        option {
            class: "{class_string}",
            id: props.id,
            value: "{props.value}",
            selected: selected,
            disabled: disabled,
            {props.children}
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct InputGroupIconProps {
    /// The content to display inside icon (typically an SVG or icon component)
    children: Element,
    /// Optional ID for icon element
    id: Option<String>,
    /// Additional CSS classes to apply to icon
    class: Option<String>,
}

#[component]
pub fn InputGroupIcon(props: InputGroupIconProps) -> Element {
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["input-group-icon".to_string()];
    
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
fn test_input_group_basic() {
    let props = InputGroupProps {
        children: rsx!(
            InputGroupInput {
                input_type: "text".to_string(),
                placeholder: "Search...".to_string()
            }
            InputGroupButton {
                button_type: "submit".to_string(),
                children: rsx!("Search")
            }
        ),
        id: None,
        class: None,
        size: None,
        vertical: None,
    };

    let result = dioxus_ssr::render_element(InputGroup(props));
    assert!(result.contains("input-group"));
}

#[test]
fn test_input_group_input() {
    let props = InputGroupInputProps {
        input_type: "text".to_string(),
        placeholder: "Enter text...".to_string(),
        id: None,
        class: None,
        name: None,
        value: None,
        disabled: None,
        required: None,
        readonly: None,
    };

    let result = dioxus_ssr::render_element(InputGroupInput(props));
    assert!(result.contains("input-group-input"));
    assert!(result.contains(r#"type="text""#));
    assert!(result.contains(r#"placeholder="Enter text...""#));
}

#[test]
fn test_input_group_button() {
    let props = InputGroupButtonProps {
        button_type: "submit".to_string(),
        children: rsx!("Submit"),
        id: None,
        class: None,
        disabled: None,
    };

    let result = dioxus_ssr::render_element(InputGroupButton(props));
    assert!(result.contains("input-group-button"));
    assert!(result.contains(r#"type="submit""#));
    assert!(result.contains("Submit"));
}

#[test]
fn test_input_group_select() {
    let props = InputGroupSelectProps {
        children: rsx!(
            InputGroupOption { value: "1", children: rsx!("Option 1") }
            InputGroupOption { value: "2", children: rsx!("Option 2") }
        ),
        id: None,
        class: None,
        name: None,
        disabled: None,
        required: None,
    };

    let result = dioxus_ssr::render_element(InputGroupSelect(props));
    assert!(result.contains("input-group-select"));
}

#[test]
fn test_input_group_option() {
    let props = InputGroupOptionProps {
        value: "1".to_string(),
        children: rsx!("Option 1"),
        id: None,
        class: None,
        selected: None,
        disabled: None,
    };

    let result = dioxus_ssr::render_element(InputGroupOption(props));
    assert!(result.contains("input-group-option"));
    assert!(result.contains(r#"value="1""#));
    assert!(result.contains("Option 1"));
}

#[test]
fn test_input_group_icon() {
    let props = InputGroupIconProps {
        children: rsx!(span { "🔍" }),
        id: None,
        class: None,
    };

    let result = dioxus_ssr::render_element(InputGroupIcon(props));
    assert!(result.contains("input-group-icon"));
}

#[test]
fn test_input_group_with_size() {
    let props = InputGroupProps {
        children: rsx!(
            InputGroupInput {
                input_type: "text".to_string(),
                placeholder: "Search...".to_string()
            }
        ),
        id: None,
        class: None,
        size: Some(InputGroupSize::Large),
        vertical: None,
    };

    let result = dioxus_ssr::render_element(InputGroup(props));
    assert!(result.contains("input-group-lg"));
}

#[test]
fn test_input_group_vertical() {
    let props = InputGroupProps {
        children: rsx!(
            InputGroupInput {
                input_type: "text".to_string(),
                placeholder: "Search...".to_string()
            }
        ),
        id: None,
        class: None,
        size: None,
        vertical: Some(true),
    };

    let result = dioxus_ssr::render_element(InputGroup(props));
    assert!(result.contains("input-group-vertical"));
}

#[test]
fn test_input_group_custom_class() {
    let props = InputGroupProps {
        children: rsx!(
            InputGroupInput {
                input_type: "text".to_string(),
                placeholder: "Search...".to_string()
            }
        ),
        id: None,
        class: Some("custom-class".to_string()),
        size: None,
        vertical: None,
    };

    let result = dioxus_ssr::render_element(InputGroup(props));
    assert!(result.contains("input-group") && result.contains("custom-class"));
}

#[test]
fn test_input_group_with_id() {
    let props = InputGroupProps {
        children: rsx!(
            InputGroupInput {
                input_type: "text".to_string(),
                placeholder: "Search...".to_string()
            }
        ),
        id: Some("test-input-group".to_string()),
        class: None,
        size: None,
        vertical: None,
    };

    let result = dioxus_ssr::render_element(InputGroup(props));
    assert!(result.contains(r#"id="test-input-group""#));
}

#[test]
fn test_input_group_input_disabled() {
    let props = InputGroupInputProps {
        input_type: "text".to_string(),
        placeholder: "Enter text...".to_string(),
        id: None,
        class: None,
        name: None,
        value: None,
        disabled: Some(true),
        required: None,
        readonly: None,
    };

    let result = dioxus_ssr::render_element(InputGroupInput(props));
    assert!(result.contains("disabled"));
}

#[test]
fn test_input_group_input_required() {
    let props = InputGroupInputProps {
        input_type: "text".to_string(),
        placeholder: "Enter text...".to_string(),
        id: None,
        class: None,
        name: None,
        value: None,
        disabled: None,
        required: Some(true),
        readonly: None,
    };

    let result = dioxus_ssr::render_element(InputGroupInput(props));
    assert!(result.contains("required"));
}

#[test]
fn test_input_group_input_readonly() {
    let props = InputGroupInputProps {
        input_type: "text".to_string(),
        placeholder: "Enter text...".to_string(),
        id: None,
        class: None,
        name: None,
        value: None,
        disabled: None,
        required: None,
        readonly: Some(true),
    };

    let result = dioxus_ssr::render_element(InputGroupInput(props));
    assert!(result.contains("readonly"));
}

#[test]
fn test_input_group_option_selected() {
    let props = InputGroupOptionProps {
        value: "1".to_string(),
        children: rsx!("Option 1"),
        id: None,
        class: None,
        selected: Some(true),
        disabled: None,
    };

    let result = dioxus_ssr::render_element(InputGroupOption(props));
    assert!(result.contains("selected"));
}

#[test]
fn test_input_group_option_disabled() {
    let props = InputGroupOptionProps {
        value: "1".to_string(),
        children: rsx!("Option 1"),
        id: None,
        class: None,
        selected: None,
        disabled: Some(true),
    };

    let result = dioxus_ssr::render_element(InputGroupOption(props));
    assert!(result.contains("disabled"));
}
