#![allow(dead_code)]

pub mod ast;

use std::{
    path::{Path, PathBuf},
    collections::HashSet,
    fs::read_to_string,
};

use lalrpop_util::lalrpop_mod;
use thiserror::Error;
use regex::Regex;
use lazy_static::lazy_static;

use ast::{ImportPath, Unit};


lalrpop_mod!(parser, "/parser/parser.rs");


lazy_static! {
    static ref COMMENT_REGEX: Regex = Regex::new(r"(//.*$)|(/\*[^*]*\*/)").unwrap();
}


type InnerError<'a> = lalrpop_util::ParseError<usize, lalrpop_util::lexer::Token<'a>, &'a str>;


#[derive(Debug, Error)]
pub enum ParseError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Import not found: {import}")]
    ImportNotFound {
        relative_to: Option<PathBuf>,
        import: ImportPath,
    },

    #[error("{0}")]
    Inner(String),
}

#[derive(Clone, Debug)]
pub struct Span {
    start: usize,
    end: usize,
}

impl ParseError {
    fn from_lalrpop_error<P: AsRef<Path>>(source: &str, path: Option<P>, e: InnerError) -> Self {
        let span = match &e {
            InnerError::InvalidToken { location } => {
                Span { start: *location, end: *location }
            },
            InnerError::UnrecognizedEof { location, .. } => {
                Span { start: *location, end: *location }
            },
            InnerError::UnrecognizedToken { token: (start, _, end), .. } => {
                Span { start: *start, end: *end }
            },
            InnerError::ExtraToken { token: (start, _, end) } => {
                Span { start: *start, end: *end }
            },
            InnerError::User { .. } => {
                todo!();
            },
        };

        eprintln!("Error while compiling: path={:?}, span={:?}", path.as_ref().map(|p| p.as_ref()), span);
        eprintln!("{}", &source[span.start..]);

        Self::Inner(format!("{0}", e))
    }
}

pub fn parse_unit(source: &str) -> Result<Unit, ParseError> {
    let mut parser = Parser::new(ParserConfig { import_paths: vec![] });
    parser.parse_unit_from_string::<&Path>(source, None)?;
    Ok(parser.units.pop().unwrap())
}


#[derive(Debug)]
pub struct ParserConfig {
    pub import_paths: Vec<PathBuf>,
}

impl Default for ParserConfig {
    fn default() -> Self {
        ParserConfig {
            import_paths: vec!["./".into()],
        }
    }
}

#[derive(Debug, Default)]
pub struct Parser {
    config: ParserConfig,
    units: Vec<Unit>,
    paths: HashSet<PathBuf>,
}

impl Parser {
    pub fn new(config: ParserConfig) -> Self {
        Self {
            config,
            units: Vec::new(),
            paths: HashSet::new(),
        }
    }

    pub fn into_ast(self) -> Vec<Unit> {
        self.units
    }

    pub fn parse_unit_from_string<P: AsRef<Path>>(&mut self, source: &str, path: Option<P>) -> Result<(), ParseError> {
        if let Some(path) = &path {
            if self.paths.contains(path.as_ref()) {
                return Ok(())
            }
        }

        // FIXME: This is a dirty hack and breaks error reporting somewhat.
        let source = COMMENT_REGEX.replace_all(source, "");

        match parser::UnitParser::new().parse(&source) {
            Ok(mut unit) => {
                unit.file_path = path.as_ref().map(|p| PathBuf::from(p.as_ref()));
                unit.source = Some(source.into_owned());

                if let Some(path) = &path {
                    self.paths.insert(path.as_ref().to_owned());
                }

                for import_path in &unit.imports {
                    self.import(path.as_ref(), &import_path)?;
                }

                self.units.push(unit);

                Ok(())
            },
            Err(e) => Err(ParseError::from_lalrpop_error(&source, path, e)),
        }
    }

    pub fn parse_unit_from_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), ParseError> {
        log::info!("Parsing file: {}", path.as_ref().display());
        let source = read_to_string(path.as_ref())?;
        self.parse_unit_from_string(&source, Some(path))
    }

    fn import<P: AsRef<Path>>(&mut self, relative_to: Option<P>, import: &ImportPath) -> Result<(), ParseError> {
        let mut imported = false;
        let import_path = import.as_path();

        // FIXME: avoid cloning
        for path in self.config.import_paths.clone() {
            if path.is_relative() {
                if let Some(relative_to) = &relative_to {
                    if self.try_import(relative_to.as_ref().join(path).join(&import_path))? {
                        imported = true;
                        break;
                    }
                }
            }
            else if self.try_import(path.join(&import_path))? {
                imported = true;
                break;
            }
        }

        if !imported {
            Err(ParseError::ImportNotFound {
                relative_to: relative_to.map(|p| p.as_ref().to_owned()),
                import: import.clone(),
            })
        }
        else {
            Ok(())
        }
    }

    fn try_import<P: AsRef<Path>>(&mut self, path: P) -> Result<bool, ParseError> {
        if path.as_ref().exists() {
            self.parse_unit_from_file(path)?;
            Ok(true)
        }
        else {
            Ok(false)
        }
    }
}


#[derive(Debug, Error)]
#[error("Invalid signal: {0}")]
pub struct ParseSignalError(String);

pub fn parse_signal(s: &str) -> Result<ast::Signal, ParseSignalError> {
    let e = || ParseSignalError(s.to_owned());

    let signal = parser::SignalParser::new().parse(s).map_err(|_| e())?;

    if let ast::Signal::Var(_) = &signal {
        Err(e())
    }
    else {
        Ok(signal)
    }
}


#[cfg(test)]
mod tests {
    use super::parser;

    #[test]
    fn it_parses_a_module_decl() {
        let s = r#"
mod clock<#n> {
    port clock_enable(red ce, _);
    port time(red t, green _);

    // This is a line comment
    wire green tmp;

    /* This is a multi-
       line comment */
    tmp[virtual-C] <- if ce[virtual-E] > 0 then 1;

    tmp[virtual-C] <- if tmp[virtual-C] < #n then _;

    diode_rename<virtual-T, virtual-C>(input: tmp, output: t);
}
        "#;

        match parser::ModDeclParser::new().parse(s) {
            Ok(mod_decl) => {
                println!("{:#?}", mod_decl);
            },
            Err(e) => {
                let rest = e.map_location(|l| &s[l .. l + 16]);
                eprintln!("Error at: {}", rest);
            }
        }
    }

    /*
    #[test]
    fn it_parses_a_clock_with_diode() {
        let s = r#"
mod diode_rename<$out, $in> {
    port input(in_red, in_green);
    port output(in_red, in_green);

    (out_red, out_green)[$out] <- (in_red, in_green)[$in] + 0;
}

mod clock<#n> {
    port clock_enable(red ce, _);
    port time(red t, green _);

    wire green tmp;

    tmp[virtual-C] <- if ce[virtual-E] > 0 then 1;

    tmp[virtual-C] <- if tmp[virtual-C] < #n then _;

    diode_rename<virtual-T, virtual-C>(input: tmp, output: t);
}
        "#;

        let unit = parse_unit(s).unwrap();
        println!("{:#?}", unit);
    }*/

    #[test]
    fn it_parses_idents() {
        parser::IdentParser::new().parse("foobar").unwrap();
        parser::IdentParser::new().parse("_foo").unwrap();
        parser::IdentParser::new().parse("f").unwrap();
        parser::IdentParser::new().parse("_").unwrap();
    }

}