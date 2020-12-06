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
        let mut p = PathBuf::new();

        for ident in &self.0 {
            p.push(&ident.0);
        }

        p
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
    Num(Ident),
    SignalID(Ident),
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
        disabled: bool,
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
    Constant(i32),
    GenericArg(Ident),
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
    All,
    Any,
    ForEach,
    Ident(Ident),
    GenericArg(Ident),
}

#[derive(Clone, Debug)]
pub enum DeciderMode {
    One,
    Input(Ident),
}

#[derive(Clone, Debug)]
pub enum GenericArg {
    NumVar(Ident),
    NumConst(i32),
    SignalVar(Ident),
    SignalConst(Ident),
}

#[derive(Clone, Debug)]
pub struct PortDef {
    pub port: Ident,
    pub wire: Ident,
}

#[derive(Clone, Debug)]
pub struct SignalConst {
    pub ident: Ident,
    pub constant: i32,
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