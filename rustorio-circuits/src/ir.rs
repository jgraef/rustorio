use std::{
    collections::hash_map::{HashMap, Entry},
    sync::atomic::{AtomicU64, Ordering},
};

use serde::{Serialize, Deserialize};

pub use rustorio_core::blueprint::types::{Signal, SignalID};



#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Display)]
pub struct WireId(u64);


#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum WireColor {
    Red,
    Green,
}


#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
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

impl RenameWires for Wires {
    fn rename(&mut self, renamer: &mut WireRenamer) {
        if let Some(red) = &mut self.red {
            *red = renamer.rename(*red);
        }
        if let Some(green) = &mut self.green {
            *green = renamer.rename(*green);
        }
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Input {
    Everything, // only for decider
    Anything, // only for decider
    ForEach,
    Constant(i32),
    Signal(SignalID),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OutputSignal {
    ForEach, // only if input was foreach
    Everything, // only for decider
    Signal(SignalID)
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum OutputCount {
    One,
    InputSignal,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
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

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum DeciderOp {
    GreaterThan,
    LessThan,
    Equal,
    GreaterEqual,
    LessEqual,
    NotEqual,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArithmeticCombinator {
    pub op: ArithmeticOp,
    pub left: Input,
    pub right: Input,
    pub output: OutputSignal,

    pub input_wires: Wires,
    pub output_wires: Wires,
}

impl RenameWires for ArithmeticCombinator {
    fn rename(&mut self, renamer: &mut WireRenamer) {
        self.input_wires.rename(renamer);
        self.output_wires.rename(renamer);
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeciderCombinator {
    pub op: DeciderOp,
    pub left: Input,
    pub right: Input,
    pub output_signal: OutputSignal,
    pub output_count: OutputCount,

    pub input_wires: Wires,
    pub output_wires: Wires,
}

impl RenameWires for DeciderCombinator {
    fn rename(&mut self, renamer: &mut WireRenamer) {
        self.input_wires.rename(renamer);
        self.output_wires.rename(renamer);
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConstantCombinator {
    pub signals: Vec<Signal>,
    pub enabled: bool,
    pub wires: Wires,
}

impl RenameWires for ConstantCombinator {
    fn rename(&mut self, renamer: &mut WireRenamer) {
        self.wires.rename(renamer);
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Combinator {
    Arithmetic(ArithmeticCombinator),
    Decider(DeciderCombinator),
    Constant(ConstantCombinator),

    // TODO: Pushbutton? Nixie-tubes?
}

impl RenameWires for Combinator {
    fn rename(&mut self, renamer: &mut WireRenamer) {
        match self {
            Combinator::Arithmetic(c) => c.rename(renamer),
            Combinator::Decider(c) => c.rename(renamer),
            Combinator::Constant(c) => c.rename(renamer),
        }
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ir {
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(default, skip_serializing_if="HashMap::is_empty")]
    pub ports: HashMap<(String, WireColor), WireId>,

    pub combinators: Vec<Combinator>,
}

impl RenameWires for Ir {
    fn rename(&mut self, renamer: &mut WireRenamer) {
        for wire_id in self.ports.values_mut() {
            *wire_id = renamer.rename(*wire_id);
        }

        for combinator in &mut self.combinators {
            combinator.rename(renamer);
        }
    }
}


#[derive(Debug)]
pub struct WireIdGen {
    next_id: AtomicU64,
}

impl Default for WireIdGen {
    fn default() -> Self {
        Self { next_id: AtomicU64::new(1) }
    }
}

impl WireIdGen {
    pub fn next(&self) -> WireId {
        WireId(self.next_id.fetch_add(1, Ordering::SeqCst))
    }
}

#[derive(Debug, Default)]
pub struct WireRenamer {
    gen: WireIdGen,
    renamed: HashMap<WireId, WireId>,
}

impl WireRenamer {
    pub fn new(gen: WireIdGen) -> Self {
        Self {
            gen,
            renamed: HashMap::new(),
        }
    }

    pub fn into_gen(self) -> WireIdGen {
        self.gen
    }

    pub fn rename(&mut self, old: WireId) -> WireId {
        match self.renamed.entry(old) {
            Entry::Occupied(occupied) => *occupied.get(),
            Entry::Vacant(vacant) => {
                let id = self.gen.next();
                vacant.insert(id);
                id
            }
        }
    }
}

pub trait RenameWires {
    fn rename(&mut self, renamer: &mut WireRenamer);
}
