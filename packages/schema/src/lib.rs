//! montrs-schema: Procedural macros for schema validation in MontRS.
//! This crate provides the `#[derive(Schema)]` macro which generates
//! compile-time validation logic for structs based on field attributes.

extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, LitInt, parse_macro_input};

/// Procedural macro to derive validation logic for a struct.
/// Supported attributes:
/// - `#[schema(min_len = N)]`: Validates that a string has at least N characters.
/// - `#[schema(email)]`: Basic check for the presence of an '@' character.
/// - `#[schema(regex = "pattern")]`: Placeholder for regex-based validation.
/// - `#[schema(custom = "fn_name")]`: Calls a custom validation method on the struct.
#[proc_macro_derive(Schema, attributes(schema))]
pub fn derive_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match derive_schema_inner(input) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn derive_schema_inner(input: DeriveInput) -> syn::Result<TokenStream2> {
    let name = input.ident;

    let mut all_field_validations = Vec::new();
    let mut regex_statics = Vec::new();

    // Parse the struct data and iterate over named fields.
    if let Data::Struct(syn::DataStruct {
        fields: Fields::Named(fields),
        ..
    }) = input.data
    {
        for f in fields.named {
            let field_name = f.ident.expect("Named fields must have idents");
            let field_name_str = field_name.to_string();

            // Iterate over attributes on each field.
            for attr in f.attrs {
                if attr.path().is_ident("schema") {
                    attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("min_len") {
                            let value = meta.value()?;
                            let lit: LitInt = value.parse()?;
                            let min = lit.base10_parse::<usize>()?;
                            all_field_validations.push(quote! {
                                if self.#field_name.len() < #min {
                                    errors.push(::montrs_core::ValidationError::MinLength {
                                        field: #field_name_str,
                                        min: #min,
                                        actual: self.#field_name.len(),
                                    });
                                }
                            });
                        } else if meta.path.is_ident("email") {
                            all_field_validations.push(quote! {
                                if !self.#field_name.contains('@') {
                                    errors.push(::montrs_core::ValidationError::InvalidEmail {
                                        field: #field_name_str,
                                    });
                                }
                            });
                        } else if meta.path.is_ident("regex") {
                            let value = meta.value()?;
                            let lit: syn::LitStr = value.parse()?;
                            let regex_str = lit.value();

                            // Compile-time validation of the regex pattern.
                            if let Err(e) = regex::Regex::new(&regex_str) {
                                return Err(meta.error(format!("Invalid regex pattern: {}", e)));
                            }

                            // Generate a unique identifier for the static regex.
                            let static_ident = syn::Ident::new(
                                &format!("__REGEX_{}_{}", name, field_name).to_uppercase(),
                                proc_macro2::Span::call_site(),
                            );

                            regex_statics.push(quote! {
                                static #static_ident: ::std::sync::OnceLock<::regex::Regex> = ::std::sync::OnceLock::new();
                            });

                            all_field_validations.push(quote! {
                                let re = #static_ident.get_or_init(|| ::regex::Regex::new(#regex_str).unwrap());
                                if !re.is_match(&self.#field_name) {
                                    errors.push(::montrs_core::ValidationError::RegexMismatch {
                                        field: #field_name_str,
                                        pattern: #regex_str,
                                    });
                                }
                            });
                        } else if meta.path.is_ident("custom") {
                            let value = meta.value()?;
                            let lit: syn::LitStr = value.parse()?;
                            let custom_fn = syn::Ident::new(&lit.value(), lit.span());
                            all_field_validations.push(quote! {
                                if let Err(e) = self.#custom_fn() {
                                    errors.push(::montrs_core::ValidationError::Custom {
                                        field: #field_name_str,
                                        message: e,
                                    });
                                }
                            });
                        }
                        Ok(())
                    })?;
                }
            }
        }
    }

    // Generate the implementation of the Validate trait.
    let expanded = quote! {
        #(#regex_statics)*

        impl ::montrs_core::Validate for #name {
            fn validate(&self) -> Result<(), Vec<::montrs_core::ValidationError>> {
                let mut errors = Vec::new();

                #(#all_field_validations)*

                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(errors)
                }
            }
        }
    };

    Ok(expanded)
}
