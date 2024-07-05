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


pub struct Compiler {pub stack_size: i32}


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
        ANode{kind: ANodeKind::BinaryExpression(Box::new(self.transform(*left, env)), Box::new(self.transform(*right, env)), operator), id: -1},


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
    let mut result = ANode{kind: ANodeKind::Nullus, id: -2};
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
    result
  }

  pub fn code_gen(&mut self, ast: AAST, file: String, env: &mut Environment) -> String {
    let mut result = format!("bits 64\nglobal _start\n_start:\n");
    let mut indent = 2;
    for i in ast.body {
      println!("{:#?}", i.clone());
      let n = &self.parse_node(i, &mut result, indent, env);
      result += n;
    }
    result
  }

  pub fn inject_label(&mut self, current: String, label: String, indent: i32) -> String {
    format!("{}{label}_start:\n{}", current.split("_start:\n").collect::<Vec<&str>>()[0], current.split("_start:\n").collect::<Vec<&str>>()[1]).to_string()
  }
  pub fn parse_node(&mut self, node: ANode, current: &mut String, indent: i32, env: &mut Environment) -> String {
    let ev_indent = mul_string(indent, String::from(" "));
    //println!("\n\n{:#?}\n```{}```\n\n", node, current);
    match node.kind {
      ANodeKind::NumericLiteral(value) => {
        format!("{ev_indent}mov rax, {}", value.to_string())
      },


      ANodeKind::Moenus(args, identifier) => {
        //println!("{:#?}", env);
        let location = env.get(identifier);
        //panic!("{:#?}", location)

        format!("{ev_indent}push QWORD[rsp + {}]\n{ev_indent}pop rax", 8*(self.stack_size-location[0].id-1))
      }
      ANodeKind::BuiltIn(fun) => {
        match fun {
          AFunctions::returnfn(value) => 
              format!("{}\n{ev_indent}push rax\n{ev_indent}pop rdi\n{ev_indent}mov rax, 60\n{ev_indent}syscall\n", self.parse_node(*value, current, indent, env))
        }
      },
      ANodeKind::FunctionDeclaration(id, statement) => {
        let raw = match id.clone().kind {
          AST::NodeKind::Identifier{symbol, ..} => symbol,
          _ => panic!("how the fucking fuck")
        };
        let temp = self.parse_node(*statement, current, indent, env);
        *current = self.inject_label(current.to_string(), format!("{raw}:\n{}\n{ev_indent}ret\n", temp), indent);


        
        format!("{ev_indent}call {raw}\n{ev_indent}push rax\n")
      }
      ANodeKind::BinaryExpression(left,right,operator) => {
        match operator {
          Operator::Addition => {
              let mut result = String::new();
              result += &format!("{}\n{ev_indent}push rax\n", self.parse_node(*left.clone(), current, indent, env));
              self.stack_size+=1;
              result += &format!("{}\n{ev_indent}push rax\n", self.parse_node(*right.clone(), current, indent, env));
              self.stack_size+=1;
              result += &format!("{ev_indent}pop rbx\n{ev_indent}pop rax\n");
              self.stack_size-=2;
              result += &format!("{ev_indent}add rax, rbx");
              result

              //format!("{}\n{ev_indent}push rax\n{}\n{ev_indent}push rax\n{ev_indent}pop rax\n{ev_indent}pop rbx\n{ev_indent}add rax, rbx\n", self.parse_node(*left, current, indent, env), self.parse_node(*right, current, indent, env))
          },
          Operator::Substraction => {
            let mut result = String::new();
            result += &format!("{}\n{ev_indent}push rax\n", self.parse_node(*left.clone(), current, indent, env));
            self.stack_size+=1;
            result += &format!("{}\n{ev_indent}push rax\n", self.parse_node(*right.clone(), current, indent, env));
            self.stack_size+=1;
            result += &format!("{ev_indent}pop rbx\n{ev_indent}pop rax\n");
            self.stack_size-=2;
            result += &format!("{ev_indent}sub rax, rbx");
            result
          },
          Operator::Multiplication => {
            let mut result = String::new();
            result += &format!("{}\n{ev_indent}push rax\n", self.parse_node(*left.clone(), current, indent, env));
            self.stack_size+=1;
            result += &format!("{}\n{ev_indent}push rax\n", self.parse_node(*right.clone(), current, indent, env));
            self.stack_size+=1;
            result += &format!("{ev_indent}pop rbx\n{ev_indent}pop rax\n");
            self.stack_size-=2;
            result += &format!("{ev_indent}mul rbx");
            result 
          },
          Operator::Division => {
            let mut result = String::new();
            result += &format!("{}\n{ev_indent}push rax\n", self.parse_node(*left.clone(), current, indent, env));
            self.stack_size+=1;
            result += &format!("{}\n{ev_indent}push rax\n", self.parse_node(*right.clone(), current, indent, env));
            self.stack_size+=1;
            result += &format!("{ev_indent}pop rbx\n{ev_indent}pop rax\n");
            self.stack_size-=2;
            result += &format!("{ev_indent}div rbx");
            result 
          },
          _ => panic!("operator wweird")
        }
      }
      ANodeKind::Nullus => String::new(),
      _ => panic!("node: {:#?}", node)
    }
    
  }
  

  pub fn compile(&mut self, program: AST::Node, env: &mut Environment, file: String) -> String {
    let transformed = self.transform_program(program, env);
    //println!("{:#?} and {:#?}", transformed.clone(), env);
    let code = self.code_gen(transformed, file, env);
    code
    //AST::Proventus{value: AST::Fructa::Nullus, id: -2}
  }
}
