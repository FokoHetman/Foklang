use core::AST::{*};


pub struct Builtins {}

impl Builtins {
  fn get(causor: Proventus, key: Proventus) -> Proventus {
    let mut returnd = Proventus{value:Fructa::Nullus, id:-3};
    match causor.value {
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
}

pub struct Arguments {
  function: FunctionArgs,
}

pub enum FunctionArgs {
  get(/*CONFIG, IDENTIFIER*/),
}
