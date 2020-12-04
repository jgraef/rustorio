use derive_more::{Into, From, AsRef, AsMut};

use rustorio_core::blueprint::types::Signal;

use crate::ir;



pub trait Builder {
    type Error;
    type Output;

    fn add_combinator(&mut self, combinator: ir::Combinator) -> Result<(), Self::Error>;

    fn finish(self) -> Self::Output;
}


pub trait Ports {
    type PortId;

    fn get_wires(&self, port_id: &Self::PortId) -> ir::Wires;

    fn get_wire_id(&self, port_id: &Self::PortId, color: ir::WireColor) -> Option<ir::WireId> {
        self.get_wires(port_id).get_color(color)
    }
}


pub trait Component {
    type Ports: Ports;

    fn construct<B: Builder>(&self, builder: &mut B, ports: &Self::Ports) -> Result<(), B::Error>;
}


/// A single port which might have a red and/or green cable connected.
#[derive(Clone, Debug, From, Into, AsRef, AsMut)]
pub struct Port(ir::Wires);

impl Ports for Port {
    type PortId = ();

    fn get_wires(&self, _: &Self::PortId) -> ir::Wires {
        self.0
    }
}


pub enum TuplePortId<A, B> {
    Left(A),
    Right(B),
}

impl<A: Ports, B: Ports> Ports for (A, B) {
    type PortId = TuplePortId<A::PortId, B::PortId>;

    fn get_wires(&self, port_id: &Self::PortId) -> ir::Wires {
        match port_id {
            TuplePortId::Left(port_id) => self.0.get_wires(port_id),
            TuplePortId::Right(port_id) => self.1.get_wires(port_id),
        }
    }
}

impl<T: Ports> Ports for [T] {
    type PortId = (usize, T::PortId);

    fn get_wires(&self, port_id: &Self::PortId) -> ir::Wires {
        // Should this rather panic when the port_id is out-of-bounds?
        self.get(port_id.0).map(|wires| wires.get_wires(&port_id.1)).unwrap_or_default()
    }
}


#[derive(Clone, Debug)]
pub struct Constant {
    pub signals: Vec<Signal>,
    pub enabled: bool,
}

impl Component for Constant {
    type Ports = Port;

    fn construct<B: Builder>(&self, builder: &mut B, ports: &Self::Ports) -> Result<(), <B as Builder>::Error> {
        builder.add_combinator(ir::Combinator::Constant(ir::ConstantCombinator {
            signals: self.signals.clone(),
            enabled: self.enabled,
            wires: ports.get_wires(&())
        }))?;

        Ok(())
    }
}


pub enum InputOutputPort {
    Input,
    Output,
}

pub struct InputOutput {
    input: ir::Wires,
    output: ir::Wires,
}

impl Ports for InputOutput {
    type PortId = InputOutputPort;

    fn get_wires(&self, port_id: &Self::PortId) -> ir::Wires {
        match port_id {
            InputOutputPort::Input => self.input,
            InputOutputPort::Output => self.output,
        }
    }
}


#[derive(Clone, Debug)]
pub struct Arithmetic {
    pub op: ir::ArithmeticOp,
    pub left: ir::Input,
    pub right: ir::Input,
    pub output: ir::OutputSignal,
}

impl Component for Arithmetic {
    type Ports = InputOutput;

    fn construct<B: Builder>(&self, builder: &mut B, ports: &Self::Ports) -> Result<(), <B as Builder>::Error> {
        builder.add_combinator(ir::Combinator::Arithmetic(ir::ArithmeticCombinator {
            op: self.op,
            left: self.left.clone(),
            right: self.right.clone(),
            output: self.output.clone(),
            input_wires: ports.get_wires(&InputOutputPort::Input),
            output_wires: ports.get_wires(&InputOutputPort::Output),
        }))
    }
}


#[derive(Clone, Debug)]
pub struct Decider {
    pub op: ir::DeciderOp,
    pub left: ir::Input,
    pub right: ir::Input,
    pub output_signal: ir::OutputSignal,
    pub output_count: ir::OutputCount,
}

impl Component for Decider {
    type Ports = InputOutput;

    fn construct<B: Builder>(&self, builder: &mut B, ports: &Self::Ports) -> Result<(), <B as Builder>::Error> {
        builder.add_combinator(ir::Combinator::Decider(ir::DeciderCombinator {
            op: self.op,
            left: self.left.clone(),
            right: self.right.clone(),
            output_signal: self.output_signal.clone(),
            output_count: self.output_count,
            input_wires: ports.get_wires(&InputOutputPort::Input),
            output_wires: ports.get_wires(&InputOutputPort::Output),
        }))
    }
}


