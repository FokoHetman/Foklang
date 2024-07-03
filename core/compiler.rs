use core::{AST, asm_env::Environment, tokenizer::Operator};

#[derive(Debug,Clone)]
enum ABuiltins {
  returnfn
}

#[derive(Debug, Clone)]
enum AFunctions {
  returnfn(Box<ANode>)
}

pub fn declare_builtin(id: String, builtin: ABuiltins, env: &mut Environment) {
  env.declare(AST::Node{kind: AST::NodeKind::Identifier {symbol: id, childs:vec![]}},
    ANode{kind: ANodeKind::BuiltInDefinition(builtin), id: -3}
  );
}

pub fn declare_builtins(env: &mut Environment) {
  declare_builtin(String::from("return"), ABuiltins::returnfn, env)
}




#[derive(Debug, Clone)]
enum ANodeKind {
  Program(Vec<Box<ANode>>),
  BinaryExpression(Box<ANode>, Box<ANode>, Operator),
  NumericLiteral(i32),
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
fn extract_value(a: AST::Node) -> i32 {
  match a.kind {
    AST::NodeKind::NumericLiteral{value} => match value { AST::NodeValue::Integer(i) => i, _ => panic!("gwuh2") },
    _ => panic!("gwuh")
  }
}

pub struct Compiler {pub stack_size: i32, pub stack_loc: i32}


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
    aast.body.push(ANode{kind: ANodeKind::BuiltIn(AFunctions::returnfn(Box::new(ANode{kind: ANodeKind::NumericLiteral(0), id: -1}))),id:-1});
    aast
  }



  pub fn transform(&mut self, node: AST::Node, env: &mut Environment) -> ANode {
    match node.kind {

      AST::NodeKind::BinaryExpression{left,right,operator,..} => 
        ANode{kind: ANodeKind::BinaryExpression(Box::new(ANode{kind: ANodeKind::NumericLiteral(extract_value(*left)), id:-1}), Box::new(ANode{id: -1, kind: ANodeKind::NumericLiteral(extract_value(*right))}), operator), id: -1},


      AST::NodeKind::NumericLiteral{value} =>
        ANode{kind: ANodeKind::NumericLiteral(match value { AST::NodeValue::Integer(i) => i, _ => panic!("??")}), id:-1},
      
      AST::NodeKind::Identifier{..} => self.transform_identifier(node, env),
      AST::NodeKind::FunctionDeclaration{..} => self.transform_function(node, env),
      _ => panic!("not impl")
    }
  }

  pub fn transform_function(&mut self, node: AST::Node, env: &mut Environment) -> ANode {
    match node.kind  {
      AST::NodeKind::FunctionDeclaration{ ref identifier,ref statement} => {
        let mut unboxed_args = Vec::<AST::Node>::new();
        match identifier.kind {
          AST::NodeKind::Identifier{symbol:_, ref childs} => {
            for i in childs {
              unboxed_args.push(*i.clone());
            }
          }
          _ => panic!("no idea how no idea why but fuck you")
        };
        env.declare(*identifier.clone(), ANode{kind: ANodeKind::Moenus(unboxed_args, *statement.clone()), id:-1});
        ANode{kind: ANodeKind::FunctionDeclaration(*identifier.clone(), Box::new(self.transform(*statement.clone(), env))), id: -2}
      }
      _ => panic!("behind you")
    }
  }

  pub fn transform_identifier(&mut self, node: AST::Node, env: &mut Environment) -> ANode {
    let mut result = ANode{kind: ANodeKind::Nullus, id: -2};
    'ma: for variation in env.get(node.clone()) {
      match variation.kind {


        ANodeKind::Moenus(args, statement) => {
          if args.len()==0 {
            
            result = ANode{kind: ANodeKind::Moenus(vec![], statement), id:-1};
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
    result
  }

  pub fn code_gen(&mut self, ast: AAST, file: String) -> String {
    let mut result = format!("bits 64\nglobal _start\n_start:\n");
    let mut indent = 2;
    for i in ast.body {
      let n = &self.parse_node(i, &mut result, indent);
      result += n;
    }
    result
  }

  pub fn inject_label(&mut self, current: String, label: String, indent: i32) -> String {
    format!("{}{label}_start:\n{}", current.split("_start:\n").collect::<Vec<&str>>()[0], current.split("_start:\n").collect::<Vec<&str>>()[1]).to_string()
  }
  pub fn parse_node(&mut self, node: ANode, current: &mut String, indent: i32) -> String {
    let ev_indent = mul_string(indent, String::from(" "));
    match node.kind {
      ANodeKind::NumericLiteral(value) => {
        format!("{ev_indent}mov rdi, {}", value.to_string())
      },


      ANodeKind::Moenus(args, identifier) => {
        format!("{ev_indent}pop rdi") //TODO - find place in stack of identifier (dw, in rust not assembly)
      }
      ANodeKind::BuiltIn(fun) => {
        match fun {
          AFunctions::returnfn(value) => 
              format!("{ev_indent}mov rax, 60\n{}\n{ev_indent}syscall\n", self.parse_node(*value, current, indent))
        }
      },
      ANodeKind::FunctionDeclaration(id, statement) => {
        let raw = match id.kind {
          AST::NodeKind::Identifier{symbol, ..} => symbol,
          _ => panic!("how the fucking fuck")
        };
        let temp = self.parse_node(*statement, current, indent);
        *current = self.inject_label(current.to_string(), format!("{raw}:\n{}\n{ev_indent}ret\n", temp), indent);
        self.stack_size+=1;
        format!("{ev_indent}call {raw}\n{ev_indent}push rdi\n")
      }
      /*ANodeKind::BinaryExpression(left,right,operator) => {
        match operator {
          Operator::Addition => {
            format!("mov rdi, {}\nmov rdx, {}\nadd rdi, rdx\n", self.parse_node(*left), self.parse_node(*right))
          },
          Operator::Substraction => {
            format!("sub {}, {}", self.parse_node(*left), self.parse_node(*right))
          },
          Operator::Multiplication => {
            format!("mul {}, {}", self.parse_node(*left), self.parse_node(*right))
          },
          Operator::Division => {
            format!("div {}, {}", self.parse_node(*left), self.parse_node(*right))
          },
          _ => panic!("operator wweird")
        }
      }*/
      ANodeKind::Nullus => String::new(),
      _ => panic!("node: {:#?}", node)
    }
  }
  

  pub fn compile(&mut self, program: AST::Node, env: &mut Environment, file: String) -> String {
    let transformed = self.transform_program(program, env);
    println!("{:#?} and {:#?}", transformed.clone(), env);
    let code = self.code_gen(transformed, file);
    code
    //AST::Proventus{value: AST::Fructa::Nullus, id: -2}
  }
}
