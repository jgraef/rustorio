use std::{
    path::PathBuf,
    fmt::{self, Display, Formatter},
};

use derive_more::{From, Into, AsRef, AsMut, Display};

use crate::ir;


#[derive(Clone, Debug)]
pub struct Unit {
    pub imports: Vec<ImportPath>,
    pub modules: Vec<ModDecl>,

    pub file_path: Option<PathBuf>,
    pub source: Option<String>,
}

impl Unit {
    pub fn new(imports: Vec<ImportPath>, modules: Vec<ModDecl>) -> Self {
        Self {
            imports,
            modules,
            file_path: None,
            source: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ImportPath(pub Vec<Ident>);

impl ImportPath {
    pub fn as_path(&self) -> PathBuf {
        let mut it = self.0.iter().peekable();
        let mut path = PathBuf::new();

        while let Some(ident) = it.next() {
            let path_segment = &ident.0;

            if it.peek().is_none() {
                path.push(format!("{}.fc", path_segment));
            }
            else {
                path.push(&ident.0);
            }
        }

        path
    }
}

impl Display for ImportPath {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut it = self.0.iter();

        // NOTE: Paths can't be empty
        write!(f, "{}", it.next().unwrap())?;

        while let Some(ident) = it.next() {
            write!(f, ".{}", ident)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct ModDecl {
    pub ident: Ident,
    pub generics: Option<Vec<GenericDecl>>,
    pub body: ModBody,
}

#[derive(Clone, Debug)]
pub struct ModBody {
    pub ports: Vec<PortDecl>,
    pub wires: Vec<WireDecl>,
    pub statements: Vec<Statement>,
}

#[derive(Clone, Debug)]
pub enum GenericDecl {
    Number(Ident),
    Signal(Ident),
}

#[derive(Clone, Debug)]
pub enum PortDecl {
    Both {
        ident: Ident,
        red: Ident,
        green: Ident,
    },
    ShortHand {
        ident: Ident,
        color: ir::WireColor,
    },
}

#[derive(Clone, Debug)]
pub struct WireDecl {
    pub color: ir::WireColor,
    pub ident: Ident,
}

#[derive(Clone, Debug)]
pub enum Statement {
    Constant {
        output: Output, // Must be wires only
        constants: Vec<SignalConst>,
    },
    Arithmetic {
        output: Output,
        op: ir::ArithmeticOp,
        left: Input,
        right: Input,
    },
    Decider {
        output: Output,
        op: ir::DeciderOp,
        left: Input,
        right: Input,
        mode: DeciderMode,
    },
    ModuleInst {
        ident: Ident,
        generics: Option<Vec<GenericArg>>,
        ports: Vec<PortDef>,
    }
}

#[derive(Clone, Debug)]
pub struct Output {
    pub wires: Wires,
    pub signal: Option<Signal>,
}

#[derive(Clone, Debug)]
pub enum Input {
    Signal {
        wires: Wires,
        signal: Option<Signal>,
    },
    Number(Expr),
}

#[derive(Clone, Debug)]
pub enum Wires {
    Single(Ident),
    Both {
        red: Ident,
        green: Ident,
    }
}

#[derive(Clone, Debug)]
pub enum Signal {
    Virtual(Ident),
    Item(Ident),
    Fluid(Ident),
    Each,
    All,
    Any,
    Var(Ident),
}

impl Signal {
    pub fn shorthand(s: &str) -> Self {
        Signal::Virtual(Ident::from(format!("signal-{}", s.to_uppercase())))
    }
}

#[derive(Clone, Debug)]
pub enum DeciderMode {
    One,
    Input(Ident),
}

#[derive(Clone, Debug)]
pub enum GenericArg {
    Signal(Signal),
    Number(Expr),
}

#[derive(Clone, Debug)]
pub struct PortDef {
    pub port: Ident,
    pub wire: Ident,
}

#[derive(Clone, Debug)]
pub struct SignalConst {
    pub signal: Signal,
    pub value: Expr,
}

#[derive(Copy, Clone, Debug)]
pub enum LiteralType {
    Number,
    Signal,
}

/*
#[derive(Clone, Debug)]
pub enum Literal {
    Virtual(Ident),
    Item(Ident),
    Fluid(Ident),
    Each,
    All,
    Any,
    Number(i32),
}

impl Literal {
    pub fn r#type(&self) -> LiteralType {
        match self {
            Literal::Number(_) => LiteralType::Number,
            _ => LiteralType::Signal,
        }
    }
}*/

#[derive(Clone, Debug)]
pub enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    BitAnd(Box<Expr>, Box<Expr>),
    BitOr(Box<Expr>, Box<Expr>),
    BitXor(Box<Expr>, Box<Expr>),
    BitNot(Box<Expr>),
    Neg(Box<Expr>),
    Const(i32),
    Var(Ident),
}


#[derive(Clone, Debug, From, Into, AsRef, AsMut, Display, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ident(String);

impl From<&str> for Ident {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl Ident {
    pub fn is_placeholder(&self) -> bool {
        self.0 == "_"
    }
}