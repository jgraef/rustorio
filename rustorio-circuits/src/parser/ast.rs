use rustorio_core::blueprint::types::SignalID;

use crate::ir;


pub enum Wire {
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
}


pub enum Statement {
    Constant {
        wire: Wire,
        constants: Vec<SignalConst>,
    },
    Arithmetic {
        output: WireSignal,
        op: ArithmeticOp,
        left: WireSignal,
        right: WireSignal,
    },
    Decider {
        output: WireSignal,
        op: DeciderOp,
        left: WireSignal,
        right: WireSignal,
    },
}


pub struct ModuleDecl {
    params: Vec<ParameterDecl>,
    ports: Vec<PortDecl>,
    wires: Vec<WireDecl>,
    statements: Vec<Statement>,
}

