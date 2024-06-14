use core::AST::{*};
use core::env::Environment;

pub fn print(arguments: Arguments) -> Proventus {
  match arguments.function {
    FunctionArgs::print(args) => {
      for i in args {
        print!("{}", i.value.display());
        /*match i.value {
          Fructa::Filum(s) => {print!("{}", s)},
          Fructa::Numerum(i) => {print!("{}", i)},
          _ => panic!("Display not implemented for: {:#?}", i.value)
        }*/
      }
    }
    _ => panic!("???")
  }
  Proventus{value: Fructa::Nullus, id: -2}
}

pub fn println(arguments: Arguments) -> Proventus {
  match arguments.function {
    FunctionArgs::print(args) => {
      for i in args {
        println!("{}", i.value.display());
      }
    }
    _ => panic!("???")
  }
  Proventus{value: Fructa::Nullus, id: -2}
}

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
                  NodeKind::Identifier{symbol, ..} => {
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
        },
        Fructa::Inventarii(body) => {
          println!("{:#?}", key);
          match key.value {
            Fructa::Numerum(i) => {
              returnd = body[i as usize].clone();

            }
            _ => panic!("index expected damn man")
          }
        }
        _ =>  panic!("damnAST")
      }
    }
    _ => panic!("damn this AST")
  }
  returnd
}

pub fn declare_builtins(env: &mut Environment) {
  env.declare(Node{kind: NodeKind::Identifier{symbol: String::from("get"), childs: vec![]}},
      Proventus{value: Fructa::BuiltIn(
        get
      ),id:-2});
  env.declare(Node{kind: NodeKind::Identifier{symbol: String::from("print"), childs: vec![]}},
      Proventus{value: Fructa::BuiltIn(
        print
      ),id:-2});
  env.declare(Node{kind: NodeKind::Identifier{symbol: String::from("println"), childs: vec![]}},
      Proventus{value: Fructa::BuiltIn(
        println
      ), id:-2});
}



#[derive(Debug)]
pub struct Arguments {
  pub function: FunctionArgs,
}
#[derive(Debug)]
pub enum FunctionArgs {
  get(Proventus, Proventus),
  print(Vec<Proventus>),
  zerum(),
}
