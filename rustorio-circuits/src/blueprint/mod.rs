use std::{
    ops::AddAssign,
    collections::HashMap,
};

use thiserror::Error;
use num_traits::One;

use rustorio_core::{
    types::UnitNumber,
    blueprint::{
        types::Position,
        Blueprint, Entity, ControlBehavior, ArithmeticConditions, DeciderConditions,
    },
};

use crate::ir;
use itertools::Itertools;
use rustorio_core::blueprint::ConnectionData;


// TODO: Move this to utils, and reuse it for WireIdGen
#[derive(Debug, Default)]
pub struct Sequential<T> {
    next: T,
}

impl<T> Sequential<T> {
    pub fn new(first: T) -> Self {
        Self { next: first }
    }
}

impl<T: Copy + AddAssign + One> Sequential<T> {
    fn next(&mut self) -> T {
        let x = self.next;
        self.next += One::one();
        x
    }
}


#[derive(Debug, Error)]
pub enum Error {

}

fn add_wires(connections: &mut HashMap<ir::WireId, Vec<(usize, u8, ir::WireColor)>>, wire_id: Option<ir::WireId>, entity_index: usize, which: u8, color: ir::WireColor) {
    if let Some(wire_id) = wire_id {
        connections.entry(wire_id)
            .or_default()
            .push((entity_index, which, color));
    }
}

fn connect_entities(entity: &mut Entity, w1: u8, to: UnitNumber, w2: u8, color: ir::WireColor) {
    let connection = entity.connections.get_or_insert_with(Default::default);

    let point = match w1 {
        1 => &mut connection.first,
        2 => connection.second.get_or_insert_with(Default::default),
        _ => unreachable!(),
    };

    let data = match color {
        ir::WireColor::Red => &mut point.red,
        ir::WireColor::Green => &mut point.green,
    };

    data.push(ConnectionData {
        entity_id: to,
        circuit_id: Some(w2),
    })
}

pub fn blueprint_from_ir(ir: &ir::Ir) -> Result<Blueprint, Error> {
    let mut entity_numbers = Sequential::new(1);
    let mut blueprint = Blueprint::default();
    let mut connections: HashMap<ir::WireId, Vec<(usize, u8, ir::WireColor)>> = HashMap::new();

    for (i, combinator) in ir.combinators.iter().enumerate() {
        let (x, y) = morton::deinterleave_morton(i as u32);
        let position = Position::new(x as f32 + 0.5, y as f32 * 2.0 + 0.5);

        let (e, w1, w2) = entity_from_ir(combinator, position, &mut entity_numbers)?;

        add_wires(&mut connections, w1.red, i, 1, ir::WireColor::Red);
        add_wires(&mut connections, w1.green, i, 1, ir::WireColor::Green);
        add_wires(&mut connections, w2.red, i, 2, ir::WireColor::Red);
        add_wires(&mut connections, w2.green, i, 2, ir::WireColor::Green);

        blueprint.entities.push(e);
    }

    for conns in connections.values() {
        for ((i, w_i, c), (j, w_j, c2)) in conns.iter().tuple_windows() {
            assert_eq!(c, c2);
            let en_i = blueprint.entities.get(*i).unwrap().entity_number;
            let en_j = blueprint.entities.get(*j).unwrap().entity_number;
            connect_entities(blueprint.entities.get_mut(*i).unwrap(), *w_i, en_j, *w_j, *c);
            // TODO: Is it enough to just encode the connection in one direction?
            connect_entities(blueprint.entities.get_mut(*j).unwrap(), *w_j, en_i, *w_i, *c);
        }
    }

    Ok(blueprint)
}


pub fn entity_from_ir(combinator: &ir::Combinator, position: Position, entity_numbers: &mut Sequential<UnitNumber>) -> Result<(Entity, ir::Wires, ir::Wires), Error> {
    let mut control = ControlBehavior::default();

    let (entity_name, w1, w2) = match combinator {
        ir::Combinator::Constant(ir::ConstantCombinator { signals, wires }) => {
            for signal in signals {
                control.add_filter(signal.clone());
            }

            ("constant-combinator", *wires, ir::Wires::default())
        },
        ir::Combinator::Arithmetic(ir::ArithmeticCombinator { op, left, right, output, input_wires, output_wires }) => {
            control.arithmetic_conditions = Some(ArithmeticConditions {
                first_signal: left.as_signal_id(),
                second_signal: right.as_signal_id(),
                first_constant: left.as_constant(),
                second_constant: right.as_constant(),
                operation: op.as_op_str().to_owned(),
                output_signal: Some(output.as_signal_id())
            });

            ("arithmetic-combinator", *input_wires, *output_wires)
        },
        ir::Combinator::Decider(ir::DeciderCombinator { op, left, right, output_signal, output_count, input_wires, output_wires }) => {
            control.decider_conditions = Some(DeciderConditions {
                first_signal: Some(left.as_signal_id().expect("LHS must be signal")),
                second_signal: right.as_signal_id(),
                constant: right.as_constant(),
                comparator: op.as_op_str().to_owned(),
                output_signal: Some(output_signal.as_signal_id()),
                copy_count_from_input: output_count.is_input_signal(),
            });

            ("decider-combinator", *input_wires, *output_wires)
        },
    };

    let mut entity = Entity::new(
        entity_numbers.next(),
        entity_name.to_owned(),
        position,
    );

    entity.direction = Some(0);
    entity.control_behavior = Some(control);

    Ok((entity, w1, w2))
}