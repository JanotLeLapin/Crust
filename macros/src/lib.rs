use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Packet)]
pub fn derive_client_bound_packet(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = input.ident;

    let lifetime = if input.generics.params.iter().any(|param| if let syn::GenericParam::Lifetime(_) = param { true } else { false }) {
        quote! { <'a> }
    } else {
        quote! {}
    };

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
                            let s_size = VarInt(self.size() as i32);
                            let mut res = Vec::with_capacity(s_size.size() + 5);
                            res.append(&mut s_size.serialize());
                            res.append(&mut Self::id().serialize());
                            #(res.append(&mut self.#field_name.serialize());)*
                            res
                        }
                    };

                    quote! {
                        impl #lifetime Serialize for #name #lifetime {
                            fn size(&self) -> usize {
                                #size_fn
                            }
                            fn serialize(&self) -> Vec<u8> {
                                #serialize_fn
                            }
                        }
                    }
                },
                _ => { unimplemented!() }
            }
        },
        _ => { unimplemented!() },
    };

    println!("{}", expanded);

    expanded.into()
}
