#![allow(dead_code)]

use darling::{FromDeriveInput, FromMeta, FromField};
use proc_macro2::{TokenStream, Literal};
use syn::{parse_macro_input, parse_str, DeriveInput, Data, DataStruct, DataEnum, Fields, Expr};
use quote::quote;
use heck::KebabCase;


#[derive(Debug, FromField)]
#[darling(attributes(lua))]
struct FieldAttributes {
    #[darling(default)]
    default: bool,

    #[darling(default)]
    default_with: Option<String>,

    #[darling(default)]
    flatten: bool,
}


#[derive(Debug, FromMeta)]
struct ContainerAttributes  {

}

#[derive(FromDeriveInput)]
#[darling(attributes(lua), forward_attrs(allow, doc, cfg))]
struct DeriveOptions {
    ident: syn::Ident,
    attrs: Vec<syn::Attribute>,
    #[darling(default)]
    lua: Option<ContainerAttributes>,
}


#[proc_macro_derive(FromLuaTable, attributes(lua))]
pub fn derive_from_lua_table(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let options = DeriveOptions::from_derive_input(&input).unwrap();

    match input.data {
        Data::Struct(data) => impl_from_lua_table_for_struct(data, options).into(),
        _ => panic!("FromLuaTable can only be derive on structs"),
    }
}


fn impl_field_init(table_key_lit: Literal, field_attributes: FieldAttributes) -> TokenStream {
    if field_attributes.default {
        quote! {
            ::rustorio_data::to_option(table.get::<_, ::mlua::Value>(#table_key_lit)?)?.unwrap_or_default(),
        }
    }
    else if let Some(default_with) = field_attributes.default_with {
        let expr: Expr = parse_str(&default_with).unwrap();
        quote! {
            ::rustorio_data::to_option(table.get::<_, ::mlua::Value>(#table_key_lit)?)?.unwrap_or_else(|| #expr),
        }
    }
    else if field_attributes.flatten {
        quote! {
            ::rustorio_data::FromLuaTable::from_lua_table(table.clone())?,
        }
    }
    else {
        quote! {
            ::rustorio_data::to_result(table.get::<_, ::mlua::Value>(#table_key_lit)?, || Error::missing_field(#table_key_lit))?,
        }
    }
}

fn impl_from_lua_table_for_struct(data: DataStruct, options: DeriveOptions) -> TokenStream {
    let struct_ident = options.ident;

    let struct_init = match data.fields {
        Fields::Named(fields) => {
            let mut field_inits = vec![];

            for field in fields.named {
                let field_attributes = FieldAttributes::from_field(&field).unwrap();
                let field_ident = field.ident.unwrap();
                let table_key_lit = Literal::string(&field_ident.to_string());

                let field_init = impl_field_init(table_key_lit, field_attributes);
                field_inits.push(quote! { #field_ident: #field_init });
            }

            quote! {
                Self {
                    #(#field_inits)*
                }
            }
        },
        Fields::Unnamed(fields) => {
            let mut field_inits = vec![];

            for (i, field) in fields.unnamed.iter().enumerate() {
                let field_attributes = FieldAttributes::from_field(&field).unwrap();
                let table_key_lit = Literal::usize_unsuffixed(i);

                let field_init = impl_field_init(table_key_lit, field_attributes);
                field_inits.push(field_init);
            }

            quote! {
                Self(
                    #(#field_inits)*
                )
            }
        },
        _ => panic!("Can't derive FromLuaTable for new-type structs."),
    };

    quote! {
        impl FromLuaTable for #struct_ident {
            fn from_lua_table(table: ::mlua::Table) -> Result<Self, ::rustorio_data::Error> {
                Ok(#struct_init)
            }
        }
    }
}


#[proc_macro_derive(FromLuaValue, attributes(lua))]
pub fn derive_from_lua_value(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let options = DeriveOptions::from_derive_input(&input).unwrap();

    match input.data {
        Data::Enum(data) => impl_from_lua_value_for_enum(data, options).into(),
        Data::Struct(data) => impl_from_lua_value_for_newtype_struct(data, options).into(),
        _ => panic!("FromLuaTable can only be derived on enums or new-type structs."),
    }
}


fn impl_from_lua_value_for_newtype_struct(data: DataStruct, options: DeriveOptions) -> TokenStream {
    let struct_ident = options.ident;

    match data.fields {
        Fields::Unnamed(fields) => {
            if fields.unnamed.len() != 1 {
                panic!("Can't derive FromLuaValue for new-type structs with more than one field.");
            }

            quote! {
                impl FromLuaValue for #struct_ident {
                    fn from_lua_value(value: Value) -> Result<Self, Error> {
                        Ok(Self(FromLuaValue::from_lua_value(value)?))
                    }
                }
            }
        }
        _ => panic!("Can't derive FromLuaValue for structs that are not new-type"),
    }
}


fn impl_from_lua_value_for_enum(data: DataEnum, options: DeriveOptions) -> TokenStream {
    let enum_ident = options.ident;
    let mut match_arms = vec![];

    for variant in data.variants {
        if variant.fields != Fields::Unit {
            panic!("Can only derive FromLuaValue for simple enums.");
        }

        let variant_ident = variant.ident;
        let variant_name_lit = Literal::string(&variant_ident.to_string().to_kebab_case());

        match_arms.push(quote! {
            #variant_name_lit => Ok(Self::#variant_ident),
        })
    }

    quote! {
        impl FromLuaValue for #enum_ident {
            fn from_lua_value(value: Value) -> Result<Self, Error> {
                let s = String::from_lua_value(value)?;
                match s.as_str() {
                    #(#match_arms)*
                    _ => Err(Error::other(format!("Enum variant string can't be matched: {}", s)))
                }
            }
        }
    }
}
