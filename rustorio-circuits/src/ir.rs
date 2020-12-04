use std::num::NonZeroUsize;

use rustorio_core::blueprint::types::{Signal, SignalID};


#[derive(Clone, Debug)]
pub enum Input {
    Everything, // only for decider
    Anything, // only for decider
    ForEach,
    Constant(i32),
    Signal(SignalID),
}

#[derive(Clone, Debug)]
pub enum OutputSignal {
    ForEach, // only if input was foreach
    Everything, // only for decider
    Signal(SignalID)
}

#[derive(Copy, Clone, Debug)]
pub enum OutputCount {
    One,
    InputSignal,
}

#[derive(Copy, Clone, Debug)]
pub enum ArithmeticOp {
    Multiply,
    Divide,
    Add,
    Subtract,
    Modulo,
    Power,
    LeftShift,
    RightShift,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
}

#[derive(Copy, Clone, Debug)]
pub enum DeciderOp {
    GreaterThan,
    LessThan,
    Equal,
    GreaterEqual,
    LessEqual,
    NotEqual,
}

#[derive(Clone, Debug)]
pub struct ArithmeticCombinator {
    pub op: ArithmeticOp,
    pub left: Input,
    pub right: Input,
    pub output: OutputSignal,

    pub input_wires: Wires,
    pub output_wires: Wires,
}

#[derive(Clone, Debug)]
pub struct DeciderCombinator {
    pub op: DeciderOp,
    pub left: Input,
    pub right: Input,
    pub output_signal: OutputSignal,
    pub output_count: OutputCount,

    pub input_wires: Wires,
    pub output_wires: Wires,
}

#[derive(Clone, Debug)]
pub struct ConstantCombinator {
    pub signals: Vec<Signal>,
    pub enabled: bool,
    pub wires: Wires,
}

pub type WireId = NonZeroUsize;

#[derive(Copy, Clone, Debug)]
pub enum WireColor {
    Red,
    Green,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Wires {
    // ID of red wire
    pub red: Option<WireId>,
    // ID of green wire
    pub green: Option<WireId>,
}

impl Wires {
    pub fn get_color(&self, color: WireColor) -> Option<WireId> {
        match color {
            WireColor::Red => self.red,
            WireColor::Green => self.green,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Combinator {
    Arithmetic(ArithmeticCombinator),
    Decider(DeciderCombinator),
    Constant(ConstantCombinator),

    // TODO: Pushbutton? Nixie-tubes?
}
