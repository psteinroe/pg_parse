mod error;
mod node_enum;
mod node_ref;
mod parse;
mod scan;

pub use error::*;
pub use node_enum::*;
pub use node_ref::*;
pub use parse::*;
pub use scan::*;

// Include the generated bindings with 2024 edition compatibility
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(improper_ctypes)]
#[allow(unsafe_op_in_unsafe_fn)]
mod bindings {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/bindings.rs"));
}

// Include the generated protobuf code
#[allow(clippy::all)]
mod protobuf {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/protobuf.rs"));
}
