use core::AST::{*};


pub fn get(arguments: Arguments) -> Proventus {
    let mut returnd = Proventus{value: Fructa::Nullus, id: -3};
    match arguments.function {
      FunctionArgs::get(causor, key) => {
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
      }
      _ => panic!("damn this AST")
    }
    returnd
}


pub struct Arguments {
  pub function: FunctionArgs,
}

pub enum FunctionArgs {
  get(Proventus, Proventus),
}
