extern crate proc_macro;

use syn::{Data, DeriveInput, Fields, PathArguments, Type, parse_macro_input};

#[proc_macro_derive(RegisterBits)]
pub fn derive_register_bits(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;

    let field = match &input.data {
        Data::Struct(s) => match &s.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => &fields.unnamed[0],
            _ => {
                return syn::Error::new_spanned(
                    &input,
                    "RegisterBits can only be derived for tuple structs with exactly one field",
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(
                &input,
                "RegisterBits can only be derived for tuple structs",
            )
            .to_compile_error()
            .into();
        }
    };

    let is_register = match &field.ty {
        Type::Path(tp) => {
            let segments = &tp.path.segments;

            matches!(segments.last(), Some(seg) if seg.ident == "Register" && matches!(seg.arguments, PathArguments::None))
        }
        _ => false,
    };

    if !is_register {
        return syn::Error::new_spanned(
            &field.ty,
            "RegisterBits may only be derived for tuple structs wrapping Register",
        )
        .to_compile_error()
        .into();
    }

    quote::quote! {
        impl #ident {
            #[inline(always)]
            fn bit(&self, bit: u8) -> bool {
                (self.0 & (1 << bit)) != 0
            }

            #[inline(always)]
            fn bitset(&mut self, bit: u8) {
                self.0 |= 1 << bit;
            }

            #[inline(always)]
            fn bitclear(&mut self, bit: u8) {
                self.0 &= !(1 << bit);
            }
        }

        impl From<Register> for #ident {
            #[inline(always)]
            fn from(value: Register) -> Self {
                Self(value)
            }
        }

        impl From<#ident> for Register {
            #[inline(always)]
            fn from(value: #ident) -> Self {
                value.0
            }
        }

        impl std::fmt::Display for #ident {
            #[inline(always)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:#x} ({})", self.0, self.0)
            }
        }

        impl std::fmt::Debug for #ident {
            #[inline(always)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:#x} ({})", self.0, self.0)
            }
        }

        impl Copy for #ident {}

        impl Clone for #ident {
            #[inline(always)]
            fn clone(&self) -> Self {
                *self
            }
        }

        impl PartialEq for #ident {
            #[inline(always)]
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl Eq for #ident {}

        impl PartialOrd for #ident {
            #[inline(always)]
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }

        impl Ord for #ident {
            #[inline(always)]
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.0.cmp(&other.0)
            }
        }

        impl std::hash::Hash for #ident {
            #[inline(always)]
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }
    }
    .into()
}
