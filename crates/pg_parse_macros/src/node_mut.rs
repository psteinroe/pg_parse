use pg_proto_parser::ProtoFile;
use quote::quote;

use crate::utils::*;

pub fn node_mut_mod(proto_file: &ProtoFile) -> proc_macro2::TokenStream {
    let node_identifiers = node_identifiers(&proto_file.nodes);
    quote! {
        #[derive(Debug, Copy, Clone)]
        pub enum NodeMut {
            #(#node_identifiers(*mut protobuf::#node_identifiers), )*
        }

        // TODO
        // impl TryFrom<NodeMut> for NodeEnum {
        //     fn from(n: NodeMut) -> NodeEnum {
        //         match n {
        //             #(NodeRef::#node_identifiers(n) => NodeEnum::#node_identifiers((*n).clone()),)*
        //         }
        //     }
        // }
    }
}
