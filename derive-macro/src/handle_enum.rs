use super::generate_fields;
use proc_macro2::{Ident, Literal, TokenStream};
use quote::quote;
use syn::{DataEnum, Variant};

fn variant_weight(variant: &Variant) -> Literal {
    for attr in variant.attrs.iter() {
        if attr.path.is_ident("weight") {
            return attr
                .parse_args::<Literal>()
                .expect("expected literal for `#[weight(...)]`");
        }
    }
    Literal::u64_suffixed(1)
}

pub fn generate(name: &Ident, ty: DataEnum) -> TokenStream {
    let variant_weights = ty
        .variants
        .into_iter()
        .enumerate()
        .map(|(i, variant)| (i, variant_weight(&variant), variant));

    let mut arms = TokenStream::new();
    let mut arms_variant = TokenStream::new();
    let mut arms_variant_name = TokenStream::new();
    let mut num_variants: usize = 0;

    let mut total_weight = quote! { 0 };
    for (index, weight, variant) in variant_weights {
        let variant_name = variant.ident;
        arms.extend(quote! {
            let start = end;
            let end = start + #weight;
            if start <= value && value < end {
                return generate_random::GenerateRandomVariant::generate_random_variant(rng, #index);
            }
        });

        let fields = generate_fields(variant.fields);
        arms_variant.extend(quote! {
            #index => Self::#variant_name #fields,
        });

        let variant_str = variant_name.to_string();
        arms_variant_name.extend(quote! {
            #index => #variant_str,
        });

        total_weight = quote! { #total_weight + #weight };
        num_variants += 1;
    }

    quote! {
        impl generate_random::GenerateRandom for #name {
            fn generate_random<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
                let total_weight = #total_weight;
                let value = rng.gen_range(0..total_weight);
                let end = 0;
                #arms
                unreachable!()
            }
        }

        impl generate_random::GenerateRandomVariant for #name {
            fn num_variants() -> usize {
                #num_variants
            }

            fn variant_name(variant: usize) -> &'static str {
                match variant {
                    #arms_variant_name
                    _ => "",
                }
            }

            fn generate_random_variant<R: rand::Rng + ?Sized>(rng: &mut R, variant: usize) -> Self {
                match variant {
                    #arms_variant
                    _ => generate_random::GenerateRandom::generate_random(rng),
                }
            }
        }
    }
}
