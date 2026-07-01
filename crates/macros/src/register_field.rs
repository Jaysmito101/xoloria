use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    DeriveInput, Error, Result,
    parse::{Parse, ParseStream},
};

pub(crate) struct RegisterFieldDef(pub(crate) DeriveInput);

impl Parse for RegisterFieldDef {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self(input.parse()?))
    }
}

impl RegisterFieldDef {
    pub(crate) fn expand(self) -> Result<TokenStream> {
        let name = self.0.ident;

        let variants = match self.0.data {
            syn::Data::Enum(e) => e.variants,
            _ => {
                return Err(Error::new_spanned(
                    name,
                    "RegisterField can only be derived on enums",
                ));
            }
        };

        let mut from_arms = Vec::new();
        let mut into_arms = Vec::new();

        for variant in variants {
            let v_name = &variant.ident;
            let disc = &variant.discriminant;
            if let Some((_, expr)) = disc {
                from_arms.push(quote! { #expr => Self::#v_name, });
                into_arms.push(quote! { Self::#v_name => #expr, });
            } else {
                return Err(Error::new_spanned(
                    v_name,
                    "RegisterField requires explicit discriminants",
                ));
            }
        }

        Ok(quote! {
        impl crate::registers::FromBits for #name {
            #[inline(always)]
            fn from_bits(bits: u64) -> Self {
                match bits {
                    #(#from_arms)*
                    _ => panic!(concat!("Invalid bits for ", stringify!(#name), ": {}"), bits),
                }
            }
        }

        impl crate::registers::IntoBits for #name {
            #[inline(always)]
            fn into_bits(self) -> u64 {
                match self {
                    #(#into_arms)*
                }
            }
        }
        })
    }
}
