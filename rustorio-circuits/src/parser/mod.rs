#![allow(dead_code)]

pub mod ast;

use lalrpop_util::lalrpop_mod;
pub use lalrpop_util::ParseError;


lalrpop_mod!(parser, "/parser/parser.rs");


#[cfg(test)]
mod tests {
    use super::parser;

    #[test]
    fn it_parses_a_module_decl() {
        let s = r#"
mod clock<#n> {
    port clock_enable(red ce, _);
    port time(red t, green _);

    wire green tmp;

    tmp[virtual-C] <- if ce[virtual-E] > 0 then 1;

    tmp[virtual-C] <- if tmp[virtual-C] < #n then _;

    t <- diode_rename<virtual-T, virtual-C>(input: t, output: C);
}
        "#;

        match parser::ModDeclParser::new().parse(s) {
            Ok(mod_decl) => {
                println!("{:#?}", mod_decl);
            },
            Err(e) => {
                let rest = e.map_location(|l| &s[l..]);
                eprintln!("Error at: {}", rest);
            }
        }
    }

    #[test]
    fn it_parses_idents() {
        parser::IdentParser::new().parse("foobar").unwrap();
        parser::IdentParser::new().parse("_foo").unwrap();
        parser::IdentParser::new().parse("f").unwrap();
        parser::IdentParser::new().parse("_").unwrap();
    }

}