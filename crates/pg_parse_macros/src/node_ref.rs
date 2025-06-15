use pg_proto_parser::ProtoFile;
use quote::quote;

use crate::utils::*;

pub fn node_ref_mod(proto_file: &ProtoFile) -> proc_macro2::TokenStream {
    let node_identifiers = node_identifiers(&proto_file.nodes);
    quote! {
        #[derive(Debug, Copy, Clone)]
        pub enum NodeRef<'a> {
            #(#node_identifiers(&'a protobuf::#node_identifiers),)*
        }

        impl<'a> From<NodeRef<'a>> for NodeEnum {
            fn from(n: NodeRef<'a>) -> NodeEnum {
                match n {
                    #(NodeRef::#node_identifiers(n) => NodeEnum::#node_identifiers((*n).clone()),)*
                }
            }
        }
    }
}
