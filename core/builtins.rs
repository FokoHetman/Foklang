use core::AST::{*};
use core::env::Environment;
use core::interpreter::Interpreter;



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

pub fn fmap(arguments: Arguments) -> Proventus {
  match arguments.function {
    FunctionArgs::fmap(fun, list, env, mut interpreter) => {
      match fun.kind {
        NodeKind::Identifier{symbol, ..} => {
          match list.value {
            Fructa::Inventarii(inv) => {
              let mut result: Vec<Proventus> = vec![];
              for i in inv {
                match &env.get(Node{kind: NodeKind::Identifier{symbol: symbol.clone(), childs: vec![]}}).into_iter().rev().collect::<Vec<Proventus>>()[0].value {
                  Fructa::Moenus(args, statement) => {
                    let mut function_env = Environment{parent: Some(Box::new(env.clone())), ..Default::default()};
                    if args.len()>1 {
                      match i.value {
                        Fructa::Inventarii(body) => {
                          for x in 0..args.len() {
                            function_env.declare(args[x].clone(), body[x].clone());
                          }
                        }
                        _ => panic!("iterating not implemented for whatever you tried lmao")
                      }
                    } else {
                      function_env.declare(args[0].clone(), i);
                    }

                    result.push(interpreter.evaluate(statement.clone(), &mut function_env));
                  }
                  _ => panic!("supra nova")
                }
              }
              return Proventus{value: Fructa::Inventarii(result), id: -1};
            }
            _ => panic!("not list list")
          }
        }
        _ => panic!("explosiod gbfdrsupra")
      }
    }
    _ => panic!("head")
  }
  //panic!("A")
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


pub fn join(arguments: Arguments) -> Proventus {
  match arguments.function {
    FunctionArgs::join(mut lists) => {
      let mut result = lists[0].clone();
      lists.remove(0);
      match result.value {
        Fructa::Inventarii(ref mut main) => {
          for li in lists {
            match li.value {
              Fructa::Inventarii(li1) => {
                main.append(&mut li1.clone());
              },
              _ => panic!("ar")
            }
          }
        }
        _ => panic!("ra")
      }
      result
    }
    _  => panic!("??????")
  }
}



pub fn declare_fn(id: String, fun: fn(Arguments) -> Proventus, env: &mut Environment) {
  env.declare(Node{kind: NodeKind::Identifier {symbol: id, childs:vec![]}},
      Proventus{value: Fructa::BuiltIn(
        fun
      ), id:-2});
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
  declare_fn(String::from("fmap"), fmap, env);
  declare_fn(String::from("join"), join, env);
}



#[derive(Debug)]
pub struct Arguments {
  pub function: FunctionArgs,
}
#[derive(Debug)]
pub enum FunctionArgs {
  get(Proventus, Proventus),
  print(Vec<Proventus>),
  fmap(Node, Proventus, Environment, Interpreter),
  zerum(),
  join(Vec<Proventus>),
}
