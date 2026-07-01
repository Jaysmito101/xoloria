use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Attribute, Expr, ExprArray, ExprCall, ExprPath, ExprRange, Ident, Result, Token, Type,
    Visibility,
    parse::{Parse, ParseStream},
};

#[derive(Debug)]
enum SpecifierType {
    Bit(u8),
    Range(u8, u8),
    Bits(Vec<u8>),
}

impl Parse for SpecifierType {
    fn parse(input: ParseStream) -> Result<Self> {
        let call: ExprCall = input.parse()?;
        let func_name = match &*call.func {
            Expr::Path(ExprPath { path, .. }) if path.segments.len() == 1 => {
                path.segments[0].ident.to_string()
            }
            _ => {
                return Err(syn::Error::new_spanned(
                    &call.func,
                    "Expected `bit`, `range`, or `bits`",
                ));
            }
        };

        if call.args.len() != 1 {
            return Err(syn::Error::new_spanned(
                &call.args,
                "Expected exactly 1 argument",
            ));
        }

        let arg = &call.args[0];

        match func_name.as_str() {
            "bit" => {
                if let Expr::Lit(expr_lit) = arg
                    && let syn::Lit::Int(lit_int) = &expr_lit.lit
                {
                    let val: u8 = lit_int.base10_parse()?;
                    return Ok(SpecifierType::Bit(val));
                }
                Err(syn::Error::new_spanned(
                    arg,
                    "Expected integer literal for bit",
                ))
            }
            "range" => {
                if let Expr::Range(ExprRange {
                    start: Some(start),
                    end: Some(end),
                    limits,
                    ..
                }) = arg
                {
                    let start_val: u8 = if let Expr::Lit(expr_lit) = &**start {
                        if let syn::Lit::Int(lit_int) = &expr_lit.lit {
                            lit_int.base10_parse()?
                        } else {
                            return Err(syn::Error::new_spanned(start, "Expected integer literal"));
                        }
                    } else {
                        return Err(syn::Error::new_spanned(start, "Expected integer literal"));
                    };

                    let end_val: u8 = if let Expr::Lit(expr_lit) = &**end {
                        if let syn::Lit::Int(lit_int) = &expr_lit.lit {
                            lit_int.base10_parse()?
                        } else {
                            return Err(syn::Error::new_spanned(end, "Expected integer literal"));
                        }
                    } else {
                        return Err(syn::Error::new_spanned(end, "Expected integer literal"));
                    };

                    let end_val = match limits {
                        syn::RangeLimits::HalfOpen(_) => {
                            if end_val == 0 {
                                return Err(syn::Error::new_spanned(
                                    limits,
                                    "Half-open range ending at 0 is invalid",
                                ));
                            }
                            end_val - 1
                        }
                        syn::RangeLimits::Closed(_) => end_val,
                    };
                    return Ok(SpecifierType::Range(start_val, end_val));
                }
                Err(syn::Error::new_spanned(
                    arg,
                    "Expected range expression (e.g., 0..=3)",
                ))
            }
            "bits" => {
                if let Expr::Array(ExprArray { elems, .. }) = arg {
                    let mut bits = Vec::new();
                    for elem in elems {
                        if let Expr::Lit(expr_lit) = elem
                            && let syn::Lit::Int(lit_int) = &expr_lit.lit
                        {
                            bits.push(lit_int.base10_parse()?);
                            continue;
                        }
                        return Err(syn::Error::new_spanned(elem, "Expected integer literal"));
                    }
                    return Ok(SpecifierType::Bits(bits));
                }
                Err(syn::Error::new_spanned(
                    arg,
                    "Expected array expression (e.g., [0, 2, 4])",
                ))
            }
            _ => Err(syn::Error::new_spanned(
                &call.func,
                "Expected `bit`, `range`, or `bits`",
            )),
        }
    }
}

struct RegisterField {
    attrs: Vec<Attribute>,
    _vis: Visibility,
    name: Ident,
    _colon_token: Token![:],
    ty: Type,
    _eq_token: Token![=],
    specifier: SpecifierType,
}

impl Parse for RegisterField {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            _vis: input.parse()?,
            name: input.parse()?,
            _colon_token: input.parse()?,
            ty: input.parse()?,
            _eq_token: input.parse()?,
            specifier: input.parse()?,
        })
    }
}

pub(crate) struct RegisterDef {
    vis: Visibility,
    _register_token: Ident,
    name: Ident,
    _brace_token: syn::token::Brace,
    fields: syn::punctuated::Punctuated<RegisterField, Token![,]>,
}

impl Parse for RegisterDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let vis: Visibility = input.parse()?;
        let register_token: Ident = input.parse()?;
        if register_token != "register" {
            return Err(syn::Error::new_spanned(
                register_token,
                "expected `register` keyword",
            ));
        }
        let name: Ident = input.parse()?;
        let content;
        let brace_token = syn::braced!(content in input);
        let fields = content.parse_terminated(RegisterField::parse, Token![,])?;
        Ok(Self {
            vis,
            _register_token: register_token,
            name,
            _brace_token: brace_token,
            fields,
        })
    }
}

