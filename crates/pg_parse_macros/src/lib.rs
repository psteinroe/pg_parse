use node_mut::node_mut_mod;
use node_ref::node_ref_mod;
use node_structs::node_structs_mod;
use proto_analyser::ProtoAnalyzer;
use quote::quote;
use std::path;

mod node_mut;
mod node_ref;
mod node_structs;
mod proto_analyser;
mod utils;

#[proc_macro]
pub fn node_ref_codegen(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let analyser = ProtoAnalyzer::from(&proto_file_path()).unwrap();

    let node_ref = node_ref_mod(analyser);

    quote! {
        use crate::*;

        #node_ref
    }
    .into()
}

#[proc_macro]
pub fn node_mut_codegen(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let analyser = ProtoAnalyzer::from(&proto_file_path()).unwrap();

    let node_mut = node_mut_mod(analyser);

    quote! {
        use crate::*;

        #node_mut
    }
    .into()
}

#[proc_macro]
pub fn node_structs_codegen(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let analyser = ProtoAnalyzer::from(&proto_file_path()).unwrap();

    let conversions = node_structs_mod(analyser);

    quote! {
        use crate::*;

        #conversions
    }
    .into()
}

fn proto_file_path() -> path::PathBuf {
    // Use the path set by the build script
    path::PathBuf::from(env!("PG_QUERY_PROTO_PATH"))
}
