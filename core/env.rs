use core::{AST, error_handler::ErrorHandler};

#[derive(Clone,Debug)]
pub struct Environment {pub parent: Option<Box<Environment>>, pub functions: Vec<(AST::Node, AST::Proventus)>, 
    pub node_stack: Vec<AST::Node>, pub current_node: i32, pub error_handler: ErrorHandler}


impl Environment {
  pub fn declare(&mut self, identifier: AST::Node, value: AST::Proventus) -> AST::Proventus {
    match identifier.kind {
      AST::NodeKind::Identifier{symbol: ref symbol, ..} => {
        for i in &self.functions {
          if match &i.0.kind {
            AST::NodeKind::Identifier{symbol:symbolIteration, ..} => *symbolIteration==*symbol,
            _ => panic!("Tf is that doing here? {:#?}", i)
          } {
            panic!("{}", self.error_handler.environment("already_defined").error_msg);
          }
        }

      }
      _ => {
        panic!("{} {:#?}", self.error_handler.environment("nonidentifier_node").error_msg, identifier);
      }

    }
    self.functions.push((identifier, value.clone()));
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
  pub fn get(&self, identifier: AST::Node) -> AST::Proventus {
    let env = self.resolve(identifier.clone());
    match identifier.kind {
      AST::NodeKind::Identifier{symbol: ref s, ..} => {
        for i in env.functions {
          if match &i.0.kind {
            AST::NodeKind::Identifier{symbol: s2, ..} => {
              *s==*s2
            }
            _ => panic!("a")
          } {
            return i.1
          }
        };
        return AST::Proventus{value: AST::Fructa::Nullus, id: -2}
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
      error_handler: ErrorHandler {},
      node_stack: [].to_vec(),
      current_node: 0,
    }
  }
}
