extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input};
use quote::quote;

/// Generate conversion functions between enum variants and `libc` constants.
///
/// This attribute is meant for the common use case of defining enums whose variants correspond to
/// constants in `libc`. It differs from the `libc_enum!` macro used in nix 0.14 in that the enum
/// discriminants are unspecified, and instead enum values are meant to be converted into `libc`
/// types via `Into` and `TryFrom`.
///
/// ```
/// use libc_enum::libc_enum;
/// use libc::c_int;
/// use std::convert::TryFrom;
///
/// #[libc_enum(c_int)]
/// enum Signal {
///     SIGINT,
///     SIGTERM,
///     #[cfg(target_os = "linux")]
///     SIGPWR,
/// }
///
/// let sig: c_int = Signal::SIGINT.into();
/// let sig: Signal = Signal::try_from(libc::SIGINT).unwrap();
/// ```
#[proc_macro_attribute]
pub fn libc_enum(args: TokenStream, input: TokenStream) -> TokenStream {
    let repr_type = parse_macro_input!(args as syn::TypePath);
    let enum_def = parse_macro_input!(input as syn::ItemEnum);
    let enum_name = &enum_def.ident;

    let into_impl = {
        let match_arms = enum_def.variants.iter().map(|var| {
            let ident = &var.ident;
            let attrs = &var.attrs;
            quote! {
                #(#attrs)* #enum_name::#ident => ::libc::#ident
            }
        });

        quote! {
            impl std::convert::Into<#repr_type> for #enum_name {
                fn into(self) -> #repr_type {
                    match self {
                        #( #match_arms ),*
                    }
                }
            }
        }
    };

    let try_from_impl = {
        let match_arms = enum_def.variants.iter().map(|var| {
            let ident = &var.ident;
            let attrs = &var.attrs;
            quote! {
                #(#attrs)* ::libc::#ident => Ok(#enum_name::#ident)
            }
        });

        quote! {
            impl std::convert::TryFrom<#repr_type> for #enum_name {
                type Error = ();

                fn try_from(value: #repr_type) -> Result<#enum_name, Self::Error> {
                    match value {
                        #( #match_arms, )*
                        _ => Err(())
                    }
                }
            }
        }
    };

    let expanded = quote! {
        #enum_def
        #into_impl
        #try_from_impl
    };

    TokenStream::from(expanded)
}
