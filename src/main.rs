mod allocator;
mod core;
mod diagnostics;
mod formatter;
mod ir;
mod optimizer;
mod utils;
fn main() {
  // let a = 3;
  let assign_a = ir::IRInstruction::BinaryOperation {
    dest: "%a".to_string(),
    left: ir::IRValue::Constant(ir::IRConstant::Integer32(3)),
    right: ir::IRValue::Constant(ir::IRConstant::Integer32(0)), // Dummy operand
    operator: ir::BinaryOperator::ADD,                          // Dummy operation
  };

  // let b = 4;
  let assign_b = ir::IRInstruction::BinaryOperation {
    dest: "%b".to_string(),
    left: ir::IRValue::Constant(ir::IRConstant::Integer32(4)),
    right: ir::IRValue::Constant(ir::IRConstant::Integer32(0)), // Dummy operand
    operator: ir::BinaryOperator::ADD,                          // Dummy operation
  };

  // let c = a + b;
  let add_c = ir::IRInstruction::BinaryOperation {
    dest: "%c".to_string(),
    left: ir::IRValue::Variable("%a".to_string()),
    right: ir::IRValue::Variable("%b".to_string()),
    operator: ir::BinaryOperator::ADD,
  };

  // return c;
  let return_c = ir::IRInstruction::Return { value: Some(ir::IRValue::Variable("%c".to_string())) };

  // Função main
  let function_main =
    ir::IRFunction { name: "main".to_string(), params: vec![], body: vec![assign_a, assign_b, add_c, return_c] };

  // Módulo
  let module = ir::IRModule { functions: vec![function_main] };

  // Imprime a representação IR para debug
  println!("{}", module);
}
