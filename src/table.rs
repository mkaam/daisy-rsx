#![allow(non_snake_case)]
use std::fmt::Display;
use dioxus::prelude::*;

/// An enhanced table component that provides comprehensive styling options based on DaisyUI table component.
///
/// # Examples
///
/// Basic usage:
///
/// ```text
/// use daisy_rsx::{Table, TableSize};
///
/// Table {
///     size: TableSize::Medium,
///     class: "table-zebra",
///     rsx! {
///         thead {
///             tr {
///                 th { "Name" }
///                 th { "Age" }
///                 th { "City" }
///             }
///         }
///         tbody {
///             tr {
///                 td { "John" }
///                 td { "25" }
///                 td { "New York" }
///             }
///             tr {
///                 td { "Jane" }
///                 td { "30" }
///                 td { "London" }
///             }
///         }
///     }
/// }
/// ```
///
/// With all styling options:
///
/// ```text
/// use daisy_rsx::{Table, TableSize};
///
/// Table {
///     size: TableSize::Large,
///     zebra: true,
///     pin_rows: true,
///     pin_cols: true,
///     row_hover: true,
///     class: "custom-table",
///     rsx! {
///         // Table content here
///     }
/// }
/// ```

/// Size options for Table component
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum TableSize {
    #[default]
    /// Default size (equivalent to Medium)
    Default,
    /// Extra small table size
    ExtraSmall,
    /// Small table size
    Small,
    /// Medium table size
    Medium,
    /// Large table size
    Large,
    /// Extra large table size
    ExtraLarge,
}

