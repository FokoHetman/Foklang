use core::tokenizer::{Operator};
use core::builtins;


#[derive(Clone, Debug, PartialEq)]
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
  ListConcat{item: Box<Node>, list: Box<Node>},
  Bool{value: NodeValue},
  Config{arguments: Vec<(Box<Node>, Box<Node>)>, flags: Vec<ConfigFlag>},
  Access{parent: Box<Node>, value: Box<Node>},
  FunctionDeclaration{identifier: Box<Node>,/* arguments: Vec<Box<Node>>,*/ statement: Box<Node>},
  TypeDeclaration{identifier: Box<Node>, ftype: Box<Node>},
  IfStatement{condition: Box<Node>, body: Box<Node> /*specific Config*/},

  Case{value: Box<Node>},
  Match{left: Box<Node>, values: Vec<Box<Node>>},
  AdvancedDeclaration{body: Box<Node>, assumptions: Vec<Box<Node>>},
  DefineBounds{identifier: Box<Node>, bound: Box<Node>},
}
#[derive(Debug, Clone, PartialEq)]
pub enum NodeValue {
  Integer(i32),
  String(String),
  Char(char),
  Bool(bool),
  Nullus,
}
#[derive(Debug,Clone,PartialEq)]
pub enum ConfigFlag {
  CodeBlock,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
  pub kind: NodeKind,
}



#[derive(Debug, Clone, PartialEq)]
struct Int {                                // define as a for each a `elem` Z
  value: i32,                               // is a
}

#[derive(Debug, Clone, PartialEq)]
struct Integer {                            // define as a for each a `elem` Z
  components: Vec<i32>, // these create a bigger number
}

#[derive(Debug,Clone,PartialEq)]
struct Fraction {                           // define as p/q for each p,q `elem` R
  numeral: Box<RealNumericalConcept>,       // is p
  nominal: Box<RealNumericalConcept>,       // is q
}


#[derive(Debug,Clone,PartialEq)]
enum RealNumericalConcept {                 // define as a for each not a `elem` I
  Int(Int),
  Integer(Integer),
  Fraction(Fraction),
}




#[derive(Debug,Clone,PartialEq)]
struct ImaginaryNumber {                    // define a + bi; a,b `elem` R
  real: RealNumericalConcept,               // is a
  imaginary: RealNumericalConcept,          // is b
}

#[derive(Debug,Clone,PartialEq)]
enum ImaginaryNumericalConcept {
  ImaginaryNumber(ImaginaryNumber),
}

#[derive(Debug,Clone,PartialEq)]
enum NumericalConcept {
  Real(RealNumericalConcept),
  Imaginary(ImaginaryNumericalConcept),
}

#[derive(Debug,Clone,PartialEq)]
enum LiteralConcept {

}


#[derive(Debug,Clone,PartialEq)]
pub enum Concept {
  Nullus,
  Numerical(NumericalConcept),
  Literal(LiteralConcept),
  /*Conditional(ConditionalConcept),
  Function(FunctionConcept),
  List(ListConcept),*/
}

/*#[derive(Clone,Debug, PartialEq)]
pub enum Fructa {
  Nullus,
  Numerum(i32),
  Ustulo(char),
  Filum(String),
  Condicio(bool),
  Moenus(/*Node,*/ Vec<Node>, Node),
  BuiltIn(fn(builtins::Arguments) -> Proventus, /*cached args*/ Vec<Node>),
  Causor(Vec<(Node,Proventus)>),
  Inventarii(Vec<Proventus>),
}*/


impl NodeKind {
  pub fn display(&self) -> String {
    match self {
      NodeKind::BinaryExpression{left,right,operator} => {
        let mut result = String::new();
        result += &left.kind.display();
        result += " ";
        result += match operator {
          Operator::Addition => "+",
          Operator::Substraction => "-",
          Operator::Multiplication => "*",
          Operator::Division => "/",
          _ => "?",
        };
        result += " ";
        result += &right.kind.display();
        result
      },
      NodeKind::NumericLiteral{value} => {
        match value {
          NodeValue::Integer(i) => i.to_string(),
          _ => panic!("noo")
        }
      },
      NodeKind::Identifier{symbol,..} => symbol.to_string(),
      NodeKind::List{body} => {
        let mut result = String::from("[");
        

        for i in body {
          result+=&(i.kind.display()+" ")
        }
        result+"]"
      },
      _ => panic!("display not implemented for: {:#?}", self)
    }

  }
  pub fn evaluate_identifier_type(&self, statement: Node) -> String {
    //for our purposes, always assume int because too lazy to implement it rn
    String::from("Int")
  }
  pub fn evaluate_type(&self) -> String {
    match self {
      NodeKind::NumericLiteral{..} => {
        String::from("Int")
      },
      NodeKind::NullLiteral{..} => {
        String::from("Null")
      },
      NodeKind::Char{..} => {
        String::from("Char")
      },
      NodeKind::Bool{..} => {
        String::from("bool")
      },
      NodeKind::Config{arguments, flags} => {
        let mut result = String::from("{");
        for i in arguments {
          result += &format!("{}: {};", i.0.kind.display(), i.1.kind.evaluate_type());
        }
        result += "}";
        result
      },
      //NodeKind::Identifier{..} => {
        
      //},
      NodeKind::BinaryExpression{left, right, operator} => {
        match left.kind {
          NodeKind::NumericLiteral{..} => {
            match right.kind {
              NodeKind::NumericLiteral{..} => {
                left.kind.evaluate_type()
              },
              NodeKind::Identifier{..} => {
                left.kind.evaluate_type()   // assume Int + Int
              },
              _ => panic!("???")
            }
          },
          NodeKind::Identifier{..} => {
            match right.kind {
              NodeKind::NumericLiteral{..} => {
                right.kind.evaluate_type() // assume Int + Int
              },
              NodeKind::Identifier{..} => {
                String::from("Int") // for lack of better thing to do, assume Int + Int
              }
              _ => panic!("???")
            }
          },
          /*NodeKind::List{..} => {
            match *right.kind {
              NodeKind::List{..} => {
                match operator {
                  Operator::Addition => left.kind.evaluate_type(),
                  Operator::Multiplication => left.kind.evaluate_type(),
                  _ => panic!("no impl")
                }
              },
              _ => panic!("?????")
            }
          },*/
          _ => panic!("??????????")
        }
      },
      NodeKind::List{body} => {
        let mut result = String::from("[");
        if body.len()>0 {
          result += &(*body[0].kind.evaluate_type());
        }
        result += "]";
        result
      },
      _ => panic!("pre-evaluation not implemented for {:#?}", self)
      //NodeKind::
    }
  }
}

/*
#[derive(Clone, Debug, PartialEq)]
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
  Bool{value: NodeValue},
  Config{arguments: Vec<(Box<Node>, Box<Node>)>},
  FunctionDeclaration{identifier: Box<Node>,/* arguments: Vec<Box<Node>>,*/ statement: Box<Node>},
}*/


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
