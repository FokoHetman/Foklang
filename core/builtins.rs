use core::AST::{*};
use core::env::Environment;
use core::interpreter::Interpreter;

use std::{process::Command, env, str, fs, collections::HashMap}; // TEMPORARY SOLUTION


pub fn print(arguments: Arguments) -> Proventus {
  match arguments.function {
    FunctionArgs::many(args) => {
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
    FunctionArgs::single(value) => {
      return value
    },
    _ => panic!("dumbass interpreter")
  }
}
pub fn println(arguments: Arguments) -> Proventus {
  match arguments.function {
    FunctionArgs::many(args) => {
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
     match fun.value {
        Fructa::Moenus(args, statement) => {
          match list.value {
            Fructa::Inventarii(inv) => {
              let mut result: Vec<Proventus> = vec![];
              for i in inv {
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

pub fn length(arguments: Arguments) -> Proventus {
  match arguments.function {
    FunctionArgs::single(list) => {
      match list.value {
        Fructa::Inventarii(l) => {
          Proventus{value: Fructa::Numerum(l.len() as i32), id: -1}
        }
        _ => panic!("dat btich2: electric boogaloo")
      }
    },
    _ => panic!("dat bitch")
  }
}

pub fn take(arguments: Arguments) -> Proventus {
  match arguments.function {
    FunctionArgs::double(amount, list) => {
      match amount.value {
        Fructa::Numerum(i) => {
          match list.value {
            Fructa::Inventarii(l) => {
              Proventus{value: Fructa::Inventarii(l[0..(i as usize)].to_vec()), id: -1}
            }
            _ => panic!("dat btich2: electric boogaloo")
          }
        }
        _ => panic!("user error I think")
      }
    },
    _ => panic!("dat bitch")
  }

}

pub fn head(arguments: Arguments) -> Proventus {
  match arguments.function {
    FunctionArgs::single(list) => {
      match list.value {
        Fructa::Inventarii(l) => {
          l[0].clone()
        }
        _ => panic!("user error iirc")
      }
    }
    _ => panic!("interpreter fuck you sincerely")
  }
}

pub fn tail(arguments: Arguments) -> Proventus {
  match arguments.function {
    FunctionArgs::single(list) => {
      match list.value {
        Fructa::Inventarii(l) => {
          l.into_iter().rev().collect::<Vec<Proventus>>()[0].clone()
        }
        _ => panic!("user error iirc")
      }
    }
    _ => panic!("interpreter fuck you sincerely")
  }

}

fn combine_list_to_string(a: Proventus) -> String {
  match a.value {
    Fructa::Inventarii(l) => {
      let mut result = String::new();
      for i in l {
        result += &match i.value {
          Fructa::Ustulo(c) => c.to_string(),
          Fructa::Numerum(i) => i.to_string(),
          _ => String::new()
        };
      }
      result
    }
    _ => panic!("????????: {:#?}", a)
  }
}

pub fn split(arguments: Arguments) -> Proventus {
  match arguments.function {
    FunctionArgs::double(splitter, value) => {
      let val1 = combine_list_to_string(value);
      let splits = val1.split(&combine_list_to_string(splitter)).collect::<Vec<&str>>();
      let mut result: Vec<Proventus> = vec![];
      for i in splits {
        let mut res: Vec<Proventus> = vec![];
        for x in i.chars() {
          res.push(Proventus{value: Fructa::Ustulo(x), id: -1});
        }
        result.push(Proventus{value: Fructa::Inventarii(res), id:-1});
      }
      Proventus{value: Fructa::Inventarii(result), id: -1}
    }
    _ => panic!("?????????????")
  }
}

pub fn replace(arguments: Arguments) -> Proventus {
  match arguments.function {
    FunctionArgs::triple(to_replace, replacement, value) => {
      let val = combine_list_to_string(value).replace(&combine_list_to_string(to_replace), &combine_list_to_string(replacement));
      let mut result: Vec<Proventus> = vec![];
      for i in val.chars() {
        result.push(Proventus{value: Fructa::Ustulo(i), id:-1});
      }
      Proventus{value: Fructa::Inventarii(result), id: -1}
    }
    _ => panic!("not taking that bs")
  }
}

pub fn envf(arguments: Arguments) -> Proventus {
  match arguments.function {
    FunctionArgs::single(var_name) => {
      let val = env::var(combine_list_to_string(var_name)).unwrap();
      let mut result: Vec<Proventus> = vec![];
      for i in val.chars() {
        result.push(Proventus{value: Fructa::Ustulo(i), id: -1});
      }
      Proventus{value: Fructa::Inventarii(result), id: -1}
    }
    _ => panic!("invalid args supplied")
  }
}

pub fn get(arguments: Arguments) -> Proventus {
  let mut returnd = Proventus{value: Fructa::Nullus, id: -3};
  match arguments.function {
    FunctionArgs::double(causor, key) => {
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
    FunctionArgs::many(mut lists) => {
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
    FunctionArgs::triple(id, params, env) => {
      //env.declare_type(id,  params);
      Proventus {value: Fructa::Nullus, id: -1}
    }
    _ => panic!("Interpreter error: Wrong args provided (should never happen)")
  }
}

pub fn type_of(arguments: Arguments) -> Proventus {
  match arguments.function {
    FunctionArgs::single(val) => {
      Proventus {value: Fructa::Filum(val.value.display_type()), id: -1}
    }
    _ => panic!("Interpreter error")
  }
}


pub fn to_int(arguments: Arguments) -> Proventus {
  match arguments.function {
    FunctionArgs::single(val) => {
      let string = combine_list_to_string(val);
      Proventus {value: Fructa::Numerum(string.parse::<i32>().unwrap()), id: -1}
    }
    _ => panic!("?")
  }
}

pub fn to_string(arguments: Arguments) -> Proventus {
  match arguments.function {
    FunctionArgs::single(val) => {
      let result = match val.value {
        Fructa::Numerum(i) => i.to_string(),
        _ =>  String::new()
      };
      let mut inventarii: Vec<Proventus> = vec![];
      for i in result.chars() {
        inventarii.push(Proventus{value: Fructa::Ustulo(i), id: -1});
      }
      Proventus{value: Fructa::Inventarii(inventarii), id: -1}
    }
    _ => panic!("?")
  }
}

pub fn load_string(arguments: Arguments) -> Proventus {
  match arguments.function {
    FunctionArgs::load_file(s, mut env, mut interpreter) => {
      let parsed = interpreter.parser.parse(interpreter.tokenizer.tokenize(combine_list_to_string(s)));
      interpreter.evaluate(parsed, &mut env)
    }
    _ => panic!("?")
  }
}

pub fn load_file(arguments: Arguments) -> Proventus {
  match arguments.function {
    FunctionArgs::load_file(s, mut env, mut interpreter) => {
      let file = fs::read_to_string(combine_list_to_string(s)).unwrap();
      let parsed = interpreter.parser.parse(interpreter.tokenizer.tokenize(file));
      interpreter.evaluate(parsed, &mut env)
    }
    _ => panic!("?")
  }
}
pub fn read_file(arguments: Arguments) -> Proventus {
  match arguments.function {
    FunctionArgs::single(s) => {
      let file = fs::read_to_string(combine_list_to_string(s)).unwrap();
      let mut result: Vec<Proventus> = vec![];
      for c in file.chars() {
        result.push(Proventus{value: Fructa::Ustulo(c), id: -1});
      }
      Proventus{value: Fructa::Inventarii(result), id: -1}
    }
    _ => panic!("?")
  }
}







pub fn declare_fn(id: String, fun: fn(Arguments) -> Proventus, env: &mut Environment) {
  env.declare(Node{kind: NodeKind::Identifier {symbol: id, childs:vec![]}},
      Proventus{value: Fructa::BuiltIn(
        fun, vec![]
      ), id:-2});
}



fn parseConf(conf: String) -> HashMap<String, String> {
  let mut result: HashMap<String, String> = HashMap::new();
  for i in conf.split("\n") {
    if i.contains("=") {
      result.insert(i.to_string().split("=").collect::<Vec<&str>>()[0].to_string(),
      i.to_string().split("=").collect::<Vec<&str>>()[1].split("\0").collect::<Vec<&str>>()[0].to_string());//.split("\"").collect::<Vec<&str>>()[1].to_string());
    }
  }

  return result;
}

fn parseCPUInfo(conf: String) -> HashMap<String, String> {
  let mut result: HashMap<String, String> = HashMap::new();
  for i in conf.split("\n") {
    if i.contains(":") {
      result.insert(i.to_string().split(":").collect::<Vec<&str>>()[0].to_string().replace("\t",""),
      i.to_string().split(":").collect::<Vec<&str>>()[1].split("\0").collect::<Vec<&str>>()[0].to_string());//.split("\"").collect::<Vec<&str>>()[1].to_string());
    }
  }

  return result;
}

pub fn globals(_arguments: Arguments) -> Proventus {
  let mut s_kernel_version = fs::read_to_string("/proc/version").unwrap().split(" ").collect::<Vec<&str>>()[2].to_string();

  let mut cpuinfo = parseCPUInfo(fs::read_to_string("/proc/cpuinfo").unwrap().split("\n\n").collect::<Vec<&str>>()[0].to_string());

  let os_release = parseConf(fs::read_to_string("/etc/os-release").unwrap());
  

  let whoami = Command::new("sh").arg("-c").arg("whoami").output().unwrap();
  let hostname = Command::new("sh").arg("-c").arg("hostname").output().unwrap();
  let s_username = str::from_utf8(&whoami.stdout).unwrap().replace("\n","");
  let s_hostname = str::from_utf8(&hostname.stdout).unwrap().replace("\n","");

  let s_uptime = fs::read_to_string("/proc/uptime").unwrap().split(" ").collect::<Vec<&str>>()[0].to_string(); // TODO: make it a function!!
  //println!("{:#?}", cpuinfo);
  let s_cpumodel = cpuinfo.get("model name").unwrap();


  let mut s_id = os_release.get("ID").unwrap().replace("\n","");

  let mut s_pretty_name = os_release.get("PRETTY_NAME").unwrap().replace("\n","");

  
  //get rid of quotes
  let mut ch_pretty_name = s_pretty_name.chars();
  ch_pretty_name.next();
  ch_pretty_name.next_back();

  s_pretty_name = ch_pretty_name.collect::<String>();



  let mut username: Vec<Proventus> = vec![];
  for i in s_username.chars() {
    username.push(Proventus{value: Fructa::Ustulo(i), id:-5});
  }
  
  let mut hostname: Vec<Proventus> = vec![];
  for i in s_hostname.chars() {
    hostname.push(Proventus{value: Fructa::Ustulo(i), id:-5});
  }

  let mut pretty_name: Vec<Proventus> = vec![];
  for i in s_pretty_name.chars() {
    pretty_name.push(Proventus{value: Fructa::Ustulo(i), id:-5});
  }
  let mut id: Vec<Proventus> = vec![];
  for i in s_id.chars() {
    id.push(Proventus{value: Fructa::Ustulo(i), id:-5});
  }

  let mut kernel_version: Vec<Proventus> = vec![];
  for i in s_kernel_version.chars() {
    kernel_version.push(Proventus{value: Fructa::Ustulo(i), id:-5});
  }

  let mut cpumodel: Vec<Proventus> = vec![];
  let mut cpu_chars = s_cpumodel.chars();
  cpu_chars.next(); // skip 1st space
  for i in cpu_chars {
    cpumodel.push(Proventus{value: Fructa::Ustulo(i), id:-5});
  }

  let mut uptime: Vec<Proventus> = vec![];
  for i in s_uptime.chars() {
    uptime.push(Proventus{value: Fructa::Ustulo(i), id:-5});
  }

  Proventus{value: Fructa::Causor(
        vec![
          (Node{kind: NodeKind::Identifier{symbol: String::from("user"), childs: vec![]}}, Proventus{value: Fructa::Causor(
            vec![
              (Node{kind: NodeKind::Identifier{symbol: String::from("username"), childs: vec![]}}, Proventus{value: Fructa::Inventarii(username), id:-5}),
              (Node{kind: NodeKind::Identifier{symbol: String::from("hostname"), childs: vec![]}}, Proventus{value: Fructa::Inventarii(hostname), id:-5}),
            ]
          ), id: -5}),
          (Node{kind: NodeKind::Identifier{symbol: String::from("os"), childs: vec![]}}, Proventus{value: Fructa::Causor(
            vec![
              (Node{kind: NodeKind::Identifier{symbol: String::from("prettyname"), childs: vec![]}}, Proventus{value: Fructa::Inventarii(pretty_name), id:-5}),
              (Node{kind: NodeKind::Identifier{symbol: String::from("id"), childs: vec![]}}, Proventus{value: Fructa::Inventarii(id), id:-5}),
              (Node{kind: NodeKind::Identifier{symbol: String::from("kernel"), childs: vec![]}}, Proventus{value: Fructa::Inventarii(kernel_version), id:-5}),
              (Node{kind: NodeKind::Identifier{symbol: String::from("uptime"), childs: vec![]}}, Proventus{value: Fructa::Inventarii(uptime), id:-5}),
            ]
          ), id: -5}),
          (Node{kind: NodeKind::Identifier{symbol: String::from("hardware"), childs: vec![]}}, Proventus{value: Fructa::Causor(
            vec![
              (Node{kind: NodeKind::Identifier{symbol: String::from("cpu"), childs: vec![]}}, Proventus{value: Fructa::Inventarii(cpumodel), id:-5}),
            ]
          ), id: -5}),

        ]
      ), id:-5}
}



pub fn declare_builtins(env: &mut Environment) {
  let functions = vec![
    (String::from("get"), get as fn(Arguments) -> Proventus), (String::from("print"), print), (String::from("println"), println),
    (String::from("fmap"), fmap), (String::from("join"), join), (String::from("return"), returnfn), (String::from("data"), data),
    (String::from("type_of"), type_of), (String::from("take"), take), (String::from("length"), length), (String::from("head"), head),
    (String::from("tail"), tail), (String::from("replace"), replace), (String::from("split"), split), (String::from("toInt"), to_int),
    (String::from("toString"), to_string), (String::from("globals"), globals), (String::from("read_file"), read_file), (String::from("load_file"), load_file),
    (String::from("load_string"), load_string), (String::from("env"), envf),
  ];
  for i in functions {
    declare_fn(i.0, i.1, env);
  }
}



#[derive(Debug)]
pub struct Arguments {
  pub function: FunctionArgs,
}
/*
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
  //read_file(Proventus), 
  length(Proventus),                                    // (list)
  take(Proventus, Proventus),                           // (amount, list)
  headTail(Proventus),                                  // (list),
  replace(Proventus, Proventus, Proventus),             // (list,list,list)
  split(Proventus, Proventus),                          // (list, list) - splitter, list
}
*/
#[derive(Debug)]
pub enum FunctionArgs {
  zerum(),
  single(Proventus),
  double(Proventus, Proventus),
  triple(Proventus, Proventus, Proventus),
  many(Vec<Proventus>),
  fmap(Proventus, Proventus, Environment, Interpreter),
  load_file(Proventus, Environment, Interpreter),
}
