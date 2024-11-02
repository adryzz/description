extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, Attribute, Data, DataEnum, DeriveInput, Error, Ident, LitStr, Result};

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

        let result: Result<LitStr> = attr.parse_args();

        vec.push(parse_args(&variant.ident, result, attr)?);
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

fn parse_args(variant_ident: &Ident, result: Result<LitStr>, attr: &Attribute) -> Result<proc_macro2::TokenStream> {
    
    let format = if let Ok(v) = result.clone() {
        v.value().contains('{')
        // use format logic
    } else {true};

    if format {
        #[cfg(feature = "format")]
        {
            let args: proc_macro2::TokenStream = attr.parse_args()?;
            Ok(quote::quote! {
                Self::#variant_ident => ::const_format::formatcp!(#args),
            })
        }

        #[cfg(not(feature = "format"))]
        {
            // check if the first argument is a string literal containing curly brackets
            let args: proc_macro2::TokenStream = attr.parse_args()?;
            let str = args.into_iter()
            .next().map(|s| s.to_string().contains('{'));

            match str {
                Some(true) => Err(Error::new_spanned(attr, "You need the 'format' feature to use format arguments")),
                _ => Err(result.err().unwrap())
            }
        }
        // TODO: give a better error if the user is trying to use format without the proper feature flag
    } else {
        let segment = result.unwrap();
        Ok(quote::quote! {
            Self::#variant_ident => #segment,
        })
    }
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

        let segment: Option<Result<LitStr>> = attr.map(|x| x.parse_args());

        let ident = &variant.ident;
        match segment {
            Some(result) => {
                vec.push(parse_args(&variant.ident, result, attr.unwrap())?);
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