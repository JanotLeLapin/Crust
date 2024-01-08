use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Packet, attributes(packet_id))]
pub fn derive_client_bound_packet(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = input.ident;

    let lifetime = if let Some(lifetime) = input.generics.lifetimes().next() {
        let lifetime = &lifetime.lifetime;
        quote! { <#lifetime> }
    } else { quote! {} };

    let packet_id = input.attrs.into_iter()
        .find(|attr| attr.path().is_ident("packet_id"))
        .map(|attr| attr.parse_args::<syn::Lit>().unwrap())
        .expect("expected packet_id attribute");

    let expanded = match input.data {
        syn::Data::Struct(data) => {
            match data.fields {
                syn::Fields::Named(fields) => {
                    let size_fn = {
                        let field_name = fields.named.iter().map(|f| &f.ident);
                        quote! {
                            Self::id().size() + #(self.#field_name.size())+*
                        }
                    };

                    let serialize_fn = {
                        let field_name = fields.named.iter().map(|f| &f.ident);
                        quote! {
                            let s_size = crust_protocol::ser::VarInt(self.size() as i32);
                            let mut res = Vec::with_capacity(s_size.size() + 5);
                            res.append(&mut s_size.serialize());
                            res.append(&mut Self::id().serialize());
                            #(res.append(&mut self.#field_name.serialize());)*
                            res
                        }
                    };

                    quote! {
                        impl #lifetime crust_protocol::ser::Serialize for #name #lifetime {
                            fn size(&self) -> usize {
                                #size_fn
                            }
                            fn serialize(&self) -> Vec<u8> {
                                #serialize_fn
                            }
                        }

                        impl #lifetime #name #lifetime {
                            fn id() -> crust_protocol::ser::VarInt { crust_protocol::ser::VarInt(#packet_id) }
                        }
                    }
                },
                _ => { unimplemented!() }
            }
        },
        _ => { unimplemented!() },
    };

    expanded.into()
}
