use node_mut::node_mut_mod;
use node_ref::node_ref_mod;
use pg_proto_parser::ProtoParser;
use quote::quote;
use std::path;

mod node_mut;
mod node_ref;
mod utils;

#[proc_macro]
pub fn node_ref_codegen(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parser = ProtoParser::new(&proto_file_path());
    let proto_file = parser.parse();

    let node_ref = node_ref_mod(&proto_file);

    quote! {
        use crate::*;

        #node_ref
    }
    .into()
}

#[proc_macro]
pub fn node_mut_codegen(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parser = ProtoParser::new(&proto_file_path());
    let proto_file = parser.parse();

    let node_ref = node_mut_mod(&proto_file);

    quote! {
        use crate::*;

        #node_ref
    }
    .into()
}

fn proto_file_path() -> path::PathBuf {
    // Use the path set by the build script
    path::PathBuf::from(env!("PG_QUERY_PROTO_PATH"))
}
