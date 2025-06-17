use pg_proto_parser::Node;
use proc_macro2::Ident;
use quote::format_ident;

pub(crate) fn node_identifiers(nodes: &[Node]) -> Vec<Ident> {
    nodes
        .iter()
        .map(|node| format_ident!("{}", &node.name))
        .collect()
}

// pub(crate) fn enum_variant_names(nodes: &[Node]) -> Vec<Ident> {
//     nodes
//         .iter()
//         .map(|node| format_ident!("{}", &node.enum_variant_name))
//         .collect()
// }
