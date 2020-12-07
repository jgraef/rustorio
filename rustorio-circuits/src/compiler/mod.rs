use std::{
    str::FromStr,
    collections::HashMap,
};

use thiserror::Error;

use crate::{
    parser::ast,
    ir,
};
use rustorio_core::blueprint::types::{SignalType, SignalIDParseError};


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

    #[error("Signal not allowed in this context: {0:?}")]
    SignalNotAllowed(ast::Signal),

    #[error("Parameter is not a number: {0}")]
    ParamNotANumber(ast::Ident),

    #[error("Parameter is not a signal: {0}")]
    ParamNotASignal(ast::Ident),

    #[error("Parameter not found: {0}")]
    ParamNotFound(ast::Ident),

    #[error("Incorrect number of parameters: Expected {expected}, but got {got}")]
    ParamCountMismatch {
        expected: usize,
        got: usize
    },

    #[error("Type of parameter '{param:?}' doesn't match type of declaration '{decl:?}'")]
    ParamTypeMismatch {
        decl: ast::GenericDecl,
        param: Param,
    },

    #[error("Overflow while evaluating const expression: {0:?}")]
    Overflow(ast::Expr),

    #[error("Conflict while connecting port: {port} (color {color:?})")]
    PortConflict {
        port: ast::Ident,
        color: ir::WireColor,
    },
}


#[derive(Debug, Error)]
#[error("Failed parsing module parameter: {0}")]
pub struct ParamParseError(String);

#[derive(Clone, Debug)]
pub enum Param {
    Number(i32),
    Signal(ir::SignalID),
}

impl FromStr for Param {
    type Err = ParamParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = || ParamParseError(s.to_owned());

        if let Ok(n) = s.parse::<i32>() {
            Ok(Param::Number(n))
        }
        else {
            let signal_id = s.parse().map_err(|_| err())?;
            Ok(Param::Signal(signal_id))
        }
    }
}

#[derive(Clone, Debug)]
pub struct PortConnections<'a> {
    conns: HashMap<&'a ast::Ident, ir::Wires>,
}

impl<'a> PortConnections<'a> {
    pub fn new(port_args: &'a [ast::PortArg], wire_map: &HashMap<&ast::Ident, (ir::WireId, ir::WireColor)>) -> Result<Self, CompilerError> {
        let mut conns: HashMap<_, ir::Wires> = HashMap::new();

        for ast::PortArg { port, wire } in port_args {
            if let Some((wire_id, color)) = wire_map.get(wire) {
                let wires = conns.entry(port)
                    .or_default();

                if wires.set_color(*color, *wire_id).is_some() {
                    return Err(CompilerError::PortConflict {
                        port: port.clone(),
                        color: *color,
                    });
                }
            }
        }

        Ok(Self { conns })
    }

    pub fn get(&self, ident: &ast::Ident) -> Option<&ir::Wires> {
        self.conns.get(ident)
    }
}

#[derive(Debug)]
pub struct Scope<'a> {
    /// Generic parameters of a module
    // TODO: We could also store them in separate maps.
    params: HashMap<&'a ast::Ident, Param>,

    // TODO: Move ports and wires in here.
}

impl<'a> Scope<'a> {
    pub fn new(decls: &'a Option<Vec<ast::GenericDecl>>, params: Vec<Param>) -> Result<Self, CompilerError> {
        let mut scope = HashMap::new();

        if let Some(decls) = decls {
            if decls.len() != params.len() {
                return Err(CompilerError::ParamCountMismatch { expected: decls.len(), got: params.len() });
            }

            for (decl, param) in decls.iter().zip(params.into_iter()) {
                match (decl, &param) {
                    (ast::GenericDecl::Number(ident), Param::Number {..}) => {
                        scope.insert(ident, param);
                    },
                    (ast::GenericDecl::Signal(ident), Param::Signal {..}) => {
                        scope.insert(ident, param);
                    },
                    _ => {
                        return Err(CompilerError::ParamTypeMismatch { decl: decl.to_owned(), param });
                    },
                }
            }
        }
        else if params.len() != 0 {
            return Err(CompilerError::ParamCountMismatch { expected: 0, got: params.len() });
        }

        Ok(Self {
            params: scope,
        })
    }

    pub fn lookup_numeric(&self, name: &ast::Ident) -> Result<i32, CompilerError> {
        match self.params.get(name) {
            Some(Param::Number(n)) => Ok(*n),
            Some(_) => Err(CompilerError::ParamNotANumber(name.clone())),
            None => Err(CompilerError::ParamNotFound(name.clone())),
        }
    }

