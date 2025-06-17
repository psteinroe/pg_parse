use pg_proto_parser::ProtoFile;
use quote::{format_ident, quote};

use crate::{proto_analyser::ProtoAnalyzer, utils::*};

pub fn node_ref_mod(analyser: ProtoAnalyzer) -> proc_macro2::TokenStream {
    let node_variants = analyser.node_variants();

    let mut from_ref_matches = Vec::new();
    let mut from_enum_matches = Vec::new();
    let mut node_enum_variants = Vec::new();

    for variant in &node_variants {
        let variant_ident = format_ident!("{}", &variant.name);
        let type_ident = format_ident!("{}", &variant.type_name);

        if variant.boxed {
            // For boxed variants, we need to box the cloned value
            from_ref_matches.push(quote! {
                NodeRef::#variant_ident(n) => NodeEnum::#variant_ident(::prost::alloc::boxed::Box::new((*n).clone()))
            });
        } else {
            // For non-boxed variants, clone directly
            from_ref_matches.push(quote! {
                NodeRef::#variant_ident(n) => NodeEnum::#variant_ident((*n).clone())
            });
        }

        node_enum_variants.push(quote! {
            #variant_ident(&'a protobuf::#type_ident)
        });

        from_enum_matches.push(quote! {
            NodeEnum::#variant_ident(n) => NodeRef::#variant_ident(&n)
        });
    }

    quote! {
        #[derive(Debug, Copy, Clone)]
        pub enum NodeRef<'a> {
            #(#node_enum_variants,)*
        }

        impl<'a> From<NodeRef<'a>> for NodeEnum {
            fn from(n: NodeRef<'a>) -> NodeEnum {
                match n {
                    #(#from_ref_matches,)*
                }
            }
        }

        impl<'a> From<&'a NodeEnum> for NodeRef<'a> {
            fn from(n: &'a NodeEnum) -> NodeRef<'a> {
                match n {
                    #(#from_enum_matches,)*
                }
            }
        }
    }
}
