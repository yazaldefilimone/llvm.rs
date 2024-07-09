use std::fmt;

use crate::ir::{IRConstant, IRFunction, IRInstruction, IRModule, IRValue};

impl fmt::Display for IRValue {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      IRValue::Constant(c) => write!(f, "{}", c),
      IRValue::Variable(v) => write!(f, "%{}", v),
    }
  }
}

impl fmt::Display for IRConstant {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      IRConstant::Integer32(i) => write!(f, "i32 {}", i),
      IRConstant::Integer64(i) => write!(f, "i64 {}", i),
      IRConstant::Float32(i) => write!(f, "f32 {}", i),
      IRConstant::Float64(i) => write!(f, "f64 {}", i),
      IRConstant::Unsigned32(i) => write!(f, "u32 {}", i),
      IRConstant::Unsigned64(i) => write!(f, "u64 {}", i),
    }
  }
}

impl fmt::Display for IRInstruction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      IRInstruction::BinaryOperation { dest, left, right, operator } => {
        write!(f, "{} = {:?} {}, {}", dest, operator, left, right)
      }
      IRInstruction::UnaryOperation { dest, operand, operator } => {
        write!(f, "{} = {:?} {}", dest, operator, operand)
      }
      IRInstruction::Load { dest, address } => {
        write!(f, "{} = load {}", dest, address)
      }
      IRInstruction::Store { address, value } => {
        write!(f, "store {}, {}", address, value)
      }
      IRInstruction::Return { value } => {
        if let Some(val) = value {
          write!(f, "ret {}", val)
        } else {
          write!(f, "ret void")
        }
      }
      IRInstruction::Call { dest, function, args } => {
        let args_str = args
          .iter()
          .map(|arg| format!("{}", arg))
          .collect::<Vec<String>>()
          .join(", ");
        if let Some(dest) = dest {
          write!(f, "{} = call {}({})", dest, function, args_str)
        } else {
          write!(f, "call {}({})", function, args_str)
        }
      }
      IRInstruction::Conditional { condition, then_instructions, else_instructions } => {
        let then_str = then_instructions
          .iter()
          .map(|instr| format!("{}", instr))
          .collect::<Vec<String>>()
          .join("\n");
        let else_str = else_instructions
          .iter()
          .map(|instr| format!("{}", instr))
          .collect::<Vec<String>>()
          .join("\n");
        write!(
          f,
          "if {} then {{\n{}\n}} else {{\n{}\n}}",
          condition, then_str, else_str
        )
      }
      IRInstruction::Loop { condition, body } => {
        let body_str = body
          .iter()
          .map(|instr| format!("{}", instr))
          .collect::<Vec<String>>()
          .join("\n");
        write!(f, "loop {} {{\n{}\n}}", condition, body_str)
      }
    }
  }
}

impl fmt::Display for IRFunction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let args_str = self.params.join(", ");
    let body_str = self
      .body
      .iter()
      .map(|instr| format!("{}", instr))
      .collect::<Vec<String>>()
      .join("\n");
    write!(f, "define @{}({}) {{\n{}\n}}", self.name, args_str, body_str)
  }
}

impl fmt::Display for IRModule {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let functions_str = self
      .functions
      .iter()
      .map(|func| format!("{}", func))
      .collect::<Vec<String>>()
      .join("\n");
    write!(f, "module {{\n{}\n}}", functions_str)
  }
}