    pub fn lookup_signal(&'a self, name: &ast::Ident) -> Result<&'a ir::SignalID, CompilerError> {
        match self.params.get(name) {
            Some(Param::Signal(s)) => Ok(s),
            Some(_) => Err(CompilerError::ParamNotASignal(name.clone())),
            None => Err(CompilerError::ParamNotFound(name.clone())),
        }
    }

    fn eval_unary_op<F: FnMut(i32) -> Option<i32>>(&self, expr: &ast::Expr, operand: &ast::Expr, mut f: F) -> Result<i32, CompilerError> {
        f(self.eval_expr(operand)?)
            .ok_or_else(|| CompilerError::Overflow(expr.clone()))
    }

    fn eval_binary_op<F: FnMut(i32, i32) -> Option<i32>>(&self, expr: &ast::Expr, left: &ast::Expr, right: &ast::Expr, mut f: F) -> Result<i32, CompilerError> {
        f(self.eval_expr(left)?, self.eval_expr(right)?)
            .ok_or_else(|| CompilerError::Overflow(expr.clone()))
    }

    pub fn eval_expr(&self, expr: &ast::Expr) -> Result<i32, CompilerError> {
        match expr {
            ast::Expr::Add(left, right) => self.eval_binary_op(expr, left, right, i32::checked_add),
            ast::Expr::Sub(left, right) => self.eval_binary_op(expr, left, right, i32::checked_sub),
            ast::Expr::Mul(left, right) => self.eval_binary_op(expr, left, right, i32::checked_mul),
            ast::Expr::Div(left, right) => self.eval_binary_op(expr, left, right, i32::checked_div),
            ast::Expr::Mod(left, right) => self.eval_binary_op(expr, left, right, i32::checked_rem_euclid),
            ast::Expr::BitAnd(left, right) => self.eval_binary_op(expr, left, right, |a, b| Some(a & b)),
            ast::Expr::BitOr(left, right) => self.eval_binary_op(expr, left, right, |a, b| Some(a | b)),
            ast::Expr::BitXor(left, right) => self.eval_binary_op(expr, left, right, |a, b| Some(a ^ b)),
            ast::Expr::BitNot(operand) => self.eval_unary_op(expr, operand, |a| Some(!a)),
            ast::Expr::Neg(operand) => self.eval_unary_op(expr, operand, i32::checked_neg),
            ast::Expr::Const(n) => Ok(*n),
            ast::Expr::Var(name) => Ok(self.lookup_numeric(name)?),
        }
    }

    pub fn eval_cond(&self, cond: &ast::ConstCond) -> Result<bool, CompilerError> {
        let left = self.eval_expr(&cond.left)?;
        let right = self.eval_expr(&cond.right)?;

        Ok(match cond.op {
            ir::DeciderOp::GreaterThan => left > right,
            ir::DeciderOp::LessThan => left < right,
            ir::DeciderOp::Equal => left == right,
            ir::DeciderOp::GreaterEqual => left >= right,
            ir::DeciderOp::LessEqual => left <= right,
            ir::DeciderOp::NotEqual => left != right,
        })
    }

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

    fn lookup_signal_literal(&self, r#type: SignalType, ident: &ast::Ident) -> Result<ir::SignalID, CompilerError> {
        // TODO: Check if the signal exists. In order to do that, store a set of all available signals in the compiler
        //       config. By default this can be the vanilla signals that are hard-coded, but could also be taken from
        //       the loaded prototypes.
        Ok(ir::SignalID::new(r#type, ident.as_ref().to_owned()))
    }

    fn map_input(&self, input: &ast::Input, scope: &Scope, wire_map: &HashMap<&ast::Ident, (ir::WireId, ir::WireColor)>) -> Result<(ir::Input, ir::Wires), CompilerError> {
        Ok(match input {
            ast::Input::Signal { wires, signal } => {
                (
                    match signal.as_ref().ok_or_else(|| CompilerError::Todo)? {
                        ast::Signal::Each => ir::Input::ForEach,
                        ast::Signal::All => ir::Input::Everything,
                        ast::Signal::Any => ir::Input::Anything,
                        ast::Signal::Virtual(ident) => ir::Input::Signal(self.lookup_signal_literal(SignalType::Virtual, ident)?),
                        ast::Signal::Item(ident) => ir::Input::Signal(self.lookup_signal_literal(SignalType::Item, ident)?),
                        ast::Signal::Fluid(ident) => ir::Input::Signal(self.lookup_signal_literal(SignalType::Fluid, ident)?),
                        ast::Signal::Var(var) => ir::Input::Signal(scope.lookup_signal(var)?.clone()),
                    },
                    Self::map_wires(wire_map, wires)?,
                )
            },
            ast::Input::Number(expr) => {
                (
                    ir::Input::Constant(scope.eval_expr(expr)?),
                    ir::Wires::default(),
                )
            },
        })
    }

    fn map_output(&self, output: &ast::Output, scope: &Scope, wire_map: &HashMap<&ast::Ident, (ir::WireId, ir::WireColor)>) -> Result<(ir::OutputSignal, ir::Wires), CompilerError> {
        let signal = output.signal.as_ref().ok_or_else(|| CompilerError::Todo)?;
        Ok((
            match signal {
                ast::Signal::Each => ir::OutputSignal::ForEach,
                ast::Signal::All => ir::OutputSignal::Everything,
                ast::Signal::Any => return Err(CompilerError::SignalNotAllowed(signal.clone())),
                ast::Signal::Virtual(ident) => ir::OutputSignal::Signal(self.lookup_signal_literal(SignalType::Virtual, ident)?),
                ast::Signal::Item(ident) => ir::OutputSignal::Signal(self.lookup_signal_literal(SignalType::Item, ident)?),
                ast::Signal::Fluid(ident) => ir::OutputSignal::Signal(self.lookup_signal_literal(SignalType::Fluid, ident)?),
                ast::Signal::Var(var) => ir::OutputSignal::Signal(scope.lookup_signal(var)?.clone()),
            },
            Self::map_wires(wire_map, &output.wires)?,
        ))
    }

    pub fn compile_module(&self, mod_ident: &ast::Ident, params: Vec<Param>, port_conns: Option<PortConnections<'a>>) -> Result<ir::Ir, CompilerError> {
        let module = *self.modules.get(mod_ident)
            .ok_or_else(|| CompilerError::UnknownModule(mod_ident.to_owned()))?;

        let scope = Scope::new(&module.generics, params)?;
        // TODO: Put wires into scope
        let mut wire_map: HashMap<&ast::Ident, (ir::WireId, ir::WireColor)> = HashMap::new();
        // TODO: I think we don't need `ports` if we don't export them.
        let mut ports: HashMap<(&ast::Ident, ir::WireColor), ir::WireId> = HashMap::new();
        let mut combinators = vec![];

        log::trace!("port_conns = {:?}", port_conns);

        for port_decl in &module.body.ports {
            match port_decl {
                ast::PortDecl::Both { ident: port_ident, red, green } => {
                    let port_wires = port_conns
                        .as_ref()
                        .and_then(|port_conns| port_conns.get(port_ident).copied())
                        .unwrap_or_default();

                    log::trace!("port_wires = {:?}", port_wires);

                    if !red.is_placeholder() {
                        let red_id = port_wires.red.unwrap_or_else(|| self.wire_ids.next());
                        log::trace!("Connect port {:?} to red wire_id={}", port_decl, red_id);
                        ports.insert((port_ident, ir::WireColor::Red), red_id);
                        wire_map.insert(red, (red_id, ir::WireColor::Red));
                    }

                    if !green.is_placeholder() {
                        let green_id = port_wires.green.unwrap_or_else(|| self.wire_ids.next());
                        log::trace!("Connect port {:?} to green wire_id={}", port_decl, green_id);
                        ports.insert((port_ident, ir::WireColor::Green), green_id);
                        wire_map.insert(green, (green_id, ir::WireColor::Green));
                    }
                },

                ast::PortDecl::ShortHand { ident: port_ident, color } => {
                    let port_wires = port_conns
                        .as_ref()
                        .and_then(|port_conns| port_conns.get(port_ident).copied())
                        .unwrap_or_default();

                    log::trace!("port_wires = {:?}", port_wires);

                    let wire_id = port_wires.get_color(*color).unwrap_or_else(|| self.wire_ids.next());
                    log::trace!("Connect port {:?} to wire_id={}", port_decl, wire_id);
                    ports.insert((port_ident, *color), wire_id);
                    wire_map.insert(port_ident, (wire_id, *color));
                },
            }
        }

        for ast::WireDecl { color, ident } in &module.body.wires {
            wire_map.insert(&ident, (self.wire_ids.next(), *color));
        }

        //log::debug!("Declared ports: {:#?}", ports);
        //log::debug!("Declared wires: {:#?}", wire_map);

        self.compile_statements(&module.body.statements, &mut combinators, &scope, &wire_map)?;

        Ok(ir::Ir {
            name: Some(mod_ident.as_ref().to_owned()),
            ports: Default::default(), // TODO, export port connections
            combinators,
        })
    }

    pub fn compile_statements(&self, statements: &'a [ast::Statement], combinators: &mut Vec<ir::Combinator>, scope: &Scope, wire_map: &HashMap<&ast::Ident, (ir::WireId, ir::WireColor)>) -> Result<(), CompilerError> {
        for statement in statements {
            self.compile_statement(statement, combinators, scope, wire_map)?;
        }
        Ok(())
    }

    pub fn compile_statement(&self, statement: &'a ast::Statement, combinators: &mut Vec<ir::Combinator>, scope: &Scope, wire_map: &HashMap<&ast::Ident, (ir::WireId, ir::WireColor)>) -> Result<(), CompilerError> {
        log::debug!("Compiling statement: {:?}", statement);

        match statement {
            ast::Statement::Constant { output, constants } => {
                if output.signal.is_some() {
                    return Err(CompilerError::Todo);
                }

                let mut signals = vec![];
                for ast::SignalConst { signal, value } in constants {
                    let signal_id = match signal {
                        ast::Signal::Virtual(ident) => self.lookup_signal_literal(SignalType::Virtual, ident)?,
                        ast::Signal::Item(ident) => self.lookup_signal_literal(SignalType::Item, ident)?,
                        ast::Signal::Fluid(ident) => self.lookup_signal_literal(SignalType::Fluid, ident)?,
                        ast::Signal::Var(var) => scope.lookup_signal(var)?.clone(),
                        _ => return Err(CompilerError::SignalNotAllowed(signal.clone())),
                    };
                    signals.push(signal_id.into_signal(scope.eval_expr(value)?));
                }

                combinators.push(ir::Combinator::Constant(ir::ConstantCombinator {
                    signals,
                    wires: Self::map_wires(&wire_map, &output.wires)?,
                }));
            },

            ast::Statement::Arithmetic { output, op, left, right } => {
                let (left_input, left_wires) = self.map_input(left, &scope, &wire_map)?;
                let (right_input, right_wires) = self.map_input(right, &scope, &wire_map)?;
                let input_wires = Self::merge_wires(left_wires, right_wires)?;
                let (output_signal, output_wires) = self.map_output(output, &scope, &wire_map)?;

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
                let (left_input, left_wires) = self.map_input(left, &scope, &wire_map)?;
                let (right_input, right_wires) = self.map_input(right, &scope, &wire_map)?;
                let input_wires = Self::merge_wires(left_wires, right_wires)?;
                let (output_signal, output_wires) = self.map_output(output, &scope, &wire_map)?;

                let output_count = match mode {
                    ast::DeciderMode::One => ir::OutputCount::One,
                    ast::DeciderMode::Passthrough => ir::OutputCount::InputSignal,
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

            ast::Statement::Lamp { op, left, right } => {
                let (left_input, left_wires) = self.map_input(left, &scope, &wire_map)?;
                let (right_input, right_wires) = self.map_input(right, &scope, &wire_map)?;
                let input_wires = Self::merge_wires(left_wires, right_wires)?;

                combinators.push(ir::Combinator::Lamp(ir::Lamp {
                    op: *op,
                    left: left_input,
                    right: right_input,
                    input_wires,
                    use_color: false,
                }));
            }

            ast::Statement::ModuleInst { ident, generics, ports } => {
                let mut params = vec![];
                if let Some(generics) = generics {
                    for param in generics {
                        params.push(match param {
                            ast::GenericArg::Number(expr) => Param::Number(scope.eval_expr(expr)?),
                            ast::GenericArg::Signal(signal) => {
                                let signal_id = match signal {
                                    ast::Signal::Virtual(ident) => self.lookup_signal_literal(SignalType::Virtual, ident)?,
                                    ast::Signal::Item(ident) => self.lookup_signal_literal(SignalType::Item, ident)?,
                                    ast::Signal::Fluid(ident) => self.lookup_signal_literal(SignalType::Fluid, ident)?,
                                    ast::Signal::Var(var) => scope.lookup_signal(var)?.clone(),
                                    _ => return Err(CompilerError::SignalNotAllowed(signal.clone())),
                                };
                                Param::Signal(signal_id)
                            },
                        });
                    }
                }

                let port_conns = PortConnections::new(ports, &wire_map)?;

                let mut ir = self.compile_module(ident, params, Some(port_conns))?;

                combinators.append(&mut ir.combinators);
            },

            ast::Statement::Conditional { cond, then_case, else_case } => {
                if scope.eval_cond(cond)? {
                    self.compile_statements(then_case, combinators, scope, wire_map)?;
                }
                else if let Some(else_case) = else_case {
                    self.compile_statements(else_case, combinators, scope, wire_map)?;
                }
            },
        }

        Ok(())
    }
}
