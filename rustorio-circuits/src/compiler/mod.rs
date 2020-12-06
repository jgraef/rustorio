use thiserror::Error;

use crate::{
    parser::ast,
    ir,
};
use std::collections::HashMap;
use rustorio_core::blueprint::types::SignalIDParseError;


#[derive(Debug, Error)]
pub enum CompilerError {
    #[error("TODO")]
    Todo,

    #[error("Unknown wire: {0}")]
    UnknownWire(ast::Ident),

    #[error("Wire {0} has differnet color thant {1:?}")]
    WireColorMismatch(ast::Ident, ir::WireColor),

    #[error("Unknown module: {0}")]
    UnknownModule(ast::Ident),

    #[error("{0}")]
    InvalidSignalLiteral(#[from] SignalIDParseError),

    #[error("Can't merge wires {0:?} and {1:?}")]
    WireConflict(ir::Wires, ir::Wires),
}


pub struct Compiler<'a> {
    wire_ids: ir::WireIdGen,

    //units: &'a [ast::Unit],

    modules: HashMap<&'a ast::Ident, &'a ast::ModDecl>,
}

impl<'a> Compiler<'a> {
    pub fn new(units: &'a [ast::Unit]) -> Self {
        let mut modules = HashMap::new();

        for unit in units {
            for module in &unit.modules {
                modules.insert(&module.ident, module);
            }
        }

        Self {
            wire_ids: ir::WireIdGen::default(),
            //units,
            modules,
        }
    }

    fn map_wires(wires: &HashMap<&ast::Ident, (ir::WireId, ir::WireColor)>, ast_wires: &ast::Wires) -> Result<ir::Wires, CompilerError> {
        match ast_wires {
            ast::Wires::Single(ident) => {
                let (wire_id, color) = wires.get(ident)
                    .ok_or_else(|| CompilerError::UnknownWire(ident.clone()))?;

                if *color == ir::WireColor::Red {
                    Ok(ir::Wires { red: Some(*wire_id), green: None })
                }
                else {
                    Ok(ir::Wires { red: None, green: Some(*wire_id) })
                }
            },
            ast::Wires::Both { red, green } => {
                let (red_id, color) = wires.get(red)
                    .ok_or_else(|| CompilerError::UnknownWire(red.clone()))?;
                if *color != ir::WireColor::Red {
                    return Err(CompilerError::WireColorMismatch(red.clone(), *color));
                }

                let (green_id, color) = wires.get(green)
                    .ok_or_else(|| CompilerError::UnknownWire(green.clone()))?;
                if *color != ir::WireColor::Green {
                    return Err(CompilerError::WireColorMismatch(green.clone(), *color));
                }

                Ok(ir::Wires { red: Some(*red_id), green: Some(*green_id) })
            },
        }
    }

    fn merge_wires(a: ir::Wires, b: ir::Wires) -> Result<ir::Wires, CompilerError> {
        let red = match (&a.red, &b.red) {
            (Some(a), None) => Some(*a),
            (None, Some(b)) => Some(*b),
            (None, None) => None,
            (Some(_), Some(_)) => return Err(CompilerError::WireConflict(a, b)),
        };

        let green = match (&a.green, &b.green) {
            (Some(a), None) => Some(*a),
            (None, Some(b)) => Some(*b),
            (None, None) => None,
            (Some(_), Some(_)) => return Err(CompilerError::WireConflict(a, b)),
        };

        Ok(ir::Wires { red, green })
    }

    fn lookup_signal_id(&self, ident: &ast::Ident) -> Result<ir::SignalID, CompilerError> {
        Ok(ident.as_ref().parse()?)
    }

    fn map_input(&self, input: &ast::Input, _generics: &HashMap<&ast::Ident, &ast::GenericArg>, wire_map: &HashMap<&ast::Ident, (ir::WireId, ir::WireColor)>) -> Result<(ir::Input, ir::Wires), CompilerError> {
        Ok(match input {
            ast::Input::Signal { wires, signal } => {
                (
                    match signal.as_ref().ok_or_else(|| CompilerError::Todo)? {
                        ast::Signal::All => ir::Input::Everything,
                        ast::Signal::Any => ir::Input::Anything,
                        ast::Signal::ForEach => ir::Input::ForEach,
                        ast::Signal::Ident(ident) => ir::Input::Signal(self.lookup_signal_id(ident)?),
                        ast::Signal::GenericArg(_ident) => {
                            // signal generic
                            todo!()
                        },
                    },
                    Self::map_wires(wire_map, wires)?,
                )
            },
            ast::Input::Constant(constant) => {
                (ir::Input::Constant(*constant), ir::Wires::default())
            }
            ast::Input::GenericArg(_ident) => {
                // numeric generic
                todo!();
            }
        })
    }

