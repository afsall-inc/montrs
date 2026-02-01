//! montrs-validator: Procedural macros for validation in MontRS.
//! This crate provides the `#[derive(Validator)]` macro which generates
//! compile-time validation logic for structs based on field attributes.

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, LitInt, parse_macro_input};

/// Procedural macro to derive validation logic for a struct.
/// Supported attributes:
/// - `#[validator(min_len = N)]`: Validates that a string has at least N characters.
/// - `#[validator(email)]`: Basic check for the presence of an '@' character.
/// - `#[validator(regex = "pattern")]`: Placeholder for regex-based validation.
/// - `#[validator(custom = "fn_name")]`: Calls a custom validation method on the struct.
#[proc_macro_derive(Validator, attributes(validator))]
pub fn derive_validator(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
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
                if attr.path().is_ident("validator") {
                    let _ = attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("min_len") {
                            let value = meta.value()?;
                            let lit: LitInt = value.parse()?;
                            let min = lit.base10_parse::<usize>()?;
                            all_field_validations.push(quote! {
                                if self.#field_name.len() < #min {
                                    errors.push(::montrs_core::ValidatorError::MinLength {
                                        field: #field_name_str,
                                        min: #min,
                                        actual: self.#field_name.len(),
                                    });
                                }
                            });
                        } else if meta.path.is_ident("max_len") {
                            let value = meta.value()?;
                            let lit: LitInt = value.parse()?;
                            let max = lit.base10_parse::<usize>()?;
                            all_field_validations.push(quote! {
                                if self.#field_name.len() > #max {
                                    errors.push(::montrs_core::ValidatorError::MaxLength {
                                        field: #field_name_str,
                                        max: #max,
                                        actual: self.#field_name.len(),
                                    });
                                }
                            });
                        } else if meta.path.is_ident("min") {
                            let value = meta.value()?;
                            let lit: LitInt = value.parse()?;
                            let min = lit.base10_parse::<i64>()?;
                            all_field_validations.push(quote! {
                                if (self.#field_name as i64) < #min {
                                    errors.push(::montrs_core::ValidatorError::Min {
                                        field: #field_name_str,
                                        min: #min,
                                        actual: self.#field_name as i64,
                                    });
                                }
                            });
                        } else if meta.path.is_ident("max") {
                            let value = meta.value()?;
                            let lit: LitInt = value.parse()?;
                            let max = lit.base10_parse::<i64>()?;
                            all_field_validations.push(quote! {
                                if (self.#field_name as i64) > #max {
                                    errors.push(::montrs_core::ValidatorError::Max {
                                        field: #field_name_str,
                                        max: #max,
                                        actual: self.#field_name as i64,
                                    });
                                }
                            });
                        } else if meta.path.is_ident("email") {
                            all_field_validations.push(quote! {
                                if !self.#field_name.contains('@') {
                                    errors.push(::montrs_core::ValidatorError::InvalidEmail {
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
                                    errors.push(::montrs_core::ValidatorError::RegexMismatch {
                                        field: #field_name_str,
                                        pattern: #regex_str,
                                    });
                                }
                            });
                        } else if meta.path.is_ident("custom") {
                            let value = meta.value()?;
                            let lit: syn::LitStr = value.parse()?;
                            let fn_name = syn::Ident::new(&lit.value(), lit.span());
                            all_field_validations.push(quote! {
                                if let Err(msg) = self.#fn_name() {
                                    errors.push(::montrs_core::ValidatorError::Custom {
                                        field: #field_name_str,
                                        message: msg,
                                    });
                                }
                            });
                        }
                        Ok(())
                    });
                }
            }
        }
    }

    // Combine all validations into the final implementation.
    let expanded = quote! {
        #(#regex_statics)*

        impl ::montrs_core::Validator for #name {
            fn validate(&self) -> ::std::result::Result<(), ::std::vec::Vec<::montrs_core::ValidatorError>> {
                let mut errors = ::std::vec::Vec::new();
                
                #(#all_field_validations)*

                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(errors)
                }
            }
        }
    };

    TokenStream::from(expanded)
}
