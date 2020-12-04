use std::{
    sync::Arc,
    collections::HashMap,
};

use rustorio_core::blueprint::types::SignalID;

use crate::ir;


pub struct Wire {
    values: HashMap<SignalID, i32>,

    /// Combinators that put their output to this wire
    inputs: Vec<Arc<ir::Combinator>>,

    /// Combinators that take their input from this wire
    outputs: Vec<Arc<ir::Combinator>>,

    /// Whether a signal changed in this wire.
    dirty: bool,
}

pub struct Simulator {
    combinators: Vec<Arc<ir::Combinator>>,
    wires: HashMap<ir::WireId, Wire>,
}