    fn map_output(&self, output: &ast::Output, _generics: &HashMap<&ast::Ident, &ast::GenericArg>, wire_map: &HashMap<&ast::Ident, (ir::WireId, ir::WireColor)>) -> Result<(ir::OutputSignal, ir::Wires), CompilerError> {
        Ok((
            match output.signal.as_ref().ok_or_else(|| CompilerError::Todo)? {
                ast::Signal::All => ir::OutputSignal::Everything,
                ast::Signal::Any => return Err(CompilerError::Todo),
                ast::Signal::ForEach => ir::OutputSignal::ForEach,
                ast::Signal::Ident(ident) => ir::OutputSignal::Signal(self.lookup_signal_id(ident)?),
                ast::Signal::GenericArg(_) => todo!(),
            },
            Self::map_wires(wire_map, &output.wires)?,
        ))
    }

    pub fn compile_module(&self, ident: &ast::Ident, _generic_args: Vec<&ast::GenericArg>) -> Result<ir::Ir, CompilerError> {
        let module = *self.modules.get(ident)
            .ok_or_else(|| CompilerError::UnknownModule(ident.to_owned()))?;

        let generics: HashMap<&ast::Ident, &ast::GenericArg> = HashMap::new();
        let mut wires: HashMap<&ast::Ident, (ir::WireId, ir::WireColor)> = HashMap::new();
        let mut ports: HashMap<(&ast::Ident, ir::WireColor), ir::WireId> = HashMap::new();
        let mut combinators = vec![];

        /*
        let empty_generics = vec![];
        let generic_decls = module.generics.as_ref().unwrap_or(&empty_generics);

        if generic_args.len() != generic_decls.len() {
            return Err(CompilerError::Todo);
        }

        for (generic_decl, generic_arg) in generic_decls.into_iter().zip(generic_args) {
            match generic_decl {
                ast::GenericDecl::SignalID(ident) => {
                    generics.insert(ident, generic_arg);
                },
                ast::GenericDecl::Num(ident) => {
                    todo!()
                }
            }
        }
        */

        for port_decl in &module.body.ports {
            match port_decl {
                ast::PortDecl::Both { ident, red, green } => {
                    if !red.is_placeholder() {
                        let red_id = self.wire_ids.next();
                        ports.insert((ident, ir::WireColor::Red), red_id);
                        wires.insert(red, (red_id, ir::WireColor::Red));
                    }

                    if !green.is_placeholder() {
                        let green_id = self.wire_ids.next();
                        ports.insert((ident, ir::WireColor::Green), green_id);
                        wires.insert(green, (green_id, ir::WireColor::Green));
                    }
                },
                ast::PortDecl::ShortHand { ident, color } => {
                    let wire_id = self.wire_ids.next();
                    ports.insert((ident, *color), wire_id);
                    wires.insert(ident, (wire_id, *color));
                },
            }
        }

        for ast::WireDecl { color, ident } in &module.body.wires {
            wires.insert(&ident, (self.wire_ids.next(), *color));
        }

        log::debug!("Declared ports: {:#?}", ports);
        log::debug!("Declared wires: {:#?}", wires);

        for statement in &module.body.statements {
            match statement {
                ast::Statement::Constant { output, constants } => {
                    if output.signal.is_some() {
                        return Err(CompilerError::Todo);
                    }

                    let mut signals = vec![];
                    for sig in constants {
                        let signal_id = self.lookup_signal_id(&sig.ident)?;
                        signals.push(signal_id.into_signal(sig.constant));
                    }

                    combinators.push(ir::Combinator::Constant(ir::ConstantCombinator {
                        signals,
                        wires: Self::map_wires(&wires, &output.wires)?,
                    }));
                },

                ast::Statement::Arithmetic { output, op, left, right } => {
                    let (left_input, left_wires) = self.map_input(left, &generics, &wires)?;
                    let (right_input, right_wires) = self.map_input(right, &generics, &wires)?;
                    let input_wires = Self::merge_wires(left_wires, right_wires)?;
                    let (output_signal, output_wires) = self.map_output(output, &generics, &wires)?;

                    combinators.push(ir::Combinator::Arithmetic(ir::ArithmeticCombinator {
                        op: *op,
                        left: left_input,
                        right: right_input,
                        output: output_signal,
                        input_wires,
                        output_wires,
                    }));
                },

                ast::Statement::Decider { output, op, left, right, mode } => {
                    let (left_input, left_wires) = self.map_input(left, &generics, &wires)?;
                    let (right_input, right_wires) = self.map_input(right, &generics, &wires)?;
                    let input_wires = Self::merge_wires(left_wires, right_wires)?;
                    let (output_signal, output_wires) = self.map_output(output, &generics, &wires)?;

                    let output_count = match mode {
                        ast::DeciderMode::One => ir::OutputCount::One,
                        ast::DeciderMode::Input(ident) => {
                            if !ident.is_placeholder() {
                                // TODO: Handle the case that we specify the output signal here and not in the output expression.
                                todo!();
                            }
                            else {
                                ir::OutputCount::InputSignal
                            }
                        },
                    };

                    combinators.push(ir::Combinator::Decider(ir::DeciderCombinator {
                        op: *op,
                        left: left_input,
                        right: right_input,
                        output_signal,
                        output_count,
                        input_wires,
                        output_wires,
                    }));
                },

                ast::Statement::ModuleInst { .. } => {
                    todo!();
                },
            }
        }

        Ok(ir::Ir {
            name: Some(ident.as_ref().to_owned()),
            ports: Default::default(),
            combinators,
        })
    }
}
