use core::AST;
use core::error_handler::ErrorHandler;
use core::tokenizer::{Operator};
use std::convert::TryInto;
use core::env::Environment;
use core::builtins;


#[derive(Debug,Clone)]
pub struct Interpreter {pub error_handler:ErrorHandler}

impl Interpreter {
  fn evaluate_program(&mut self, program: AST::Node, env: &mut Environment) -> AST::Proventus {
    let mut last_eval = AST::Proventus{..Default::default()};

    match program.kind {
      AST::NodeKind::Program{body:body,id:id} => {
        env.current_node = 0;
        let mut counter = 0;
        env.node_stack = [].to_vec();
        env.push_args(body.clone());
        for i in body {
          if counter==env.current_node {
            last_eval = self.evaluate(*i, env);
            env.current_node+=1;
          }
          counter+=1;
        }
        last_eval.id = id;
      }
      _ => panic!("[Interpreter Error] Tried to evaluate non-Program Node as a Program, {:#?}", program)
    }
    return last_eval
  }
  pub fn evaluate(&mut self, node: AST::Node, env: &mut Environment) -> AST::Proventus {
    match node.kind {
      AST::NodeKind::Program{body:_,id:_} => self.evaluate_program(node, env),
      AST::NodeKind::NullLiteral{value:_} => AST::Proventus{value: AST::Fructa::Nullus, id: -1},
      AST::NodeKind::NumericLiteral{value:i} => AST::Proventus{value: AST::Fructa::Numerum(match i {AST::NodeValue::Integer(i) => i, _ => 0}), id: -1},
      AST::NodeKind::BinaryExpression{left:_,right:_,operator:_} => self.evaluate_binary_expression(node, env),
      AST::NodeKind::Identifier{symbol:_, ..} => self.evaluate_identifier(node, env),
      AST::NodeKind::List{..} => self.evaluate_list(node, env),
      AST::NodeKind::Config{arguments:_} => self.evaluate_object(node, env),
      AST::NodeKind::FunctionDeclaration{identifier: _, statement: _} => self.evaluate_function(node, env),
      AST::NodeKind::Char{..} => self.evaluate_char(node, env),
      AST::NodeKind::Bool{..} => self.evaluate_bool(node, env),
      _ => panic!("{} {:#?}", self.error_handler.interpreter("unknown_node").error_msg, node)
    }
  }
  fn evaluate_bool(&mut self, node: AST::Node, env: &mut Environment) -> AST::Proventus {
    match node.kind {
      AST::NodeKind::Bool{value} => {
        AST::Proventus{value: AST::Fructa::Condicio(match value {AST::NodeValue::Bool(b) => b, _ => false}), id: -1}
      }
      _ => panic!("false && true * 1 _2@4930refdjigbkc")
    }
  }
  fn evaluate_char(&mut self, node: AST::Node, env: &mut Environment) -> AST::Proventus {
    match node.kind {
      AST::NodeKind::Char{value} => {
        AST::Proventus{value: AST::Fructa::Ustulo(match value { AST::NodeValue::Char(c) => c, _ => 'h'}), id: -1}
      }
      _ => panic!("huh?")
    }
  }
  fn evaluate_list(&mut self, node: AST::Node, env: &mut Environment) -> AST::Proventus {
    match node.kind {
      AST::NodeKind::List{body} => {
        let mut args: Vec<AST::Proventus> = vec![];
        for i in body {
          if self.evaluate(*i.clone(), env).id==112 {
            match self.evaluate(*i.clone(), env).value {
              AST::Fructa::Inventarii(inv) => {
                for x in inv {
                  args.push(x);
                }
              }
              _ => panic!("112 - toParent call for non-Inventarii object")
            }
          } else {
            args.push(self.evaluate(*i.clone(), env));
          }
        }
        AST::Proventus{value: AST::Fructa::Inventarii(args), id: -1}
        
      }
      _ => panic!("A")
    }

  }
  fn evaluate_function(&mut self, node: AST::Node, env: &mut Environment) -> AST::Proventus {
    match node.kind {
      AST::NodeKind::FunctionDeclaration{identifier: identifier, statement: statement} => {
        let mut unboxed_args = Vec::<AST::Node>::new();
        match identifier.kind {
          AST::NodeKind::Identifier{symbol:_, ref childs} => {
            for i in childs {
              unboxed_args.push(*i.clone());
            }
          }
          _ => panic!("??")
        };
        env.declare(*identifier, AST::Proventus{value: AST::Fructa::Moenus(unboxed_args, *statement),id:-1});
        AST::Proventus{value: AST::Fructa::Nullus, id: -1}
      }
      _ => panic!("{} {:#?}", self.error_handler.interpreter("nonfunctiondeclaration_node").error_msg, node)

    }

  }
  fn evaluate_binary_expression(&mut self, node: AST::Node, env: &mut Environment) -> AST::Proventus {
    match node.kind {
      AST::NodeKind::BinaryExpression{left: node_left,right: node_right,operator: node_operator} => {
        /*let left = match self.evaluate(*node_left, env).value {
          AST::Fructa::Numerum(i) => i,
          _ => panic!("hi I fucked up yipee!")
        };
        let right = match self.evaluate(*node_right, env).value {
          AST::Fructa::Numerum(i) => i,
          _ => panic!("hi I fucked up big yipee!")
        };*/

        /*if self.error_handler.check_binary_expression(left,right).bool {
          panic!("[Interpreter Error] Binary Expression: {:#?}", self.error_handler.check_binary_expression(left,right).error_msg);
        };*/
        match node_operator {
          Operator::Addition => match self.evaluate(*node_left, env).value {
            AST::Fructa::Numerum(i) => {
              match self.evaluate(*node_right, env).value {
                AST::Fructa::Numerum(i2) => {
                  AST::Proventus{value: AST::Fructa::Numerum(i+i2), id: -1}
                }
                _ => panic!("Addition of l and r not implemented"),
              }
            },
            _ => panic!("Addition of l and r not implemented"),
          },


          Operator::Substraction => match self.evaluate(*node_left, env).value {
            AST::Fructa::Numerum(i) => {
              match self.evaluate(*node_right, env).value {
                AST::Fructa::Numerum(i2) => {
                  AST::Proventus{value: AST::Fructa::Numerum(i-i2), id: -1}
                }
                _ => panic!("Substraction imbluedabudidabudaj")
              }
            },
            _ => panic!("Substraction of l and r not implemented"),
          },


          Operator::Multiplication => match self.evaluate(*node_left, env).value {
            AST::Fructa::Numerum(i) => {
              match self.evaluate(*node_right, env).value {
                AST::Fructa::Numerum(i2) => {
                  AST::Proventus{value: AST::Fructa::Numerum(i*i2), id: -1}
                }
                _ => panic!("Multiplication efsduixcvjjuodvcf")
              }
            },
            _ => panic!("Multiplication of l and r not implemented"),
          },
          Operator::Division => match self.evaluate(*node_left, env).value {
            AST::Fructa::Numerum(i) => {
              match self.evaluate(*node_right, env).value {
                AST::Fructa::Numerum(i2) => {
                  if i2==0 {//error handle it i beg
                    panic!("Division by zero") 
                  }
                  AST::Proventus{value: AST::Fructa::Numerum(i/i2), id: -1}
                }
                _ => panic!("Division cyka blyat")
              }
            },
            _ => panic!("Division of l and r not implemented")
          },
          Operator::Exponentiation => match self.evaluate(*node_left, env).value {
            AST::Fructa::Numerum(i) => {
              match self.evaluate(*node_right, env).value {
                AST::Fructa::Numerum(i2) => {
                  AST::Proventus{value: AST::Fructa::Numerum(i.pow(i2.try_into().unwrap())), id: -1}
                }
                _ => panic!("Exponentiation stalinium")
              }
            }
            _ => panic!("Exponentiation of l and r not implemented")
          },
          Operator::Comparision => match self.evaluate(*node_left, env).value {
            AST::Fructa::Numerum(i) => {
              match self.evaluate(*node_right, env).value {
                AST::Fructa::Numerum(i2) => {
                  AST::Proventus{value: AST::Fructa::Condicio(i==i2), id: -1}
                }
                _  => panic!("huh?")
              }
            },
            AST::Fructa::Inventarii(li) => {
              match self.evaluate(*node_right, env).value {
                AST::Fructa::Inventarii(li2) => {
                  for i in 0..li.len() {
                    if li2[i as usize]!=li[i as usize] {
                      return AST::Proventus{value: AST::Fructa::Condicio(false), id: -1}
                    }
                  }
                  AST::Proventus{value: AST::Fructa::Condicio(true), id: -1}
                }
                _ => panic!("death")
              }
            },
            _ => panic!("Comparision died")
          },
          Operator::Greater => match self.evaluate(*node_left, env).value {
            AST::Fructa::Numerum(i) => {
              match self.evaluate(*node_right, env).value {
                AST::Fructa::Numerum(i2) => {
                  AST::Proventus{value: AST::Fructa::Condicio(i>i2), id: -1}
                },
                _ => panic!("heat deth of the unvirser!")
              }
            }
            _ => panic!("heat deth of teh univers the sequel!")
          },
          Operator::Lower => match self.evaluate(*node_left, env).value {
            AST::Fructa::Numerum(i) => {
              match self.evaluate(*node_right, env).value {
                AST::Fructa::Numerum(i2) => {
                  AST::Proventus{value: AST::Fructa::Condicio(i<i2), id: -1}
                },
                _ => panic!("gravitational wave did it not me I swear")
              }
            }
            _ => panic!("another gravitational wave!!!")
          },

          Operator::GreaterEqual => match self.evaluate(*node_left, env).value {
            AST::Fructa::Numerum(i) => {
              match self.evaluate(*node_right, env).value {
                AST::Fructa::Numerum(i2) => {
                  AST::Proventus{value: AST::Fructa::Condicio(i>=i2), id: -1}
                },
                _ => panic!("heat deth of the unvirser!")
              }
            }
            _ => panic!("heat deth of teh univers the sequel!")
          },

          Operator::LowerEqual => match self.evaluate(*node_left, env).value {
            AST::Fructa::Numerum(i) => {
              match self.evaluate(*node_right, env).value {
                AST::Fructa::Numerum(i2) => {
                  AST::Proventus{value: AST::Fructa::Condicio(i<=i2), id: -1}
                },
                _ => panic!("gravitational wave did it not me I swear")
              }
            }
            _ => panic!("another gravitational wave!!!")
          },
          Operator::DoubleDot => match self.evaluate(*node_left, env).value {
            AST::Fructa::Numerum(i) => {
              match self.evaluate(*node_right, env).value {
                AST::Fructa::Numerum(i2) => {
                  let mut li: Vec<AST::Proventus> = vec![];
                  if i>i2 {
                    for x in (i2..i+1).rev() {
                      li.push(AST::Proventus{value: AST::Fructa::Numerum(x), id: -1});
                    }
                  } else {
                    for x in i..i2+1 {
                      li.push(AST::Proventus{value: AST::Fructa::Numerum(x), id: -1});
                    }
                  }
                  AST::Proventus{value: AST::Fructa::Inventarii(li), id: 112}
                },
                AST::Fructa::Nullus =>  {
                  let mut li: Vec<AST::Proventus> = vec![];
                  for x in (i..i32::MAX) {
                    li.push(AST::Proventus{value: AST::Fructa::Numerum(x), id: -1});
                  }
                  AST::Proventus{value: AST::Fructa::Inventarii(li), id: 112}
                },
                _ => panic!("gravitational wave happen godmdmdandd")
              }
            }
            _ => panic!("ranges not implemented for non-Numerum values")
          },

          _ => panic!("[Interpreter Error] Unknown Operator: {:#?}", node_operator)
        }

        //AST::Proventus{value: AST::Fructa::Numerum(result), id: -1}
      }
      _ => panic!("[Interpreter Error] Tried to evaluate non-BinaryExpression Node as BinaryExpression, {:#?}", node)
    }
  }
  fn evaluate_identifier(&mut self, node: AST::Node, env: &mut Environment) -> AST::Proventus {
    let mut result = AST::Proventus{value: AST::Fructa::Nullus, id: -2};
    //println!("{:#?}", env.get(node.clone()));
    'ma: for variation in env.get(node.clone()) {
      //println!("{:#?}", variation);
      match variation.value {
        AST::Fructa::Moenus(args, statement) => {
          let mut function_env = Environment{parent: Some(Box::new(env.clone())), error_handler: self.error_handler, ..Default::default()};

          match node.kind {
            AST::NodeKind::Identifier{ref symbol, ref childs} => {
              for i in 0..args.len() {
                //let evaluated = self.evaluate(env.node_stack[env.current_node as usize+i+1].clone(), env);
                let userdata = match self.evaluate(*childs[i].clone(), env).value {AST::Fructa::Numerum(i) => i, _ => 0};
                //println!("{:#?} != {:#?} => {:#?}", args[i].clone(), self.evaluate(*childs[i].clone(), env), match args[i].clone().kind { AST::NodeKind::NumericLiteral{value:i, ..} => match i { AST::NodeValue::Integer(i) => i, _ => userdata}, _ => userdata} != userdata);
                if match args[i].clone().kind { AST::NodeKind::NumericLiteral{value: i, ..} => match i { AST::NodeValue::Integer(i) => i, _ => userdata}, _ => userdata} != userdata {
                  //println!("continuing..");
                  continue 'ma
                }
                match args[i].clone().kind {
                  AST::NodeKind::Identifier{..} => {function_env.declare(args[i].clone(), self.evaluate(*childs[i].clone(), env));},
                  AST::NodeKind::NumericLiteral{value,..} =>  {},
                  _ => {}
                };
                //self.evaluate();
              //get further nodes, and assing them to real arguments (eg. x = 5)
              }
            }
            _ => {}
          }
          env.current_node+=args.len() as i32;
          //println!("FUNCTION_ENV: {:#?}", function_env);
          result = self.evaluate(statement.clone(), &mut function_env);
          //println!("R, STATEMENT: {:#?}, {:#?}", result, statement);
          break
          //evaluate the statement, with defined x and y
        }
      AST::Fructa::BuiltIn(f) => {
        //if f==core::builtins::get {
        let mut args_vec: Vec<AST::Node> = vec![];
        match node.kind {
          AST::NodeKind::Identifier{ref symbol, ref childs} => {
            for i in childs {
              args_vec.push(*i.clone());
            }
          }
          _ => panic!("")
        }
          /*let val = self.evaluate(env.node_stack[env.current_node as usize+1].clone(), env);
          let val2 = match env.node_stack[env.current_node as usize+2].clone().kind {
            AST::NodeKind::Identifier{symbol, ..} => {
              AST::Proventus{value: AST::Fructa::Filum(symbol), id: -1}
            }
            _ => panic!("")
          };*/
        let mut fargs = builtins::FunctionArgs::zerum();
        if f==builtins::print {
          let mut n_args: Vec<AST::Proventus> = vec![];
          for i in args_vec {
            //parse types!!
            n_args.push(self.evaluate(i, env));
          }
          fargs = builtins::FunctionArgs::print(n_args);
        }
        else if f==builtins::get {
          fargs = builtins::FunctionArgs::get(self.evaluate(args_vec[0].clone(), env), 
                AST::Proventus{value: match args_vec[1].clone().kind { AST::NodeKind::Identifier{symbol,..} => AST::Fructa::Filum(symbol),
                AST::NodeKind::NumericLiteral{value} => match value { AST::NodeValue::Integer(i) => AST::Fructa::Numerum(i), _ => AST::Fructa::Numerum(0)}, 
                _ => AST::Fructa::Nullus}, id: -1});
        }
        else if f==builtins::println {
          let mut n_args: Vec<AST::Proventus> = vec![];
          for i in args_vec {
            n_args.push(self.evaluate(i, env));
          }
          fargs = builtins::FunctionArgs::print(n_args);

        }
        else if f==builtins::fmap {
          fargs = builtins::FunctionArgs::fmap(args_vec[0].clone(), self.evaluate(args_vec[1].clone(), env), env.clone(), self.clone());
        }
        else if f==builtins::join {
          let mut n_args: Vec<AST::Proventus> = vec![];
          for i in args_vec {
            n_args.push(self.evaluate(i, env));
          }
          fargs = builtins::FunctionArgs::join(n_args);
        }
        let args = builtins::Arguments{function: fargs};
        result = f(args)
          //panic!("builtin")
        /*} else if f==core::builtins::print {
          

        }*/
      }
      AST::Fructa::Inventarii(i) => { result = AST::Proventus {value: AST::Fructa::Inventarii(i), id: -1}},
      AST::Fructa::Numerum(i) => { result = AST::Proventus {value: AST::Fructa::Numerum(i), id: -1}},
      _ => panic!("damn")
      }
    }
    result
  }
  fn evaluate_object(&mut self, node: AST::Node, env: &mut Environment) -> AST::Proventus {
    match node.kind {
      AST::NodeKind::Config{arguments} => {
        let mut args: Vec<(AST::Node, AST::Proventus)> = vec![];
        for i in arguments {
          args.push((*i.0.clone(), self.evaluate(*i.1.clone(), env)));
        }
        AST::Proventus{value: AST::Fructa::Causor(args), id: -1}

      }
      _ => panic!("evaluation non-object as object damn")
    }
  }
}
