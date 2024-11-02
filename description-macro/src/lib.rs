extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DataEnum, DeriveInput, Error, ExprLit, Ident, Result};

#[proc_macro_derive(Description, attributes(description))]
pub fn derive_description(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match try_expand(&input) {
        Ok(expanded) => expanded,
        Err(error) => error.to_compile_error().into(),
    }
}

fn try_expand(input: &DeriveInput) -> Result<TokenStream> {
    match &input.data {
        Data::Enum(e) => Ok(impl_enum(&input.ident, e)?),
        _ => Err(Error::new_spanned(input, "Description cannot be implemented on structs or unions"))
    }
}

fn impl_enum(ident: &Ident, input: &DataEnum) -> Result<TokenStream> {
    let mut vec = Vec::with_capacity(input.variants.len());
    for variant in &input.variants {
        let attr = variant.attrs.iter()
        .find(|x| x.path().is_ident("description"))
        .ok_or(syn::Error::new_spanned(variant, "Missing 'description' attribute"))?;

        let segment: ExprLit = attr.parse_args()?;
        let ident = &variant.ident;
        vec.push(quote::quote! {
            Self::#ident => #segment,
        });
    }

    Ok(quote::quote! {
        impl ::description::Description for #ident {
            fn description(&self) -> &'static str {
                match self {
                    #(#vec)*
                }
            }
        }
    }.into())
}

#[proc_macro_derive(OptionalDescription, attributes(description))]
pub fn derive_optional_description(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    match try_expand_optional(&input) {
        Ok(expanded) => expanded,
        Err(error) => error.to_compile_error().into(),
    }
}

fn try_expand_optional(input: &DeriveInput) -> Result<TokenStream> {
    match &input.data {
        Data::Enum(e) => Ok(impl_enum_optional(&input.ident, e)?),
        _ => Err(Error::new_spanned(input, "Description cannot be implemented on structs or unions"))
    }
}

fn impl_enum_optional(ident: &Ident, input: &DataEnum) -> Result<TokenStream> {
    let mut vec = Vec::with_capacity(input.variants.len());

    for variant in &input.variants {
        let attr = variant.attrs.iter()
        .find(|x| x.path().is_ident("description"));

        let segment: Option<ExprLit> = attr.map(|x| x.parse_args()).transpose()?;

        let ident = &variant.ident;
        match segment {
            Some(l) => {
                vec.push(quote::quote! {
                    Self::#ident => Some(#l),
                });
            },
            None => {
                vec.push(quote::quote! {
                    Self::#ident => None,
                });
            }
        }
    }

    Ok(quote::quote! {
        impl ::description::OptionalDescription for #ident {
            fn description(&self) -> Option<&'static str> {
                match self {
                    #(#vec)*
                }
            }
        }
    }.into())
}