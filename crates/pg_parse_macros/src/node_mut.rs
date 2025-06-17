use quote::{format_ident, quote};

use crate::proto_analyser::ProtoAnalyzer;

pub fn node_mut_mod(analyser: ProtoAnalyzer) -> proc_macro2::TokenStream {
    let node_variants = analyser.node_variants();

    let mut from_mut_matches = Vec::new();
    let mut from_enum_matches = Vec::new();
    let mut node_enum_variants = Vec::new();

    for variant in &node_variants {
        let variant_ident = format_ident!("{}", &variant.name);
        let type_ident = format_ident!("{}", &variant.type_name);

        if variant.boxed {
            // For boxed variants, we need to box the cloned value
            from_mut_matches.push(quote! {
                NodeMut::#variant_ident(n) => Ok(NodeEnum::#variant_ident(Box::new(n.as_ref().ok_or(err)?.clone())))
            });
        } else {
            // For non-boxed variants, clone directly
            from_mut_matches.push(quote! {
                NodeMut::#variant_ident(n) => Ok(NodeEnum::#variant_ident(n.as_ref().ok_or(err)?.clone()))
            });
        }

        node_enum_variants.push(quote! {
            #variant_ident(*mut protobuf::#type_ident)
        });

        if variant.boxed {
            from_enum_matches.push(quote! {
                NodeEnum::#variant_ident(n) => NodeMut::#variant_ident(&mut **n as *mut _)
            });
        } else {
            from_enum_matches.push(quote! {
                NodeEnum::#variant_ident(n) => NodeMut::#variant_ident(n as *mut _)
            });
        }
    }

    quote! {
        #[derive(Debug, Copy, Clone)]
        pub enum NodeMut {
            #(#node_enum_variants, )*
        }

        impl TryFrom<NodeMut> for NodeEnum {
            type Error = Error;

            fn try_from(value: NodeMut) -> Result<Self> {
                unsafe {
                    let err = Error::InvalidPointer;
                    match value {
                        #(#from_mut_matches,)*
                        _ => Err(Error::InvalidPointer),
                    }
                }
            }
        }

        impl From<&mut NodeEnum> for NodeMut {
            fn from(n: &mut NodeEnum) -> NodeMut {
                match n {
                    #(#from_enum_matches,)*
                }
            }
        }
    }
}
