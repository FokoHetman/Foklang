use core::AST::{*};

pub fn print(arguments: Arguments) -> Proventus {
  match arguments.function {
    FunctionArgs::print(args) => {
      for i in args {
        match i.value {
          Fructa::Filum(s) => {print!("{}", s)},
          Fructa::Numerum(i) => {print!("{}", i)},
          _ => panic!("Display not implemented for: {:#?}", i.value)
        }
      }
    }
    _ => panic!("???")
  }
  return Proventus{value: Fructa::Nullus, id: -2}
}
/*pub fn println(arguments: Arguments) -> Proventus {


}*/

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
        }
        _ =>  panic!("damnAST")
      }
    }
    _ => panic!("damn this AST")
  }
  returnd
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
