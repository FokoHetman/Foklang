#[derive(Clone, Debug)]
pub enum NodeKind {
  Program,
  Identifier,
  NumericLiteral,
  Expression,
  BinaryExpression,
  Stmt,
  NullLiteral,
  VariableDeclaration
}

#[derive(Clone, Debug)]
pub struct Node {
  pub kind: NodeKind,
  pub body: Option<Vec<Box<Node>>>,
  pub left: Option<Box<Node>>,
  pub right: Option<Box<Node>>,
  pub symbol: Option<String>,
}


impl Default for Node {
  fn default() -> Node {
    Node {
      kind: NodeKind::NullLiteral,
      body: None,
      left: None,
      right: None,
      symbol: None,
    }
  }
}
