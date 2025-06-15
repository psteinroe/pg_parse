use std::ffi::{CStr, CString};

use crate::bindings::*;
use crate::error::*;
use crate::protobuf;
use crate::NodeEnum;

use prost::Message;

/// Parses the given SQL statement into the given abstract syntax tree.
///
/// # Example
///
/// ```rust
/// use pg_query::{Node, NodeEnum, NodeRef};
///
/// let result = pg_query::parse("SELECT * FROM contacts");
/// assert!(result.is_ok());
/// let result = result.unwrap();
/// assert_eq!(result.tables(), vec!["contacts"]);
/// assert!(matches!(result.protobuf.nodes()[0].0, NodeRef::SelectStmt(_)));
/// ```
pub fn parse(statement: &str) -> Result<ParseResult> {
    let input = CString::new(statement)?;
    let result = unsafe { pg_query_parse_protobuf(input.as_ptr()) };
    let parse_result = if !result.error.is_null() {
        let message = unsafe { CStr::from_ptr((*result.error).message) }
            .to_string_lossy()
            .to_string();
        Err(Error::Parse(message))
    } else {
        let data = unsafe {
            std::slice::from_raw_parts(
                result.parse_tree.data as *const u8,
                result.parse_tree.len as usize,
            )
        };
        let stderr = unsafe { CStr::from_ptr(result.stderr_buffer) }
            .to_string_lossy()
            .to_string();
        protobuf::ParseResult::decode(data)
            .map_err(Error::Decode)
            .map(|result| ParseResult::new(result, stderr))
    };
    unsafe { pg_query_free_protobuf_parse_result(result) };
    parse_result
}

/// The result of parsing a SQL query
pub struct ParseResult {
    /// The parsed protobuf result
    pub protobuf: protobuf::ParseResult,
    /// Warnings captured during parsing
    pub warnings: Vec<String>,
}

impl ParseResult {
    /// Create a new ParseResult
    pub fn new(protobuf: protobuf::ParseResult, stderr: String) -> Self {
        let warnings = stderr
            .lines()
            .filter_map(|l| {
                if l.starts_with("WARNING") {
                    Some(l.trim().into())
                } else {
                    None
                }
            })
            .collect();

        Self { protobuf, warnings }
    }

    pub fn root(&self) -> Result<&NodeEnum> {
        // Check if we have exactly one statement
        if self.protobuf.stmts.len() != 1 {
            return Err(Error::Parse(format!(
                "Expected exactly one statement, found {}",
                self.protobuf.stmts.len()
            )));
        }

        // Get the first (and only) statement
        let raw_stmt = &self.protobuf.stmts[0];

        // Navigate: RawStmt -> Node -> NodeEnum
        raw_stmt
            .stmt
            .as_ref()
            .and_then(|stmt| stmt.node.as_ref())
            .ok_or_else(|| Error::Parse("Statement contains no node".to_string()))
    }
}
