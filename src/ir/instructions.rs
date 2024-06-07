#[derive(Debug)]
pub enum IRInstruction {
  Add,    // +
  Sub,    // -
  Mul,    // *
  Div,    // /
  Eq,     // ==
  Neq,    // !=
  Lt,     // <
  Gt,     // >
  Le,     // <=
  Ge,     // >=
  And,    // &
  Or,     // |
  Not,    // !
  Shl,    // <<
  Shr,    // >>
  Xor,    // ^
  Load,   // load
  Store,  // store
  Call,   // call
  Return, // return
  Nop,    // nop
}
