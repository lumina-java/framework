use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, ItemStruct, LitStr};

/// Attribute macro untuk mendeklarasikan Validasi Form dengan gaya yang lebih manusiawi.
/// Contoh:
/// ```rust,ignore
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
#[proc_macro_derive(LuminaModel, attributes(table, primary_key, column, lumina))]
pub fn lumina_model_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let name = &input.ident;

    // Ambil nama tabel dari atribut #[table("...")] atau #[lumina(table = "...")]
    let mut table_name = format!("{}s", name.to_string().to_lowercase());
    for attr in &input.attrs {
        if attr.path().is_ident("table") {
            if let Ok(lit) = attr.parse_args::<LitStr>() {
                table_name = lit.value();
            }
        } else if attr.path().is_ident("lumina") {
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("table") {
                    let value = meta.value()?;
                    let s: LitStr = value.parse()?;
                    table_name = s.value();
                    Ok(())
                } else {
                    Ok(())
                }
            });
        }
    }

    // Ambil nama-nama field untuk query SELECT dan INSERT
    let fields = if let syn::Fields::Named(ref fields) = input.fields {
        fields.named.iter().collect::<Vec<_>>()
    } else {
        panic!("LuminaModel hanya bisa digunakan pada struct dengan named fields");
    };

    // Cari nama field primary key (default "id" jika tidak ada atribut #[primary_key])
    let primary_key_field = fields
        .iter()
        .find(|f| f.attrs.iter().any(|attr| attr.path().is_ident("primary_key")))
        .and_then(|f| f.ident.as_ref().map(|i| i.to_string()))
        .unwrap_or_else(|| "id".to_string());

    let field_names: Vec<String> = fields
        .iter()
        .map(|f| {
            // Cek jika ada atribut #[column("...")]
            f.attrs
                .iter()
                .find(|attr| attr.path().is_ident("column"))
                .and_then(|attr| attr.parse_args::<LitStr>().ok())
                .map(|lit| lit.value())
                .unwrap_or_else(|| f.ident.as_ref().unwrap().to_string())
        })
        .collect();

    let primary_key_ident = syn::Ident::new(&primary_key_field, proc_macro2::Span::call_site());

    // Field names untuk SELECT SQLite / MySQL (CAST timestamp ke TEXT agar selalu cocok dengan Option<String>)
    let select_fields = field_names
        .iter()
        .map(|n| {
            if n == "created_at" || n == "updated_at" || n == "deleted_at" {
                format!("CAST({} AS TEXT) as {}", n, n)
            } else {
                n.clone()
            }
        })
        .collect::<Vec<_>>()
        .join(", ");

    // Field names untuk SELECT PostgreSQL (format timestamp dengan to_char agar aman didecode ke Option<String>)
    let pg_select_fields = field_names
        .iter()
        .map(|n| {
            if n == "created_at" || n == "updated_at" || n == "deleted_at" {
                format!("to_char({}, 'YYYY-MM-DD HH24:MI:SS') as {}", n, n)
            } else {
                n.clone()
            }
        })
        .collect::<Vec<_>>()
        .join(", ");

    // Field names untuk INSERT (lewati primary key & timestamp auto)
    let insert_fields: Vec<String> = field_names
        .iter()
        .filter(|&n| n != &primary_key_field && n != "created_at" && n != "updated_at" && n != "deleted_at")
        .cloned()
        .collect();

    let insert_placeholders = vec!["?"; insert_fields.len()].join(", ");
    let insert_sql_fields = insert_fields.join(", ");

    let update_set_clause = insert_fields
        .iter()
        .map(|n| format!("{} = ?", n))
        .collect::<Vec<_>>()
        .join(", ");

    // Bindings untuk INSERT & UPDATE
    let insert_bindings_pg: Vec<_> = insert_fields.iter().map(|n| {
        let ident = syn::Ident::new(n, proc_macro2::Span::call_site());
        quote! { .bind(&self.#ident) }
    }).collect();

    let insert_bindings_other = insert_bindings_pg.clone();

    let update_bindings = insert_fields.iter().map(|n| {
        let ident = syn::Ident::new(n, proc_macro2::Span::call_site());
        quote! { .bind(&self.#ident) }
    });

    let pk_clause = format!("{} = ?", primary_key_field);

    let expanded = quote! {
        #[async_trait::async_trait]
        impl lumina::database::model::Model for #name {
            const TABLE: &'static str = #table_name;

            async fn find(pool: &lumina::database::connection::DatabasePool, id: i64) -> Result<Self, sqlx::Error> {
                let is_pg = pool.kind == lumina::database::connection::DatabaseKind::Postgres;
                let select_cols = if is_pg { #pg_select_fields } else { #select_fields };
                let sql = format!("SELECT {} FROM {} WHERE {} AND deleted_at IS NULL", select_cols, Self::TABLE, #pk_clause);
                let sql = pool.prepare_sql(&sql);
                sqlx::query_as::<_, Self>(&sql)
                    .bind(id)
                    .fetch_one(&pool.pool)
                    .await
            }

            async fn all(pool: &lumina::database::connection::DatabasePool) -> Result<Vec<Self>, sqlx::Error> {
                let is_pg = pool.kind == lumina::database::connection::DatabaseKind::Postgres;
                let select_cols = if is_pg { #pg_select_fields } else { #select_fields };
                let sql = format!("SELECT {} FROM {} WHERE deleted_at IS NULL ORDER BY {} DESC", select_cols, Self::TABLE, #primary_key_field);
                let sql = pool.prepare_sql(&sql);
                sqlx::query_as::<_, Self>(&sql)
                    .fetch_all(&pool.pool)
                    .await
            }

            /// Simpan record ke database. Jika record memiliki ID > 0, lakukan UPDATE. Jika ID == 0, lakukan INSERT.
            async fn save(&self, pool: &lumina::database::connection::DatabasePool) -> Result<i64, sqlx::Error> {
                if self.#primary_key_ident > 0 {
                    let sql = format!(
                        "UPDATE {} SET {}, updated_at = CURRENT_TIMESTAMP WHERE {}",
                        Self::TABLE,
                        #update_set_clause,
                        #pk_clause
                    );
                    let sql = pool.prepare_sql(&sql);
                    sqlx::query(&sql)
                        #(#update_bindings)*
                        .bind(self.#primary_key_ident)
                        .execute(&pool.pool)
                        .await?;
                    Ok(self.#primary_key_ident)
                } else {
                    let is_pg = pool.kind == lumina::database::connection::DatabaseKind::Postgres;
                    if is_pg {
                        let sql = format!(
                            "INSERT INTO {} ({}) VALUES ({}) RETURNING {}",
                            Self::TABLE,
                            #insert_sql_fields,
                            #insert_placeholders,
                            #primary_key_field
                        );
                        let sql = pool.prepare_sql(&sql);
                        let row: (i64,) = sqlx::query_as(&sql)
                            #(#insert_bindings_pg)*
                            .fetch_one(&pool.pool)
                            .await?;
                        Ok(row.0)
                    } else {
                        let sql = format!(
                            "INSERT INTO {} ({}) VALUES ({})",
                            Self::TABLE,
                            #insert_sql_fields,
                            #insert_placeholders
                        );
                        let sql = pool.prepare_sql(&sql);
                        let result = sqlx::query(&sql)
                            #(#insert_bindings_other)*
                            .execute(&pool.pool)
                            .await?;
                        let inserted_id = result.last_insert_id().unwrap_or(0);
                        if inserted_id > 0 {
                            Ok(inserted_id)
                        } else {
                            let sql_max = format!("SELECT COALESCE(MAX({}), 0) FROM {}", #primary_key_field, Self::TABLE);
                            let row: (i64,) = sqlx::query_as(&sql_max).fetch_one(&pool.pool).await.unwrap_or((0,));
                            Ok(row.0)
                        }
                    }
                }
            }

            async fn delete(pool: &lumina::database::connection::DatabasePool, id: i64) -> Result<bool, sqlx::Error> {
                let sql = format!("UPDATE {} SET deleted_at = CURRENT_TIMESTAMP WHERE {}", Self::TABLE, #pk_clause);
                let sql = pool.prepare_sql(&sql);
                sqlx::query(&sql)
                    .bind(id)
                    .execute(&pool.pool)
                    .await?;
                Ok(true)
            }
        }

        impl #name {
            /// Ambil semua record aktif (Laravel: Model::all())
            pub async fn all(pool: &lumina::database::connection::DatabasePool) -> Result<Vec<Self>, sqlx::Error> {
                <Self as lumina::database::model::Model>::all(pool).await
            }

            /// Ambil satu record berdasarkan ID. Mengembalikan `Ok(Some(item))` jika ada, `Ok(None)` jika tidak ada (Laravel: Model::find(id)).
            pub async fn find(pool: &lumina::database::connection::DatabasePool, id: i64) -> Result<Option<Self>, sqlx::Error> {
                match <Self as lumina::database::model::Model>::find(pool, id).await {
                    Ok(item) => Ok(Some(item)),
                    Err(sqlx::Error::RowNotFound) => Ok(None),
                    Err(e) => Err(e),
                }
            }

            /// Ambil satu record atau error jika tidak ada (Laravel: Model::findOrFail(id))
            pub async fn find_or_fail(pool: &lumina::database::connection::DatabasePool, id: i64) -> Result<Self, sqlx::Error> {
                <Self as lumina::database::model::Model>::find(pool, id).await
            }

            /// Hapus record berdasarkan primary key (Laravel: Model::destroy(id))
            pub async fn destroy(pool: &lumina::database::connection::DatabasePool, id: i64) -> Result<bool, sqlx::Error> {
                <Self as lumina::database::model::Model>::delete(pool, id).await
            }

            /// Mulai QueryBuilder dinamis (Laravel: Model::query())
            pub fn query<'a>(pool: &'a lumina::database::connection::DatabasePool) -> lumina::database::orm::QueryBuilder<'a, Self> {
                <Self as lumina::database::model::Model>::query(pool)
            }

            /// Query dengan kondisi WHERE column = value (Laravel: Model::where('col', 'val'))
            pub fn where_eq<'a, V>(pool: &'a lumina::database::connection::DatabasePool, column: &'a str, value: V) -> lumina::database::orm::QueryBuilder<'a, Self>
            where
                V: 'a + Send + sqlx::Encode<'a, sqlx::Any> + sqlx::Type<sqlx::Any>,
            {
                <Self as lumina::database::model::Model>::where_eq(pool, column, value)
            }

            /// Shortcut untuk ORDER BY created_at DESC (Laravel: Model::latest())
            pub fn latest<'a>(pool: &'a lumina::database::connection::DatabasePool) -> lumina::database::orm::QueryBuilder<'a, Self> {
                <Self as lumina::database::model::Model>::latest(pool)
            }

            /// Shortcut WHERE EXISTS relasi (Laravel: Model::whereHas)
            pub fn where_has<'a>(
                pool: &'a lumina::database::connection::DatabasePool,
                relation_table: &'a str,
                foreign_key: &'a str,
                local_key: &'a str,
            ) -> lumina::database::orm::QueryBuilder<'a, Self> {
                <Self as lumina::database::model::Model>::where_has(pool, relation_table, foreign_key, local_key)
            }

            /// Shortcut WHERE NOT EXISTS relasi (Laravel: Model::whereDoesntHave)
            pub fn where_doesnt_have<'a>(
                pool: &'a lumina::database::connection::DatabasePool,
                relation_table: &'a str,
                foreign_key: &'a str,
                local_key: &'a str,
            ) -> lumina::database::orm::QueryBuilder<'a, Self> {
                <Self as lumina::database::model::Model>::where_doesnt_have(pool, relation_table, foreign_key, local_key)
            }

            /// Paginasi data langsung dari model (Laravel: Model::paginate(15))
            pub async fn paginate(
                pool: &lumina::database::connection::DatabasePool,
                page: usize,
                per_page: usize,
            ) -> Result<lumina::database::orm::LengthAwarePaginator<Self>, sqlx::Error> {
                <Self as lumina::database::model::Model>::paginate(pool, page, per_page).await
            }

            /// Hitung total record aktif (Laravel: Model::count())
            pub async fn count(pool: &lumina::database::connection::DatabasePool) -> Result<i64, sqlx::Error> {
                <Self as lumina::database::model::Model>::count(pool).await
            }

            /// Simpan perubahan pada instance record ini ke database (Laravel: $model->save())
            pub async fn save(&self, pool: &lumina::database::connection::DatabasePool) -> Result<i64, sqlx::Error> {
                <Self as lumina::database::model::Model>::save(self, pool).await
            }

            /// Soft delete instance record ini (Laravel: $model->delete())
            pub async fn delete(&self, pool: &lumina::database::connection::DatabasePool) -> Result<bool, sqlx::Error> {
                <Self as lumina::database::model::Model>::delete(pool, self.#primary_key_ident).await
            }

            /// Soft delete instance record ini (Laravel: $model->soft_delete())
            pub async fn soft_delete(&self, pool: &lumina::database::connection::DatabasePool) -> Result<bool, sqlx::Error> {
                <Self as lumina::database::model::Model>::delete(pool, self.#primary_key_ident).await
            }

            /// Pulihkan instance record ini dari soft delete (Laravel: $model->restore())
            pub async fn restore(&self, pool: &lumina::database::connection::DatabasePool) -> Result<bool, sqlx::Error> {
                <Self as lumina::database::model::Model>::restore(pool, self.#primary_key_ident).await
            }

            /// Hapus permanen instance record ini dari database (Laravel: $model->forceDelete())
            pub async fn force_delete(&self, pool: &lumina::database::connection::DatabasePool) -> Result<bool, sqlx::Error> {
                <Self as lumina::database::model::Model>::force_delete(pool, self.#primary_key_ident).await
            }

            /// Pulihkan record berdasarkan ID (Laravel: Model::restore(id))
            pub async fn restore_by_id(pool: &lumina::database::connection::DatabasePool, id: i64) -> Result<bool, sqlx::Error> {
                <Self as lumina::database::model::Model>::restore(pool, id).await
            }

            /// Hapus permanen record berdasarkan ID (Laravel: Model::forceDelete(id))
            pub async fn force_delete_by_id(pool: &lumina::database::connection::DatabasePool, id: i64) -> Result<bool, sqlx::Error> {
                <Self as lumina::database::model::Model>::force_delete(pool, id).await
            }

            /// Relasi Has Many (Laravel: $this->hasMany(Post::class, 'user_id'))
            pub async fn has_many<R>(&self, pool: &lumina::database::connection::DatabasePool, foreign_key: &str) -> Result<Vec<R>, sqlx::Error>
            where
                R: lumina::database::model::Model,
            {
                <Self as lumina::database::model::Model>::has_many::<R>(pool, foreign_key, self.#primary_key_ident).await
            }

            /// Relasi Has One (Laravel: $this->hasOne(Profile::class, 'user_id'))
            pub async fn has_one<R>(&self, pool: &lumina::database::connection::DatabasePool, foreign_key: &str) -> Result<R, sqlx::Error>
            where
                R: lumina::database::model::Model,
            {
                <Self as lumina::database::model::Model>::has_one::<R>(pool, foreign_key, self.#primary_key_ident).await
            }

            /// Relasi Belongs To (Laravel: $this->belongsTo(User::class, 'user_id'))
            pub async fn belongs_to<R>(&self, pool: &lumina::database::connection::DatabasePool, foreign_key_id: i64) -> Result<R, sqlx::Error>
            where
                R: lumina::database::model::Model,
            {
                <Self as lumina::database::model::Model>::belongs_to::<R>(pool, foreign_key_id).await
            }

            /// Relasi Belongs To Many (Laravel: $this->belongsToMany(Tag::class, 'post_tag', 'post_id', 'tag_id'))
            pub async fn belongs_to_many<R>(
                &self,
                pool: &lumina::database::connection::DatabasePool,
                pivot_table: &str,
                foreign_key: &str,
                related_key: &str,
            ) -> Result<Vec<R>, sqlx::Error>
            where
                R: lumina::database::model::Model,
            {
                <Self as lumina::database::model::Model>::belongs_to_many::<R>(
                    pool,
                    pivot_table,
                    foreign_key,
                    related_key,
                    self.#primary_key_ident,
                ).await
            }
        }
    };

    TokenStream::from(expanded)
}