impl Display for TableSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableSize::Default => write!(f, ""),
            TableSize::ExtraSmall => write!(f, "table-xs"),
            TableSize::Small => write!(f, "table-sm"),
            TableSize::Medium => write!(f, "table-md"),
            TableSize::Large => write!(f, "table-lg"),
            TableSize::ExtraLarge => write!(f, "table-xl"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TableProps {
    /// The content to display inside the table
    children: Element,
    /// Optional ID for the table element
    id: Option<String>,
    /// Additional CSS classes to apply to the table
    class: Option<String>,
    /// Size of the table
    size: Option<TableSize>,
    /// Whether to apply zebra striping to rows
    zebra: Option<bool>,
    /// Whether to pin (make sticky) header and footer rows
    pin_rows: Option<bool>,
    /// Whether to pin (make sticky) the first column
    pin_cols: Option<bool>,
    /// Whether to apply hover effects to rows
    row_hover: Option<bool>,
}

#[component]
pub fn Table(props: TableProps) -> Element {
    let size = props.size.unwrap_or_default();
    let class = props.class.unwrap_or_default();
    let zebra = props.zebra.filter(|&x| x);
    let pin_rows = props.pin_rows.filter(|&x| x);
    let pin_cols = props.pin_cols.filter(|&x| x);
    let row_hover = props.row_hover.filter(|&x| x);

    // Build CSS classes
    let mut classes = vec!["table".to_string()];
    
    if !size.to_string().is_empty() {
        classes.push(size.to_string());
    }
    
    if zebra.is_some() {
        classes.push("table-zebra".to_string());
    }
    
    if pin_rows.is_some() {
        classes.push("table-pin-rows".to_string());
    }
    
    if pin_cols.is_some() {
        classes.push("table-pin-cols".to_string());
    }
    
    if row_hover.is_some() {
        classes.push("row-hover".to_string());
    }
    
    if !class.is_empty() {
        classes.push(class);
    }

    let class_string = classes.join(" ");

    rsx!(
        table {
            class: "{class_string}",
            id: props.id,
            {props.children}
        }
    )
}

#[test]
fn test_table_basic() {
    let props = TableProps {
        children: rsx!(
            thead {
                tr {
                    th { "Header" }
                }
            }
            tbody {
                tr {
                    td { "Data" }
                }
            }
        ),
        id: None,
        class: None,
        size: None,
        zebra: None,
        pin_rows: None,
        pin_cols: None,
        row_hover: None,
    };

    let result = dioxus_ssr::render_element(Table(props));
    assert!(result.contains(r#"<table class="table""#));
    assert!(result.contains("<th>Header</th>"));
    assert!(result.contains("<td>Data</td>"));
}

#[test]
fn test_table_with_all_props() {
    let props = TableProps {
        children: rsx!(
            thead {
                tr {
                    th { "Name" }
                    th { "Age" }
                }
            }
            tbody {
                tr {
                    td { "John" }
                    td { "25" }
                }
            }
        ),
        id: Some("test-table".to_string()),
        class: Some("custom-class".to_string()),
        size: Some(TableSize::Large),
        zebra: Some(true),
        pin_rows: Some(true),
        pin_cols: Some(true),
        row_hover: Some(true),
    };

    let result = dioxus_ssr::render_element(Table(props));
    assert!(result.contains(r#"<table class="table table-lg table-zebra table-pin-rows table-pin-cols row-hover custom-class""#));
    assert!(result.contains(r#"id="test-table""#));
    assert!(result.contains("<th>Name</th>"));
    assert!(result.contains("<td>John</td>"));
}

#[test]
fn test_all_table_sizes() {
    let sizes = [
        (TableSize::Default, ""),
        (TableSize::ExtraSmall, "table-xs"),
        (TableSize::Small, "table-sm"),
        (TableSize::Medium, "table-md"),
        (TableSize::Large, "table-lg"),
        (TableSize::ExtraLarge, "table-xl"),
    ];

    for (size, expected_class) in sizes {
        let props = TableProps {
            children: rsx!(tr { td { "Test" } }),
            id: None,
            class: None,
            size: Some(size),
            zebra: None,
            pin_rows: None,
            pin_cols: None,
            row_hover: None,
        };

        let result = dioxus_ssr::render_element(Table(props));
        if expected_class.is_empty() {
            // Default size should not add any size class
            assert!(result.contains(r#"<table class="table""#), 
                    "Expected basic table class, but got: {}", result);
        } else {
            assert!(result.contains(expected_class),
                    "Expected '{}' to contain '{}', but got: {}",
                    result, expected_class, result);
        }
    }
}

#[test]
fn test_table_zebra() {
    let props = TableProps {
        children: rsx!(tr { td { "Test" } }),
        id: None,
        class: None,
        size: None,
        zebra: Some(true),
        pin_rows: None,
        pin_cols: None,
        row_hover: None,
    };

    let result = dioxus_ssr::render_element(Table(props));
    assert!(result.contains(r#"<table class="table table-zebra""#));
}

#[test]
fn test_table_pin_rows() {
    let props = TableProps {
        children: rsx!(tr { td { "Test" } }),
        id: None,
        class: None,
        size: None,
        zebra: None,
        pin_rows: Some(true),
        pin_cols: None,
        row_hover: None,
    };

    let result = dioxus_ssr::render_element(Table(props));
    assert!(result.contains(r#"<table class="table table-pin-rows""#));
}

#[test]
fn test_table_pin_cols() {
    let props = TableProps {
        children: rsx!(tr { td { "Test" } }),
        id: None,
        class: None,
        size: None,
        zebra: None,
        pin_rows: None,
        pin_cols: Some(true),
        row_hover: None,
    };

    let result = dioxus_ssr::render_element(Table(props));
    assert!(result.contains(r#"<table class="table table-pin-cols""#));
}

#[test]
fn test_table_row_hover() {
    let props = TableProps {
        children: rsx!(tr { td { "Test" } }),
        id: None,
        class: None,
        size: None,
        zebra: None,
        pin_rows: None,
        pin_cols: None,
        row_hover: Some(true),
    };

    let result = dioxus_ssr::render_element(Table(props));
    assert!(result.contains(r#"<table class="table row-hover""#));
}

#[test]
fn test_table_with_all_features() {
    let props = TableProps {
        children: rsx!(
            thead {
                tr {
                    th { "Header 1" }
                    th { "Header 2" }
                }
            }
            tbody {
                tr {
                    td { "Row 1 Col 1" }
                    td { "Row 1 Col 2" }
                }
                tr {
                    td { "Row 2 Col 1" }
                    td { "Row 2 Col 2" }
                }
            }
        ),
        id: Some("full-feature-table".to_string()),
        class: Some("my-table".to_string()),
        size: Some(TableSize::Medium),
        zebra: Some(true),
        pin_rows: Some(true),
        pin_cols: Some(true),
        row_hover: Some(true),
    };

    let result = dioxus_ssr::render_element(Table(props));
    assert!(result.contains(r#"<table class="table table-md table-zebra table-pin-rows table-pin-cols row-hover my-table""#));
    assert!(result.contains(r#"id="full-feature-table""#));
    assert!(result.contains("<thead>"));
    assert!(result.contains("<tbody>"));
    assert!(result.contains("<th>Header 1</th>"));
    assert!(result.contains("<td>Row 1 Col 1</td>"));
}