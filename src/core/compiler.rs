use std::fmt::Display;

use crate::core::{AST, asm_env::Environment, tokenizer::Operator};

#[derive(Debug,Clone)]
pub enum ABuiltins {
  returnfn,
  print,
}

#[derive(Debug,Clone)]
pub enum AType {
  Int,
  String,
}
impl Display for AType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", match self {
      AType::Int => "int",
      AType::String => "char*"
    })
  }
}


#[derive(Debug, Clone)]
enum AFunctions {
  returnfn(Box<ANode>),
}

pub fn declare_builtin(id: String, builtin: ABuiltins, env: &mut Environment) {
  env.declare(AST::Node{kind: AST::NodeKind::Identifier {symbol: id, childs:vec![]}},
    ANode{kind: ANodeKind::BuiltInDefinition(builtin), id: -3}
  );
}

pub fn declare_builtins(env: &mut Environment) {
  declare_builtin(String::from("return"), ABuiltins::returnfn, env);
  declare_builtin(String::from("print"), ABuiltins::print, env);
}




#[derive(Debug, Clone)]
enum ANodeKind {
  Program(Vec<Box<ANode>>),
  BinaryExpression(Box<ANode>, Box<ANode>, Operator),
  NumericLiteral(i32),
  Identifier(String, Vec<ANode>),
  Moenus(Vec<AST::Node>, AST::Node),

  FunctionDeclaration(AST::Node, Box<ANode>),
  BuiltInDefinition(ABuiltins),
  BuiltIn(AFunctions),
  Nullus,
}


#[derive(Debug, Clone)]
pub struct ANode {
  kind: ANodeKind,
  id: i32,
}

#[derive(Debug, Clone)]
pub struct AAST {body: Vec<ANode>} // Assembly AST










fn mul_string(times: i32, string: String) -> String {
  let mut result = String::new();
  for i in 0..times {
    result.push_str(&string);
  }
  result
}

pub enum Language {
  Assembly,
  C,
}

pub struct Compiler {pub stack_size: i32, pub language: Language}


impl Compiler {
  pub fn transform_program(&mut self, program: AST::Node, env: &mut Environment) -> AAST {

    let mut aast = AAST{body: vec![]};

    match program.kind {
      AST::NodeKind::Program{body,..} => {
        for i in body {
          aast.body.push(self.transform(*i, env));
        }
      }
      _ => panic!("why that node dumbass compiler")
    }
    //aast.body.push(ANode{kind: ANodeKind::BuiltIn(AFunctions::returnfn(Box::new(ANode{kind: ANodeKind::NumericLiteral(0), id: -1}))),id:-1});
    aast
  }



  pub fn transform(&mut self, node: AST::Node, env: &mut Environment) -> ANode {
    match node.kind {

      AST::NodeKind::BinaryExpression{left,right,operator,..} => 
        ANode{kind: ANodeKind::BinaryExpression(Box::new(self.transform(*left, env)), Box::new(self.transform(*right, env)), operator), id: -1},


      AST::NodeKind::NumericLiteral{value} =>
        ANode{kind: ANodeKind::NumericLiteral(match value { AST::NodeValue::Integer(i) => i, _ => panic!("??")}), id:-1},
      
      AST::NodeKind::Identifier{..} => self.transform_identifier(node, env),
      AST::NodeKind::FunctionDeclaration{..} => self.transform_function(node, env),
      
      AST::NodeKind::NullLiteral { .. } => ANode { kind: ANodeKind::Nullus, id: -1 },

      _ => panic!("Transform not implemented for node: {:#?}", node),
    }
  }