impl RegisterDef {
    pub(crate) fn expand(self) -> Result<TokenStream> {
        let vis = &self.vis;
        let name = &self.name;

        let mut methods = Vec::new();
        let mut debug_fields = Vec::new();

        for field in self.fields {
            let f_name = &field.name;
            let f_ty = &field.ty;
            let f_attrs = &field.attrs;

            let set_name = format_ident!("set_{}", f_name);
            let with_name = format_ident!("with_{}", f_name);

            let spec = &field.specifier;

            let (get_impl, set_impl) = match spec {
                SpecifierType::Bit(b) => (
                    quote! {
                        let val = (self.0 >> #b) & 1;
                        <#f_ty as crate::registers::FromBits>::from_bits(val)
                    },
                    quote! {
                        let val = <#f_ty as crate::registers::IntoBits>::into_bits(value);
                        self.0 = (self.0 & !(1 << #b)) | ((val & 1) << #b);
                    },
                ),
                SpecifierType::Range(start, end) => {
                    let len = end - start + 1;
                    let mask: u64 = if len == 64 { !0 } else { (1 << len) - 1 };
                    (
                        quote! {
                            let val = (self.0 >> #start) & #mask;
                            <#f_ty as crate::registers::FromBits>::from_bits(val)
                        },
                        quote! {
                            let val = <#f_ty as crate::registers::IntoBits>::into_bits(value);
                            self.0 = (self.0 & !(#mask << #start)) | ((val & #mask) << #start);
                        },
                    )
                }
                SpecifierType::Bits(bits) => {
                    let mut get_parts = Vec::new();
                    let mut set_parts = Vec::new();
                    let mut clear_mask: u64 = 0;

                    for (i, &b) in bits.iter().enumerate() {
                        let i = i as u8;
                        get_parts.push(quote! { ((self.0 >> #b) & 1) << #i });
                        set_parts.push(quote! { ((val >> #i) & 1) << #b });
                        clear_mask |= 1 << b;
                    }

                    let get_expr = if get_parts.is_empty() {
                        quote!(0)
                    } else {
                        quote!( #(#get_parts)|* )
                    };
                    let set_expr = if set_parts.is_empty() {
                        quote!(0)
                    } else {
                        quote!( #(#set_parts)|* )
                    };

                    (
                        quote! {
                            let val = #get_expr;
                            <#f_ty as crate::registers::FromBits>::from_bits(val)
                        },
                        quote! {
                            let val = <#f_ty as crate::registers::IntoBits>::into_bits(value);
                            self.0 = (self.0 & !#clear_mask) | (#set_expr);
                        },
                    )
                }
            };

            let is_bool = if let syn::Type::Path(type_path) = f_ty {
                type_path.path.is_ident("bool")
            } else {
                false
            };

            let without_name = format_ident!("without_{}", f_name);

            let builder_methods = if is_bool {
                quote! {
                    #(#f_attrs)*
                    #[inline(always)]
                    pub fn #with_name(mut self) -> Self {
                        self.#set_name(true);
                        self
                    }

                    #(#f_attrs)*
                    #[inline(always)]
                    pub fn #without_name(mut self) -> Self {
                        self.#set_name(false);
                        self
                    }
                }
            } else {
                quote! {
                    #(#f_attrs)*
                    #[inline(always)]
                    pub fn #with_name(mut self, value: #f_ty) -> Self {
                        self.#set_name(value);
                        self
                    }
                }
            };

            methods.push(quote! {
                #(#f_attrs)*
                #[inline(always)]
                pub fn #f_name(&self) -> #f_ty {
                    #get_impl
                }

                #(#f_attrs)*
                #[inline(always)]
                pub fn #set_name(&mut self, value: #f_ty) {
                    #set_impl
                }

                #builder_methods
            });

            debug_fields.push(quote! {
                .field(stringify!(#f_name), &self.#f_name())
            });
        }

        Ok(quote! {
        #[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #vis struct #name(pub crate::registers::Register);

        impl #name {
            #(#methods)*
        }

        impl From<crate::registers::Register> for #name {
            #[inline(always)]
            fn from(value: crate::registers::Register) -> Self {
                Self(value)
            }
        }

        impl From<#name> for crate::registers::Register {
            #[inline(always)]
            fn from(value: #name) -> Self {
                value.0
            }
        }

        impl std::fmt::Display for #name {
            #[inline(always)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Debug::fmt(self, f)
            }
        }

        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!(#name))
                    .field("raw_bits", &format_args!("{:#x}", self.0))
                    #(#debug_fields)*
                    .finish()
            }
        }
        })
    }
}
