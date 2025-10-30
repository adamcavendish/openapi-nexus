//! Template filter for formatting documentation comments

use minijinja::value::ViaDeserialize;

use crate::ast::TsDocComment;
use openapi_nexus_core::traits::{EmissionContext, ToRcDocWithContext};

/// Input type for documentation comment filter
#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum DocCommentInput {
    /// Structured documentation comment
    DocComment(TsDocComment),
    /// Raw string documentation
    String(String),
}

impl DocCommentInput {
    /// Convert to TsDocComment
    fn into_doc_comment(self) -> TsDocComment {
        match self {
            Self::String(s) => TsDocComment::new(s),
            Self::DocComment(doc) => doc,
        }
    }
}

/// Template filter for formatting documentation comments
/// Accepts either a String or a serialized TsDocComment Value
pub fn format_doc_comment_filter(
    value: ViaDeserialize<DocCommentInput>,
    indent: Option<usize>,
    max_line_width: usize,
) -> Result<String, minijinja::Error> {
    let ctx = EmissionContext {
        indent: indent.unwrap_or(0),
        max_line_width,
    };

    let doc_comment = value.0.into_doc_comment();

    doc_comment
        .to_rcdoc_with_context(&ctx)
        .map(|doc| doc.pretty(max_line_width).to_string())
        .map_err(|e| {
            minijinja::Error::new(
                minijinja::ErrorKind::InvalidOperation,
                format!("Failed to render doc comment: {:?}", e),
            )
        })
}

/// Create a format_doc_comment filter with the given max_line_width
pub fn create_format_doc_comment_filter(
    max_line_width: usize,
) -> impl Fn(ViaDeserialize<DocCommentInput>, Option<usize>) -> Result<String, minijinja::Error>
+ Send
+ Sync
+ 'static {
    move |value, indent| format_doc_comment_filter(value, indent, max_line_width)
}
