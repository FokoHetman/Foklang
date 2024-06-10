use core::tokenizer::{Operator};
//MAKE IT USE ENUM ()THING
#[derive(Clone, Debug)]
pub enum NodeKind {
  Program {body: Vec<Box<Node>>, id: i32},
  Identifier {symbol: String},
  NumericLiteral{ value: NodeValue},
  Expression,
  BinaryExpression{ left: Box<Node>, right: Box<Node>, operator: Operator},
  Stmt,
  NullLiteral{value: NodeValue},
  FunctionDeclaration{identifier: Box<Node>, arguments: Vec<Box<Node>>, statement: Box<Node>},
}
#[derive(Debug, Clone)]
pub enum NodeValue {
  Integer(i32),
  String(String),
  Nullus,
}

#[derive(Clone, Debug)]
pub struct Node {
  pub kind: NodeKind,
}
#[derive(Clone,Debug)]
pub enum Fructa {
  Nullus,
  Numerum(i32),
  Filum(String),
  Moenus(/*Node,*/ Vec<Node>, Node),
}
#[derive(Clone,Debug)]
pub struct Proventus {
  pub value: Fructa,
  pub id: i32,
}

impl Default for Proventus {
  fn default() -> Proventus {
    Proventus{value: Fructa::Nullus, id: 0}
  }
}
/*impl Default for NodeKind {
  fn default() -> Self{ NodeKind::BinaryExpression{left: Box<Node>, right: Box<Node>, operator: Operator} } {//NodeKind {
    NodeKind::BinaryExpression{left: Box::<Node>::new(Node{kind: NodeKind::NullLiteral{value: NodeValue::Nullus}}),
        right: Box::<Node>::new(Node{kind: NodeKind::NullLiteral{value: NodeValue::Nullus}}),
        operator: Operator::Addition,
    }
  }
}
impl Default for Node {
  fn default() -> Node {
    Node {
      kind: NodeKind::NullLiteral,
      body: None,
      left: None,
      right: None,
      symbol: None,
      operator: None,
      value: None,
    }
  }
}*/
impl Node {
  pub fn appendToBody(&mut self, node: Node) {
    let mut boxed = Box::<Node>::new(node);
    match self.kind {
      NodeKind::Program{body: ref mut s ,id:_} => {
        s.push(boxed);
      }
      _ => panic!("AST Error: Tried to append a Node to non-Program Node")
    }
  }
}
