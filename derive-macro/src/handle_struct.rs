use super::generate_fields;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::DataStruct;

pub fn generate(name: &Ident, ty: DataStruct) -> TokenStream {
    let fields = generate_fields(ty.fields);
    quote! {
        impl generate_random::GenerateRandom for #name {
            fn generate_random<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
                Self #fields
            }
        }
    }
}
