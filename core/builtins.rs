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

pub fn returnfn(arguments: Arguments) -> Proventus {
  match arguments.function {
    FunctionArgs::returnfn(value) => {
      return value
    },
    _ => panic!("dumbass interpreter")
  }
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


pub fn data(arguments: Arguments) -> Proventus {
  match arguments.function {
    FunctionArgs::data(id, params, env) => {
      //env.declare_type(id,  params);
      Proventus {value: Fructa::Nullus, id: -1}
    }
    _ => panic!("Interpreter error: Wrong args provided (should never happen)")
  }
}

pub fn type_of(arguments: Arguments) -> Proventus {
  match arguments.function {
    FunctionArgs::type_of(val) => {
      Proventus {value: Fructa::Filum(val.value.display_type()), id: -1}
    }
    _ => panic!("Interpreter error")
  }
}





pub fn declare_fn(id: String, fun: fn(Arguments) -> Proventus, env: &mut Environment) {
  env.declare(Node{kind: NodeKind::Identifier {symbol: id, childs:vec![]}},
      Proventus{value: Fructa::BuiltIn(
        fun
      ), id:-2});
}

pub fn declare_builtins(env: &mut Environment) {
  /*declare_fn(String::from("get"), get, env);
  declare_fn(String::from("print"), print, env);
  declare_fn(String::from("println"), println, env);
  declare_fn(String::from("fmap"), fmap, env);
  declare_fn(String::from("join"), join, env);
  declare_fn(String::from("return"), returnfn, env);*/

  let functions = vec![
    (String::from("get"), get as fn(Arguments) -> Proventus), (String::from("print"), print), (String::from("println"), println),
    (String::from("fmap"), fmap), (String::from("join"), join), (String::from("return"), returnfn), (String::from("data"), data),
    (String::from("t"), type_of),
  ];
  for i in functions {
    declare_fn(i.0, i.1, env);
  }
}



#[derive(Debug)]
pub struct Arguments {
  pub function: FunctionArgs,
}
#[derive(Debug)]
pub enum FunctionArgs {
  returnfn(Proventus),                                  // (value_to_return)

  get(Proventus, Proventus),                            // (config, identifier)
  print(Vec<Proventus>),
  fmap(Node, Proventus, Environment, Interpreter),      // (function_identifier, list)
  zerum(),                                              // I don't remember implementing that
  join(Vec<Proventus>),                                 // ([lists]), ex. (List1, List2)
  data(Node, Vec<Node>, Environment),                   // (type_identifier,  [Parameterers]) ex. (Point Int Int) / (Point Float Float)
  type_of(Proventus),                                   // (value_to_get_type_of)
}
