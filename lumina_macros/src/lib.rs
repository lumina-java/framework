use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, ItemStruct, LitStr};

/// Attribute macro untuk mendeklarasikan Validasi Form dengan gaya yang lebih manusiawi.
/// Contoh:
/// ```rust
/// #[lumina_form]
/// pub struct RegisterForm {
///     #[rule("required|min:3")]
///     pub name: String,
/// }
/// ```
#[proc_macro_attribute]
pub fn lumina_form(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_struct = parse_macro_input!(item as ItemStruct);

    for field in input_struct.fields.iter_mut() {
        let mut new_attrs = Vec::new();
        let mut validate_tokens = Vec::new();

        for attr in field.attrs.drain(..) {
            if attr.path().is_ident("rule") {
                if let Ok(lit_str) = attr.parse_args::<LitStr>() {
                    let rule_str = lit_str.value();
                    let rules: Vec<&str> = rule_str.split('|').collect();

                    let mut min_val: Option<u64> = None;
                    let mut max_val: Option<u64> = None;
                    let mut length_msg = String::from("Format panjang karakter tidak sesuai");

                    for rule in rules {
                        let rule = rule.trim();
                        if rule == "required" {
                            if min_val.is_none() {
                                min_val = Some(1);
                                length_msg = "Field ini wajib diisi".to_string();
                            }
                        } else if rule == "email" {
                            validate_tokens.push(quote! { email(message = "Format email tidak valid") });
                        } else if rule == "url" {
                            validate_tokens.push(quote! { url(message = "Format URL tidak valid") });
                        } else if rule.starts_with("min:") {
                            if let Some(val) = rule.strip_prefix("min:") {
                                if let Ok(num) = val.parse::<u64>() {
                                    min_val = Some(num);
                                    length_msg = format!("Minimal {} karakter", num);
                                }
                            }
                        } else if rule.starts_with("max:") {
                            if let Some(val) = rule.strip_prefix("max:") {
                                if let Ok(num) = val.parse::<u64>() {
                                    max_val = Some(num);
                                    if length_msg == "Field ini wajib diisi" || length_msg == "Format panjang karakter tidak sesuai" {
                                        length_msg = format!("Maksimal {} karakter", num);
                                    }
                                }
                            }
                        }
                    }

                    if min_val.is_some() || max_val.is_some() {
                        let mut length_args = Vec::new();
                        if let Some(min) = min_val {
                            length_args.push(quote! { min = #min });
                        }
                        if let Some(max) = max_val {
                            length_args.push(quote! { max = #max });
                        }
                        length_args.push(quote! { message = #length_msg });
                        
                        validate_tokens.push(quote! { length(#(#length_args),*) });
                    }
                }
            } else {
                new_attrs.push(attr);
            }
        }

        field.attrs = new_attrs;

        if !validate_tokens.is_empty() {
            let validate_attr: Attribute = syn::parse_quote! {
                #[validate(#(#validate_tokens),*)]
            };
            field.attrs.push(validate_attr);
        }
    }

    let name = &input_struct.ident;
    let vis = &input_struct.vis;
    let fields = &input_struct.fields;

    let expanded = quote! {
        #[derive(serde::Deserialize, serde::Serialize, validator::Validate)]
        #vis struct #name #fields
    };

    TokenStream::from(expanded)
}
