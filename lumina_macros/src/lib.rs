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
                            validate_tokens
                                .push(quote! { email(message = "Format email tidak valid") });
                        } else if rule == "url" {
                            validate_tokens
                                .push(quote! { url(message = "Format URL tidak valid") });
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
                                    if length_msg == "Field ini wajib diisi"
                                        || length_msg == "Format panjang karakter tidak sesuai"
                                    {
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

/// Derive macro untuk mengimplementasikan trait Model secara otomatis.
/// Ini menghilangkan boilerplate SQL manual untuk operasi CRUD standar.
#[proc_macro_derive(LuminaModel, attributes(table))]
pub fn lumina_model_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let name = &input.ident;

    // Ambil nama tabel dari atribut #[table("...")]
    let table_name = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("table"))
        .and_then(|attr| attr.parse_args::<LitStr>().ok())
        .map(|lit| lit.value())
        .unwrap_or_else(|| format!("{}s", name.to_string().to_lowercase()));

    // Ambil nama-nama field untuk query SELECT dan INSERT
    let fields = if let syn::Fields::Named(ref fields) = input.fields {
        fields.named.iter().collect::<Vec<_>>()
    } else {
        panic!("LuminaModel hanya bisa digunakan pada struct dengan named fields");
    };

    let field_names: Vec<String> = fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap().to_string())
        .collect();

    // Field names untuk SELECT (semua field)
    let select_fields = field_names.join(", ");

    // Field names untuk INSERT (lewati 'id')
    let insert_fields: Vec<String> = field_names
        .iter()
        .filter(|&n| n != "id" && n != "created_at" && n != "updated_at" && n != "deleted_at")
        .cloned()
        .collect();

    let insert_placeholders = vec!["?"; insert_fields.len()].join(", ");
    let insert_sql_fields = insert_fields.join(", ");

    // Bindings untuk INSERT
    let insert_bindings: Vec<_> = insert_fields.iter().map(|n| {
        let ident = syn::Ident::new(n, proc_macro2::Span::call_site());
        quote! { .bind(&self.#ident) }
    }).collect();

    let expanded = quote! {
        #[async_trait::async_trait]
        impl crate::database::model::Model for #name {
            const TABLE: &'static str = #table_name;

            async fn find(pool: &crate::database::connection::DatabasePool, id: i64) -> Result<Self, sqlx::Error> {
                let sql = format!("SELECT {} FROM {} WHERE id = ? AND deleted_at IS NULL", #select_fields, Self::TABLE);
                sqlx::query_as::<_, Self>(&sql)
                    .bind(id)
                    .fetch_one(&pool.pool)
                    .await
            }

            async fn all(pool: &crate::database::connection::DatabasePool) -> Result<Vec<Self>, sqlx::Error> {
                let sql = format!("SELECT {} FROM {} WHERE deleted_at IS NULL ORDER BY id DESC", #select_fields, Self::TABLE);
                sqlx::query_as::<_, Self>(&sql)
                    .fetch_all(&pool.pool)
                    .await
            }

            async fn save(&mut self, pool: &lumina::database::connection::DatabasePool) -> Result<i64, sqlx::Error> {
                if self.id > 0 {
                    let mut update_pairs = Vec::new();
                    #(update_pairs.push(format!("{} = ?", #insert_fields));)*

                    let sql = format!("UPDATE {} SET {} WHERE id = ?", Self::TABLE, update_pairs.join(", "));

                    let result = sqlx::query(&sql)
                        #(#insert_bindings)*
                        .bind(self.id)
                        .execute(&pool.pool)
                        .await?;

                    Ok(self.id)
                } else {
                    let sql = format!(
                        "INSERT INTO {} ({}) VALUES ({})",
                        Self::TABLE,
                        #insert_sql_fields,
                        #insert_placeholders
                    );

                    let result = sqlx::query(&sql)
                        #(#insert_bindings)*
                        .execute(&pool.pool)
                        .await?;

                    let id = result.last_insert_id() as i64;
                    self.id = id;
                    Ok(id)
                }
            }

            async fn delete(pool: &crate::database::connection::DatabasePool, id: i64) -> Result<bool, sqlx::Error> {
                let sql = format!("UPDATE {} SET deleted_at = CURRENT_TIMESTAMP WHERE id = ?", Self::TABLE);
                sqlx::query(&sql)
                    .bind(id)
                    .execute(&pool.pool)
                    .await?;
                Ok(true)
            }
        }
    };

    TokenStream::from(expanded)
}
