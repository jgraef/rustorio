use proc_macro2::{TokenStream, Literal};
use syn::{Data, Path, parse_str};
use darling::{FromMeta, FromDeriveInput};
use heck::{KebabCase, SnekCase};
use quote::{format_ident, quote};

#[derive(Debug, FromMeta, Default)]
pub(crate) struct ContainerAttributes  {
    inherits: Option<String>,
    r#abstract: bool,
}

#[derive(FromDeriveInput)]
#[darling(attributes(lua), forward_attrs(allow, doc, cfg))]
pub(crate) struct DeriveOptions {
    ident: syn::Ident,
    attrs: Vec<syn::Attribute>,
    #[darling(default)]
    prototype: ContainerAttributes,
}



pub(crate) fn impl_prototype(_data: Data, options: DeriveOptions) -> TokenStream {
    let struct_ident = options.ident;
    let name_camelcase = struct_ident.to_string();
    let name_kebabcase_lit = Literal::string(&name_camelcase.to_kebab_case());
    let visit_method = format_ident!("visit_{}", name_camelcase.to_snek_case());

    let inherits_type = if let Some(inherits) = &options.prototype.inherits {
        let inherits_path: Path = parse_str(&inherits).unwrap();
        quote! { #inherits_path }
    }
    else {
        quote! { () }
    };

    let type_name = if options.prototype.r#abstract {
        quote! { None }
    }
    else {
        quote! { Some(#name_kebabcase_lit) }
    };

    quote! {
        impl crate::prototypes::Prototype for #struct_ident {
            const TYPE: Option<&'static str> = #type_name;
            type Inherits = #inherits_type;

            fn accept<V: crate::prototypes::Visitor>(&self, visitor: &mut V) -> Result<(), <V as crate::prototypes::Visitor>::Error> {
                visitor.#visit_method(self)
            }
        }
    }
}
