#[derive(Debug, Clone)]
pub enum BinaryOperator {
  ADD, // +
  SUB, // -
  MUL, // *
  DIV, // /
  EQ,  // ==
  NEQ, // !=
  LT,  // <
  GT,  // >
  LE,  // <=
  GE,  // >=
  AND, // &
  OR,  // |
  NOT, // !
  SHL, // <<
  SHR, // >>
  XOR, // ^
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
  NEG, // -
  NOT, // !
}

#[derive(Debug, Clone)]
pub enum IRValue {
  Constant(IRConstant), // constant (e.g i32, i64, f32, f64, u32, u64)
  Variable(String),     // variable (e.g. %x, %y, %z, etc.)
}

#[derive(Debug, Clone)]
pub enum IRConstant {
  Integer32(i32),  // 32-bit signed integer
  Integer64(i64),  // 64-bit signed integer
  Float32(f32),    // 32-bit floating point
  Float64(f64),    // 64-bit floating point
  Unsigned32(u32), // 32-bit unsigned integer
  Unsigned64(u64), // 64-bit unsigned integer
}

#[derive(Debug, Clone)]
pub enum IRInstruction {
  // binary operations (e.g. +, -, *, /, ==, !=, <, >, <=, >=, &, |, ~)
  BinaryOperation {
    dest: String,             // destination variable (e.g. %x, %y, %z, etc.)
    left: IRValue,            // left operand
    right: IRValue,           // right operand
    operator: BinaryOperator, // binary operator
  },
  // unary operations (e.g. +, -, ~)
  UnaryOperation {
    dest: String,            // destination variable (e.g. %x, %y, %z, etc.)
    operand: IRValue,        // operand
    operator: UnaryOperator, // operator (e.g. +, -, ~)
  },
  // load value from memory (e.g. load i32, i64, f32, f64, u32, u64) at address
  Load {
    dest: String,     //  name of the destination variable (e.g. %x, %y, %z, etc.)
    address: IRValue, // address of the memory
  },
  // store value in memory (e.g. store i32, i64, f32, f64, u32, u64) at address
  Store {
    address: IRValue, // address of the memory
    value: IRValue,   // value to be stored
  },
  // return value from a function (e.g. return i32, i64, f32, f64, u32, u64)
  Return {
    value: Option<IRValue>, // value to be returned (optional)
  },
  //  call a function (e.g. call i32, i64, f32, f64, u32, u64)
  Call {
    dest: Option<String>, // name of the destination variable (optional)
    function: String,     // function to be called
    args: Vec<IRValue>,   // arguments of the function
  },
  // conditional (if-else) block
  Conditional {
    condition: IRValue,                    // Condição
    then_instructions: Vec<IRInstruction>, // instructions in the then block
    else_instructions: Vec<IRInstruction>, // instructions in the else block
  },
  // loop (while) block
  Loop {
    condition: IRValue,       // condition of continuation of the loop
    body: Vec<IRInstruction>, // body of the loop
  },
}

#[derive(Debug, Clone)]
pub struct IRFunction {
  pub name: String,             // name of the function
  pub params: Vec<String>,      // parameters of the function
  pub body: Vec<IRInstruction>, // body of the function
}

#[derive(Debug, Clone)]
pub struct IRModule {
  pub functions: Vec<IRFunction>, // List of functions in the module
}
