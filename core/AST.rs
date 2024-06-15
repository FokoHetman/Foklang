use core::tokenizer::{Operator};
use core::builtins;


#[derive(Clone, Debug)]
pub enum NodeKind {
  Program {body: Vec<Box<Node>>, id: i32},
  Identifier {symbol: String, childs: Vec<Box<Node>>},
  NumericLiteral{ value: NodeValue},
  Expression,
  BinaryExpression{ left: Box<Node>, right: Box<Node>, operator: Operator},
  Stmt,
  Char{value: NodeValue},
  NullLiteral{value: NodeValue},
  List{body: Vec<Box<Node>>},
  Config{arguments: Vec<(Box<Node>, Box<Node>)>},
  FunctionDeclaration{identifier: Box<Node>,/* arguments: Vec<Box<Node>>,*/ statement: Box<Node>},
}
#[derive(Debug, Clone)]
pub enum NodeValue {
  Integer(i32),
  String(String),
  Char(char),
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
  Ustulo(char),
  Filum(String),
  Moenus(/*Node,*/ Vec<Node>, Node),
  BuiltIn(fn(builtins::Arguments) -> Proventus),
  Causor(Vec<(Node,Proventus)>),
  Inventarii(Vec<Proventus>),
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

impl Fructa {
  pub fn display(&self) -> String {
    match self {
      Fructa::Nullus => String::new(),
      Fructa::Numerum(i) => i.to_string(),
      Fructa::Filum(s) => s.to_string(),
      Fructa::Causor(b) => format!("{:#?}", b),
      Fructa::Inventarii(b) => {
        let mut result = String::new();
        let mut string = String::new();
        let mut stringlist = true;

        for i in b {
          string += &match i.value {
            Fructa::Ustulo(c) => c.to_string(),
            _ => {
              stringlist = false;
              String::new()
            }
          };
          result+=&(i.value.display()+";")
        }
        if stringlist {
          string
        } else {
          result
        }
      },
      Fructa::Ustulo(c) => c.to_string(),
      _ => panic!("display not implemented")
    }
  }
}
/*impl Proventus {
  fn get(self, key: Proventus) -> Proventus {
    let mut returnd = Proventus{value:Fructa::Nullus, id:-3};
    match self.value {
      Fructa::Causor(arguments) => {
        match key.value {
          Fructa::Filum(s) => {
            for i in arguments {
              match i.0.kind {
                NodeKind::Identifier{symbol} => {
                  if symbol==s {
                    returnd = i.1;
                  }
                }
                _ => panic!("A")
              }
            }
          }
          _ => panic!("a")
        }
      }
      _ =>  panic!("damnAST")
    }
    returnd
  }
}*/

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
