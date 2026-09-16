extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, LitStr, LitBool};

#[proc_macro_derive(FerroxEntity, attributes(ferrox))]
pub fn derive_ferrox_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    
    // Default values for struct attributes
    let mut table_name = struct_name.to_string().to_lowercase();
    let mut role_read = "User".to_string();
    let mut role_write = "Admin".to_string();
    let mut admin_grid = true;
    let mut admin_kanban = false;

    // Parse struct-level attributes
    for attr in &input.attrs {
        if attr.path().is_ident("ferrox") {
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("table") {
                    let value = meta.value()?;
                    let s: LitStr = value.parse()?;
                    table_name = s.value();
                    Ok(())
                } else if meta.path.is_ident("roles") {
                    let _ = meta.parse_nested_meta(|role_meta| {
                        if role_meta.path.is_ident("read") {
                            let v = role_meta.value()?;
                            let s: LitStr = v.parse()?;
                            role_read = s.value();
                        } else if role_meta.path.is_ident("write") {
                            let v = role_meta.value()?;
                            let s: LitStr = v.parse()?;
                            role_write = s.value();
                        }
                        Ok(())
                    });
                    Ok(())
                } else if meta.path.is_ident("admin_ui") {
                     let _ = meta.parse_nested_meta(|ui_meta| {
                        if ui_meta.path.is_ident("grid") {
                            let v = ui_meta.value()?;
                            let b: LitBool = v.parse()?;
                            admin_grid = b.value;
                        } else if ui_meta.path.is_ident("kanban") {
                            let v = ui_meta.value()?;
                            let b: LitBool = v.parse()?;
                            admin_kanban = b.value;
                        }
                        Ok(())
                    });
                    Ok(())
                } else {
                    Ok(())
                }
            });
        }
    }

    // Parse field-level attributes
    let mut fields_schema = Vec::new();
    if let Data::Struct(data_struct) = &input.data {
        if let Fields::Named(fields_named) = &data_struct.fields {
            for field in &fields_named.named {
                let field_name = field.ident.as_ref().unwrap().to_string();
                let field_type_tokens = &field.ty;
                let field_type_str = quote!(#field_type_tokens).to_string();
                
                let mut is_primary_key = false;
                let mut is_searchable = false;
                let mut is_editable = false;
                let mut is_audit_log = false;

                for attr in &field.attrs {
                    if attr.path().is_ident("ferrox") {
                        let _ = attr.parse_nested_meta(|meta| {
                            if meta.path.is_ident("primary_key") { is_primary_key = true; }
                            if meta.path.is_ident("searchable") { is_searchable = true; }
                            if meta.path.is_ident("editable") { is_editable = true; }
                            if meta.path.is_ident("audit_log") { is_audit_log = true; }
                            Ok(())
                        });
                    }
                }

                fields_schema.push(quote! {
                    serde_json::json!({
                        "name": #field_name,
                        "type": #field_type_str,
                        "is_primary_key": #is_primary_key,
                        "is_searchable": #is_searchable,
                        "is_editable": #is_editable,
                        "is_audit_log": #is_audit_log
                    })
                });
            }
        }
    }

    let expanded = quote! {
        impl #struct_name {
            pub fn get_ferrox_schema() -> serde_json::Value {
                serde_json::json!({
                    "entity_name": stringify!(#struct_name),
                    "table_name": #table_name,
                    "roles": {
                        "read": #role_read,
                        "write": #role_write
                    },
                    "admin_ui": {
                        "grid": #admin_grid,
                        "kanban": #admin_kanban
                    },
                    "fields": [
                        #(#fields_schema),*
                    ]
                })
            }
        }
    };

    TokenStream::from(expanded)
}