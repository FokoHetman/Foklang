use crate::core::{AST, compiler, error_handler::ErrorHandler};

#[derive(Clone,Debug)]
pub struct Environment {pub parent: Option<Box<Environment>>, pub functions: Vec<(AST::Node, compiler::ANode)>, 

    pub types: Vec<(AST::Node, compiler::AType)>,
    pub node_stack: Vec<AST::Node>, pub current_node: i32, pub error_handler: ErrorHandler}


impl Environment {
  pub fn get_type(&mut self, id: AST::Node) -> Result<compiler::AType, ()> {
    for i in self.types.clone() {
      if i.0 == id {
        return Ok(i.1)
      }
    }
    Err(())
  }
  pub fn guess_type(&mut self, statement: compiler::ANode) -> compiler::AType {
    compiler::AType::Int
  }

  pub fn declare(&mut self, identifier: AST::Node, value: compiler::ANode) -> compiler::ANode {
    match identifier.kind {
      AST::NodeKind::Identifier{symbol: ref symbol, ref childs} => {
        for i in &self.functions {
          if match &i.0.kind {
            AST::NodeKind::Identifier{symbol:symbolIteration, ..} => *symbolIteration==*symbol,
            _ => panic!("Tf is that doing here? {:#?}", i)
          } {
            match &i.0.kind {
              AST::NodeKind::Identifier{symbol, childs: c2} => {
                  if childs==c2 {
                    panic!("{}", self.error_handler.environment("already_defined").error_msg);
                  }

              }
              _ => panic!("?")
            }
          }
        }

      }
      _ => {
        panic!("{} {:#?}", self.error_handler.environment("nonidentifier_node").error_msg, identifier);
      }

    }
    self.functions.push((identifier, value.clone()));
    //println!("{:#?}", self.functions);
    return value
  }
  pub fn push_args(&mut self, args: Vec<Box<AST::Node>>) {
    for i in args {
      self.node_stack.push(*i);
    }

  }
  pub fn resolve(&self, identifier: AST::Node) -> Environment {
    match identifier.kind {
      AST::NodeKind::Identifier{symbol: ref s, ..} => {
        for i in &self.functions {
          if match &i.0.kind {
            AST::NodeKind::Identifier{symbol: s2, ..} => {
              *s==*s2
            },
            _ => panic!("huh")
          } {
            return self.clone()
          }
        };
        match &self.parent {
          Some(parent) => parent.resolve(identifier),
          None => panic!("No such variable or something idk: {:#?}", identifier)
        }
      }
      _ => panic!("huh")
    }
  }
  pub fn has(&self, identifier: AST::Node) -> bool {
    match identifier.kind {
      AST::NodeKind::Identifier{symbol: ref s, ..} => {
        for i in &self.functions {
          if match &i.0.kind {
            AST::NodeKind::Identifier{symbol: s2, ..} => {
              *s==*s2
            },
            _ => panic!("huh")
          } {
            return true
          }
        };
        match &self.parent {
          Some(parent) => parent.has(identifier),
          None => false
        }
      }
      _ => panic!("huh")
    }
  }
  pub fn get(&self, identifier: AST::Node) -> Vec<compiler::ANode> {
    let env = self.resolve(identifier.clone());
    match identifier.kind {
      AST::NodeKind::Identifier{symbol: ref s, ..} => {
        let mut results: Vec<compiler::ANode> = vec![];
        for i in env.functions {
          if match &i.0.kind {
            AST::NodeKind::Identifier{symbol: s2, ..} => {
              *s==*s2
            }
            _ => panic!("a")
          } {
            results.push(i.1);
          }
        };
        return results
      }
      _ => panic!("a")
    }
  }
}

impl Default for Environment {
  fn default() -> Environment {
    Environment {
      parent: None,
      functions: [].to_vec(),
      types: [].to_vec(),
      error_handler: ErrorHandler {},
      node_stack: [].to_vec(),
      current_node: 0,
    }
  }
}
