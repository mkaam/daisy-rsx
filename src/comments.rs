#![allow(non_snake_case)]
use std::fmt::Display;
use dioxus::prelude::*;

/// A Comments component for displaying comments and discussions.
///
/// # Examples
///
/// Basic usage:
///
/// ```text
/// use daisy_rsx::{Comments, Comment, CommentHeader, CommentBody, CommentActions};
///
/// Comments {
///     children: rsx!(
///         Comment {
///             author: Some("John Doe"),
///             avatar: Some("/avatar1.jpg"),
///             timestamp: Some("2 hours ago"),
///             children: rsx!(
///                 CommentBody { children: rsx!("Great post!") }
///                 CommentActions {
///                     Button { children: rsx!("Like") }
///                     Button { children: rsx!("Reply") }
///                 }
///             )
///         }
///     )
/// }
/// ```

/// Color scheme options for Comments component
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CommentsColorScheme {
    /// Neutral color
    Neutral,
    /// Primary color
    Primary,
    /// Secondary color
    Secondary,
}

impl Display for CommentsColorScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommentsColorScheme::Neutral => write!(f, "chat-neutral"),
            CommentsColorScheme::Primary => write!(f, "chat-primary"),
            CommentsColorScheme::Secondary => write!(f, "chat-secondary"),
        }
    }
}

/// Size options for Comments component
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CommentsSize {
    /// Small size
    Small,
    /// Medium size
    Medium,
    /// Large size
    Large,
}

impl Display for CommentsSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommentsSize::Small => write!(f, "chat-sm"),
            CommentsSize::Medium => write!(f, "chat-md"),
            CommentsSize::Large => write!(f, "chat-lg"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CommentsProps {
    /// The content to display inside comments (Comment children)
    children: Element,
    /// Optional ID for comments element
    id: Option<String>,
    /// Additional CSS classes to apply to comments
    class: Option<String>,
    /// Color scheme for comments
    color_scheme: Option<CommentsColorScheme>,
    /// Size of comments
    size: Option<CommentsSize>,
}

#[component]
pub fn Comments(props: CommentsProps) -> Element {
    let class = props.class.unwrap_or_default();
    let color_scheme = props.color_scheme;
    let size = props.size;

    // Build CSS classes
    let mut classes = vec!["chat".to_string()];
    
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
pub struct CommentProps {
    /// The content to display inside comment (CommentHeader, CommentBody, CommentActions children)
    children: Element,
    /// Optional ID for comment element
    id: Option<String>,
    /// Additional CSS classes to apply to comment
    class: Option<String>,
    /// Author name
    author: Option<String>,
    /// Avatar URL
    avatar: Option<String>,
    /// Timestamp
    timestamp: Option<String>,
    /// Whether comment is liked
    liked: Option<bool>,
    /// Number of replies
    replies: Option<i32>,
    /// Color scheme for comment
    color_scheme: Option<CommentsColorScheme>,
}

#[component]
pub fn Comment(props: CommentProps) -> Element {
    let class = props.class.unwrap_or_default();
    let color_scheme = props.color_scheme;

    // Build CSS classes
    let mut classes = vec!["chat-bubble".to_string()];
    
    if let Some(color) = color_scheme {
        classes.push(color.to_string());
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
pub struct CommentHeaderProps {
    /// The content to display inside comment header
    children: Element,
    /// Optional ID for comment header element
    id: Option<String>,
    /// Additional CSS classes to apply to comment header
    class: Option<String>,
    /// Author name
    author: Option<String>,
    /// Avatar URL
    avatar: Option<String>,
    /// Timestamp
    timestamp: Option<String>,
}

#[component]
pub fn CommentHeader(props: CommentHeaderProps) -> Element {
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
            {props.avatar.as_ref().map(|avatar| rsx!(
                div { class: "chat-image",
                    img { src: "{avatar}", class: "avatar-sm" }
                }
            ))}
            {props.author.as_ref().map(|author| rsx!(div { class: "chat-name", "{author}" }))}
            {props.timestamp.as_ref().map(|timestamp| rsx!(time { class: "chat-time", "{timestamp}" }))}
            {props.children}
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct CommentBodyProps {
    /// The content to display inside comment body
    children: Element,
    /// Optional ID for comment body element
    id: Option<String>,
    /// Additional CSS classes to apply to comment body
    class: Option<String>,
}

#[component]
pub fn CommentBody(props: CommentBodyProps) -> Element {
    let class = props.class.unwrap_or_default();

    // Build CSS classes
    let mut classes = vec!["chat-content".to_string()];
    
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
pub struct CommentActionsProps {
    /// The content to display inside comment actions
    children: Element,
    /// Optional ID for comment actions element
    id: Option<String>,
    /// Additional CSS classes to apply to comment actions
    class: Option<String>,
}

#[component]
pub fn CommentActions(props: CommentActionsProps) -> Element {
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
fn test_comments_basic() {
    let props = CommentsProps {
        children: rsx!(
            Comment {
                children: rsx!(
                    CommentBody { children: rsx!("Great post!") }
                )
            }
        ),
        id: None,
        class: None,
        color_scheme: None,
        size: None,
    };

    let result = dioxus_ssr::render_element(Comments(props));
    assert!(result.contains("chat"));
}

#[test]
fn test_comment_basic() {
    let props = CommentProps {
        children: rsx!(
            CommentBody { children: rsx!("Comment content") }
        ),
        id: None,
        class: None,
        author: None,
        avatar: None,
        timestamp: None,
        liked: None,
        replies: None,
        color_scheme: None,
    };

    let result = dioxus_ssr::render_element(Comment(props));
    assert!(result.contains("chat-bubble"));
}

#[test]
fn test_comment_header() {
    let props = CommentHeaderProps {
        children: rsx!(),
        id: None,
        class: None,
        author: Some("John Doe".to_string()),
        avatar: Some("/avatar.jpg".to_string()),
        timestamp: Some("2 hours ago".to_string()),
    };

    let result = dioxus_ssr::render_element(CommentHeader(props));
    assert!(result.contains("chat-header"));
}

#[test]
fn test_comment_body() {
    let props = CommentBodyProps {
        children: rsx!("Comment content"),
        id: None,
        class: None,
    };

    let result = dioxus_ssr::render_element(CommentBody(props));
    assert!(result.contains("chat-content"));
}

#[test]
fn test_comment_actions() {
    let props = CommentActionsProps {
        children: rsx!(div { "Like" }),
        id: None,
        class: None,
    };

    let result = dioxus_ssr::render_element(CommentActions(props));
    assert!(result.contains("chat-footer"));
}

#[test]
fn test_comments_with_color_scheme() {
    let props = CommentsProps {
        children: rsx!(Comment { children: rsx!(CommentBody { children: rsx!("Content") }) }),
        id: None,
        class: None,
        color_scheme: Some(CommentsColorScheme::Primary),
        size: None,
    };

    let result = dioxus_ssr::render_element(Comments(props));
    assert!(result.contains("chat-primary"));
}

#[test]
fn test_comments_custom_class() {
    let props = CommentsProps {
        children: rsx!(Comment { children: rsx!(CommentBody { children: rsx!("Content") }) }),
        id: None,
        class: Some("custom-class".to_string()),
        color_scheme: None,
        size: None,
    };

    let result = dioxus_ssr::render_element(Comments(props));
    assert!(result.contains("chat") && result.contains("custom-class"));
}
