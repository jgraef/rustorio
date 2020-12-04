#![allow(dead_code)]

mod lua;
mod prototype;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Data};
use darling::FromDeriveInput;


#[proc_macro_derive(FromLuaTable, attributes(lua))]
pub fn derive_from_lua_table(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let options = lua::DeriveOptions::from_derive_input(&input).unwrap();

    match input.data {
        Data::Struct(data) => lua::impl_from_lua_table_for_struct(data, options).into(),
        _ => panic!("FromLuaTable can only be derive on structs"),
    }
}

#[proc_macro_derive(FromLuaValue, attributes(lua))]
pub fn derive_from_lua_value(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let options = lua::DeriveOptions::from_derive_input(&input).unwrap();

    match input.data {
        Data::Enum(data) => lua::impl_from_lua_value_for_enum(data, options).into(),
        Data::Struct(data) => lua::impl_from_lua_value_for_newtype_struct(data, options).into(),
        _ => panic!("FromLuaTable can only be derived on enums or new-type structs."),
    }
}

#[proc_macro_derive(Prototype, attributes(prototype))]
pub fn derive_prototype(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let options = prototype::DeriveOptions::from_derive_input(&input).unwrap();

    prototype::impl_prototype(input.data, options).into()
}
