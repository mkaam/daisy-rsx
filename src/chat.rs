#![allow(non_snake_case)]
use std::fmt::Display;
use dioxus::prelude::*;

/// A Chat component for chat interfaces.
///
/// # Examples
///
/// Basic usage:
///
/// ```text
/// use daisy_rsx::{Chat, ChatBubble, ChatBubbleColor};
///
/// Chat {
///     children: rsx!(
///         ChatBubble {
///             color: ChatBubbleColor::Primary,
///             children: rsx!("Hello!")
///         }
///     )
/// }
/// ```

/// Color options for ChatBubble component
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ChatBubbleColor {
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

impl Display for ChatBubbleColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatBubbleColor::Primary => write!(f, "chat-bubble-primary"),
            ChatBubbleColor::Secondary => write!(f, "chat-bubble-secondary"),
            ChatBubbleColor::Accent => write!(f, "chat-bubble-accent"),
            ChatBubbleColor::Info => write!(f, "chat-bubble-info"),
            ChatBubbleColor::Success => write!(f, "chat-bubble-success"),
            ChatBubbleColor::Warning => write!(f, "chat-bubble-warning"),
            ChatBubbleColor::Error => write!(f, "chat-bubble-error"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ChatProps {
    /// The content to display inside chat (ChatBubble children)
    children: Element,
    /// Optional ID for chat element
    id: Option<String>,
    /// Additional CSS classes to apply to chat
    class: Option<String>,
}

#[component]
pub fn Chat(props: ChatProps) -> Element {
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["chat".to_string()];
    
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
pub struct ChatBubbleProps {
    /// The content to display inside chat bubble
    children: Element,
    /// Optional ID for chat bubble element
    id: Option<String>,
    /// Additional CSS classes to apply to chat bubble
    class: Option<String>,
    /// Color of chat bubble
    color: Option<ChatBubbleColor>,
}

#[component]
pub fn ChatBubble(props: ChatBubbleProps) -> Element {
    let class = props.class.unwrap_or_default();
    let color = props.color;

    // Build CSS classes
    let mut classes = vec!["chat-bubble".to_string()];
    
    if let Some(c) = color {
        classes.push(c.to_string());
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
pub struct ChatHeaderProps {
    /// The content to display inside chat header
    children: Element,
    /// Optional ID for chat header element
    id: Option<String>,
    /// Additional CSS classes to apply to chat header
    class: Option<String>,
}

#[component]
pub fn ChatHeader(props: ChatHeaderProps) -> Element {
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["chat-header".to_string()];
    
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
pub struct ChatFooterProps {
    /// The content to display inside chat footer
    children: Element,
    /// Optional ID for chat footer element
    id: Option<String>,
    /// Additional CSS classes to apply to chat footer
    class: Option<String>,
}

#[component]
pub fn ChatFooter(props: ChatFooterProps) -> Element {
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["chat-footer".to_string()];
    
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
fn test_chat_basic() {
    let props = ChatProps {
        children: rsx!(
            ChatBubble { children: rsx!("Hello!") }
        ),
        id: None,
        class: None,
    };

    let result = dioxus_ssr::render_element(Chat(props));
    assert!(result.contains(r#"class="chat""#));
}

#[test]
fn test_chat_bubble_basic() {
    let props = ChatBubbleProps {
        children: rsx!("Message"),
        id: None,
        class: None,
        color: None,
    };

    let result = dioxus_ssr::render_element(ChatBubble(props));
    assert!(result.contains(r#"class="chat-bubble""#));
}

#[test]
fn test_chat_bubble_color() {
    let colors = vec![
        ChatBubbleColor::Primary,
        ChatBubbleColor::Secondary,
        ChatBubbleColor::Accent,
        ChatBubbleColor::Info,
        ChatBubbleColor::Success,
        ChatBubbleColor::Warning,
        ChatBubbleColor::Error,
    ];

    for color in colors {
        let props = ChatBubbleProps {
            children: rsx!("Message"),
            id: None,
            class: None,
            color: Some(color),
        };

        let result = dioxus_ssr::render_element(ChatBubble(props));
        assert!(result.contains(&format!("class=\"chat-bubble {}\"", color.to_string())));
    }
}

#[test]
fn test_chat_header() {
    let props = ChatHeaderProps {
        children: rsx!("Chat Header"),
        id: None,
        class: None,
    };

    let result = dioxus_ssr::render_element(ChatHeader(props));
    assert!(result.contains(r#"class="chat-header""#));
}

#[test]
fn test_chat_footer() {
    let props = ChatFooterProps {
        children: rsx!("Chat Footer"),
        id: None,
        class: None,
    };

    let result = dioxus_ssr::render_element(ChatFooter(props));
    assert!(result.contains(r#"class="chat-footer""#));
}

#[test]
fn test_chat_custom_class() {
    let props = ChatProps {
        children: rsx!(
            ChatBubble { children: rsx!("Message") }
        ),
        id: None,
        class: Some("custom-class".to_string()),
    };

    let result = dioxus_ssr::render_element(Chat(props));
    assert!(result.contains(r#"class="chat custom-class""#));
}

#[test]
fn test_chat_with_id() {
    let props = ChatProps {
        children: rsx!(
            ChatBubble { children: rsx!("Message") }
        ),
        id: Some("test-chat".to_string()),
        class: None,
    };

    let result = dioxus_ssr::render_element(Chat(props));
    assert!(result.contains(r#"id="test-chat""#));
}
