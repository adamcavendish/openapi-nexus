//! Template functions module

pub mod do_not_edit;
pub mod http_method_body;
pub mod model_helpers;

pub use do_not_edit::do_not_edit;
pub use http_method_body::http_method_body;
pub use model_helpers::{from_json_line, instance_guard_line, to_json_line};