  pub fn transform_function(&mut self, node: AST::Node, env: &mut Environment) -> ANode {
    match node.kind  {
      AST::NodeKind::FunctionDeclaration{ ref identifier,ref statement} => {
        
        let mut unboxed_args = Vec::<AST::Node>::new();
        match identifier.clone().kind {
          AST::NodeKind::Identifier{symbol:_, ref childs} => {
            for i in childs {
              unboxed_args.push(*i.clone());
            }
          }
          _ => panic!("no idea how no idea why but fuck you")
        };

        env.declare(*identifier.clone(), ANode{kind: ANodeKind::Moenus(unboxed_args, *identifier.clone()), id:self.stack_size});
        self.stack_size+=1;
        
        ANode{kind: ANodeKind::FunctionDeclaration(*identifier.clone(), Box::new(self.transform(*statement.clone(), env))), id: -2}
      }
      _ => panic!("behind you")
    }
  }

  pub fn transform_identifier(&mut self, node: AST::Node, env: &mut Environment) -> ANode {
    ANode{kind: ANodeKind::Identifier(match node.kind {AST::NodeKind::Identifier{ref symbol, ..} => symbol.clone(), _ => panic!("how")}, match node.kind {AST::NodeKind::Identifier{childs, ..} => 
        {
          let mut result: Vec<ANode> = vec![];
          for i in childs {
            result.push(self.transform(*i.clone(), env));
          }
          result
        }, _ => panic!("how")}), id: -1}
    /*let mut result = ANode{kind: ANodeKind::Nullus, id: -2};
    'ma: for variation in env.get(node.clone()) {
      match variation.kind {


        ANodeKind::Moenus(args, _) => {
          if args.len()==0 {
            
            result = ANode{kind: ANodeKind::Moenus(vec![], node.clone()), id:-1};
          }

        }

        ANodeKind::BuiltInDefinition(fun) => {
          let mut args_vec: Vec<AST::Node> = vec![];
          match node.kind {
            AST::NodeKind::Identifier{ref symbol, ref childs} => {
              for i in childs {
                args_vec.push(*i.clone());
              }
            }
            _ => panic!("??")
          }


          result = match fun {
            ABuiltins::returnfn => ANode{kind: ANodeKind::BuiltIn(AFunctions::returnfn(Box::new(self.transform(args_vec[0].clone(), env)))), id: -1}
          }
        }
        _ => panic!("no impl/todo/I'llneverdoitprobably")
      }
    }
    result*/
  }

