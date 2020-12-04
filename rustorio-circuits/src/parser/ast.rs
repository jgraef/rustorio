use derive_more::{From, Into, AsRef, AsMut, Display};

use crate::ir;



/*pub enum Wire {
    Red,
    Green,
    Anonymous,
}

pub enum Signal {
    Everything,
    Anything,
    ForEach
    Constant(i32),
    Signal(SignalID),
    Anonymous,
}

pub struct WireSignal {
    wire: Wire,
    signal: Signal,
}

pub struct SignalConst {
    signal: SignalID,
    constant: i32,
}*/

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
        color: WireColor,
        //wire: Ident,
    },
}

#[derive(Clone, Debug)]
pub struct WireDecl {
    pub color: WireColor,
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
        output: Output,
        ident: Ident,
        generics: Option<Vec<GenericArg>>,
        ports: Vec<PortDef>,
    }
}

#[derive(Clone, Debug)]
pub struct SignalConst {
    pub signal: Ident,
    pub constant: i32,
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
}

#[derive(Clone, Debug)]
pub enum DeciderMode {
    One,
    Input(Ident),
}

#[derive(Clone, Debug)]
pub enum WireColor {
    Red,
    Green
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

#[derive(Clone, Debug, From, Into, AsRef, AsMut, Display, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ident(String);

impl From<&str> for Ident {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}


