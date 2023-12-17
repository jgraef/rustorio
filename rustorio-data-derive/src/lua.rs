use darling::{
    FromDeriveInput,
    FromField,
    FromMeta,
};
use heck::ToKebabCase;
use proc_macro2::{
    Literal,
    TokenStream,
};
use quote::quote;
use syn::{
    ext::IdentExt,
    parse_str,
    DataEnum,
    DataStruct,
    Expr,
    Fields,
    Generics,
    Path,
};

#[derive(Debug, FromField)]
#[darling(attributes(lua))]
pub(crate) struct FieldAttributes {
    #[darling(default)]
    default: bool,

    #[darling(default)]
    default_with: Option<String>,

    #[darling(default)]
    flatten: bool,

    #[darling(default)]
    with: Option<String>,

    #[darling(default)]
    with_context: Option<String>,
}

#[derive(Debug, FromMeta)]
pub(crate) struct ContainerAttributes {}

#[derive(FromDeriveInput)]
#[darling(attributes(lua), forward_attrs(allow, doc, cfg))]
pub(crate) struct DeriveOptions {
    ident: syn::Ident,
    attrs: Vec<syn::Attribute>,
    generics: Generics,
    #[darling(default)]
    lua: Option<ContainerAttributes>,
}

fn impl_field_init(table_key_lit: Literal, field_attributes: FieldAttributes) -> TokenStream {
    let mut code = vec![];

    if field_attributes.flatten {
        code.push(quote! {
            let x = ::rustorio_data::FromLuaTable::from_lua_table(table.clone())?;
        });
    }
    else if let Some(with_context) = field_attributes.with_context {
        let path: Path = parse_str(&with_context).unwrap();
        code.push(quote! {
            let x = #path(&table)?;
        });
    }
    else {
        code.push(quote! {
            let x = table.get::<_, ::rustorio_data::__private::mlua::Value>(#table_key_lit)?;
        });

        if let Some(with) = field_attributes.with {
            let path: Path = parse_str(&with).unwrap();
            code.push(quote! {
                let x = #path(x)?;
            });
        }
        else if field_attributes.default {
            code.push(quote! {
                let x = ::rustorio_data::to_option(x)?.unwrap_or_default();
            });
        }
        else if let Some(default_with) = field_attributes.default_with {
            let expr: Expr = parse_str(&default_with).unwrap();
            code.push(quote! {
                let x = ::rustorio_data::to_option(x)?.unwrap_or_else(|| #expr);
            });
        }
        else {
            code.push(quote! {
                let x = ::rustorio_data::to_result(x, || rustorio_data::Error::missing_field(#table_key_lit))?;
            });
        }
    }

    quote! {
        {
            #(#code)*
            x
        }
    }
}

pub(crate) fn impl_from_lua_table_for_struct(
    data: DataStruct,
    options: DeriveOptions,
) -> TokenStream {
    let struct_ident = options.ident;

    let (impl_generics, ty_generics, where_clause) = options.generics.split_for_impl();

    let struct_init = match data.fields {
        Fields::Named(fields) => {
            let mut field_inits = vec![];

            for field in fields.named {
                let field_attributes = FieldAttributes::from_field(&field).unwrap();
                let field_ident = field.ident.unwrap();
                let table_key_lit = Literal::string(&field_ident.unraw().to_string());

                let field_init = impl_field_init(table_key_lit.clone(), field_attributes);
                field_inits.push(quote! { #field_ident: { log::trace!("Parsing field: {}", #table_key_lit); #field_init }, });
            }

            quote! {
                Self {
                    #(#field_inits)*
                }
            }
        }
        Fields::Unnamed(fields) => {
            let mut field_inits = vec![];

            for (i, field) in fields.unnamed.iter().enumerate() {
                let field_attributes = FieldAttributes::from_field(&field).unwrap();
                let table_key_lit = Literal::usize_unsuffixed(i);

                let field_init = impl_field_init(table_key_lit, field_attributes);
                field_inits.push(quote! { #field_init, });
            }

            quote! {
                Self(
                    #(#field_inits)*
                )
            }
        }
        _ => panic!("Can't derive FromLuaTable for new-type structs."),
    };

    quote! {
        impl #impl_generics ::rustorio_data::FromLuaTable for #struct_ident #ty_generics #where_clause {
            fn from_lua_table(table: ::rustorio_data::__private::mlua::Table) -> Result<Self, ::rustorio_data::Error> {
                Ok(#struct_init)
            }
        }
    }
}

pub(crate) fn impl_from_lua_value_for_newtype_struct(
    data: DataStruct,
    options: DeriveOptions,
) -> TokenStream {
    let struct_ident = options.ident;
    let (impl_generics, ty_generics, where_clause) = options.generics.split_for_impl();

    match data.fields {
        Fields::Unnamed(fields) => {
            if fields.unnamed.len() != 1 {
                panic!("Can't derive FromLuaValue for new-type structs with more than one field.");
            }

            quote! {
                impl #impl_generics ::rustorio_data::FromLuaValue for #struct_ident #ty_generics #where_clause {
                    fn from_lua_value(value: ::rustorio_data::Value) -> Result<Self, ::rustorio_data::Error> {
                        Ok(Self(::rustorio_data::FromLuaValue::from_lua_value(value)?))
                    }
                }
            }
        }
        _ => panic!("Can't derive FromLuaValue for structs that are not new-type"),
    }
}

pub(crate) fn impl_from_lua_value_for_enum(data: DataEnum, options: DeriveOptions) -> TokenStream {
    let enum_ident = options.ident;
    let mut match_arms = vec![];
    let (impl_generics, ty_generics, where_clause) = options.generics.split_for_impl();

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
        impl #impl_generics ::rustorio_data::FromLuaValue for #enum_ident #ty_generics #where_clause {
            fn from_lua_value(value: ::rustorio_data::Value) -> Result<Self, ::rustorio_data::Error> {
                let s = String::from_lua_value(value)?;
                match s.as_str() {
                    #(#match_arms)*
                    _ => Err(Error::other(format!("Enum variant string can't be matched: {}", s)))
                }
            }
        }
    }
}