  pub fn code_gen(&mut self, ast: AAST, file: String, env: &mut Environment) -> String {
    match self.language {
      Language::Assembly => {
        let mut result = format!("bits 64\nglobal _start\n_start:\n");
        let mut indent = 2;
        for i in ast.body {
          println!("{:#?}", i.clone());
          //let n = &self.parse_asm_node(i, &mut result, indent, env);
          //result += n;
        }
        result
      },
      Language::C => {
        let mut result = format!("#include<stdio.h>\n//FUNCTIONS\n\n//END FUNCTIONS\n\nint main() {{");
        let mut indent = 2;
        for i in ast.body {
          println!("{:#?}", i.clone());
          let n = &self.parse_c_node(i, &mut result, indent, env);
          result += n;
        }
        result + ";}\n"
      },
    }
  }
  pub fn parse_c_node(&mut self, node: ANode, current: &mut String, indent: i32, env: &mut Environment) -> String {
    let ev_indent = mul_string(indent, String::from(" "));
    //println!("\n\n{:#?}\n```{}```\n\n", node, current);
    match node.kind {
      ANodeKind::NumericLiteral(value) => {
        //format!("{}", value)
        format!("{}", value.to_string())
      },


      ANodeKind::Moenus(args, identifier) => {
        //println!("{:#?}", env);
        let location = env.get(identifier.clone());
        //panic!("{:#?}", location)

        let mut call = format!("{id}(", id=match identifier.kind {AST::NodeKind::Identifier{symbol, ..} => symbol, _ => panic!("fok")});//, 8*(self.stack_size-location[0].id-1))
        for i in args {
          let transformed = self.transform(i, env);
          call += &self.parse_c_node(transformed, current, indent, env);
          call += ",";
        }
        if call.ends_with(",") {
          let mut callc = call.chars();
          callc.next_back();
          call = callc.collect::<String>();
        }
        call + ")"
      }
      ANodeKind::BuiltIn(fun) => {
        match fun {
          AFunctions::returnfn(value) => 
              format!("return {};\n", self.parse_c_node(*value, current, indent, env))
        }
      },
      ANodeKind::FunctionDeclaration(id, statement) => {
        let (raw, childs) = match id.clone().kind {
          AST::NodeKind::Identifier{symbol, childs} => (symbol, childs),
          _ => panic!("how the foking fok")
        };

        // HERE, it is evaluated when identifier is undefined (smh);
        let mut t_env = Environment{parent: Some(Box::new(env.clone())), ..Default::default()};
        
        let value_type =
          match env.get_type(id) {
            Ok(t) => t,
            Err(_) => env.guess_type(*statement.clone())
          };

        
        if childs.len()>0 {
          let mut label = String::new();
          label += &(value_type.to_string() + " " + &raw);
          label += "(";
          for i in childs {
            let (raw, childs) = match i.clone().kind {
              AST::NodeKind::Identifier{symbol, childs} => (symbol, childs),
              _ => panic!("how the foking fok")
            };
            let value_type =
              match env.get_type(*i) {
                Ok(t) => t,
                Err(_) => env.guess_type(*statement.clone())
              };
            label += &(value_type.to_string() + " " + &raw + ", ");
          }
          if label.ends_with(", ") {
            let mut labch = label.chars();
            for _ in 0..2 {labch.next_back();};
            label = labch.collect::<String>();
          }
          label += ") {\n  return ";
          label += &self.parse_c_node(*statement, current, 2, env);
          label += ";};";
          *current = self.inject_label(current.to_string(), label, 2);
          String::new()
        } else {
          format!("{value_type} {raw} = {};", self.parse_c_node(*statement, current, indent, env))
        }
      }
      ANodeKind::BinaryExpression(left,right,operator) => {
        match operator {
          Operator::Addition => {
              format!("{} + {}", self.parse_c_node(*left.clone(), current, indent, env), self.parse_c_node(*right.clone(), current, indent, env))

              //format!("{}\n{ev_indent}push rax\n{}\n{ev_indent}push rax\n{ev_indent}pop rax\n{ev_indent}pop rbx\n{ev_indent}add rax, rbx\n", self.parse_node(*left, current, indent, env), self.parse_node(*right, current, indent, env))
          },
          Operator::Substraction => {
            format!("{} - {}", self.parse_c_node(*left.clone(), current, indent, env), self.parse_c_node(*right.clone(), current, indent, env))
          },
          Operator::Multiplication => {
            format!("{} * {}", self.parse_c_node(*left.clone(), current, indent, env), self.parse_c_node(*right.clone(), current, indent, env))

          },
          Operator::Division => {
             format!("{} / {}", self.parse_c_node(*left.clone(), current, indent, env), self.parse_c_node(*right.clone(), current, indent, env))
         },
          _ => panic!("operator wweird")
        }
      }
      ANodeKind::Identifier(s, childs) => {
        if childs.len()==0 {
          s
        } else {
          let mut result = String::from("(");
          for i in childs {
            result += &(self.parse_c_node(i, current, 2, env) + ", ");
          }
          if result.ends_with(", ") {
            let mut resch = result.chars();
            for _ in 0..2 { resch.next_back(); }
            result = resch.collect();
          }
          result += ")";
          s+&result
        }
      },
      ANodeKind::Nullus => String::new(),
      _ => panic!("node: {:#?}", node)
    }
    
  }

  pub fn inject_label(&mut self, current: String, label: String, indent: i32) -> String {
    format!("{}//FUNCTIONS\n{label}{}", current.split("//FUNCTIONS\n").collect::<Vec<&str>>()[0], current.split("//FUNCTIONS\n").collect::<Vec<&str>>()[1]).to_string()
  }
    

  pub fn compile(&mut self, program: AST::Node, env: &mut Environment, file: String) -> String {
    let transformed = self.transform_program(program, env);
    println!("{:#?} and {:#?}", transformed.clone(), env);
    let code = self.code_gen(transformed, file, env);
    code
    //AST::Proventus{value: AST::Fructa::Nullus, id: -2}
  }
}
