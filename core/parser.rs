use core::AST;
use core::tokenizer::{Token, TokenValue, TokenType, Operator};
#[derive(Debug, Clone, Copy)]
pub struct Parser {}

impl Parser {
  pub fn at(self, tokens: &mut Vec<Token>) -> Token {
    return tokens[0].clone()
  }
  pub fn eat(&mut self, tokens: &mut Vec<Token>) -> Token {
    let token = tokens[0].clone();
    tokens.remove(0);
    return token
 }
  pub fn eatExpect(&mut self, expectedTokenType: TokenType, err_msg: String, tokens: &mut Vec<Token>) -> Token {
    let token = tokens[0].clone();
    tokens.remove(0);

    if token.tokentype!=expectedTokenType {
      panic!("{}", err_msg);
    }
    return token
    
  }
  pub fn eatExpectValue(&self, expectedTokenValue: TokenValue, err_msg: String, tokens: &mut Vec<Token>)-> Token {
    let token = tokens[0].clone();
    tokens.remove(0);

    if token.tokenvalue!=expectedTokenValue {
        panic!("{}",err_msg);
    }
    return token

  }
  pub fn parse(&mut self, static_tokens: Vec<Token>) -> AST::Node {
    let mut tokens = static_tokens.clone();
    let mut result = AST::Node{kind: AST::NodeKind::Program{body: Vec::<Box<AST::Node>>::new(), id: 0}};
    while tokens[0].tokentype!=TokenType::EOF {
      result.appendToBody(self.parse_stmt( &mut tokens, 1));
    }

    return result
  }

  pub fn parse_stmt(&mut self, tokens: &mut Vec<Token>, depth: i32) -> AST::Node {
    let mut left = self.parse_hstmts(tokens, depth);
    if self.at(tokens).tokenvalue == TokenValue::Operator(Operator::Equal) {
        let _ = self.eat(tokens); // get rid of `=`
        let function_id = left;
        

        let statement = self.parse_hstmts(tokens, depth); // get the function

        //println!("FUNCTION PARS:  {:#?}, {:#?}, {:#?}",function_id,args, statement);
        return AST::Node{kind: AST::NodeKind::FunctionDeclaration {identifier: Box::new(function_id), /*arguments: args, */statement: Box::new(statement)}    }
        //panic!("impl a Function here");
    }
    return left
  }
  pub fn parse_hstmts(&mut self, tokens: &mut Vec<Token>, depth: i32) -> AST::Node {
    let mut left = AST::Node{kind: AST::NodeKind::NullLiteral{value: AST::NodeValue::Nullus}};
    if self.at(tokens).tokentype == TokenType::If {
      let _ = self.eat(tokens);

      left = AST::Node{kind:AST::NodeKind::IfStatement {
        condition: Box::new(self.parse_type_declaration(tokens, depth)),
        body: Box::new(self.parse_type_declaration(tokens, depth)),
      }};
    } else {
      left = self.parse_type_declaration(tokens, depth);
    }
    left
  }
  pub fn parse_type_declaration(&mut self, tokens: &mut Vec<Token>, depth: i32) -> AST::Node {
    let mut left = self.parse_statements(tokens, depth); // Function
    while self.at(tokens).tokenvalue == TokenValue::Operator(Operator::DoubleColon) {
      self.eat(tokens);
      left = AST::Node{kind: AST::NodeKind::TypeDeclaration{
        identifier: Box::new(left),
        ftype: Box::new(self.parse_statements(tokens, depth)),
      }};
    }
    left
  }
  pub fn parse_statements(&mut self, tokens: &mut Vec<Token>, depth: i32) -> AST::Node {
    let mut left = self.parse_expr(tokens, depth);
    while self.at(tokens).tokentype == TokenType::If || self.at(tokens).tokentype == TokenType::Match || self.at(tokens).tokentype == TokenType::Case || self.at(tokens).tokenvalue==TokenValue::Operator(Operator::LeftArrow) {

      left = match self.at(tokens).tokentype {
        TokenType::If => {
          self.eat(tokens);
          AST::Node{kind: AST::NodeKind::IfStatement {
            condition: Box::new(self.parse_expr(tokens, depth)),
            body: Box::new(self.parse_expr(tokens, depth)),
          }}
        },
        TokenType::Match => {
          self.eat(tokens);
          let mut impls: Vec<Box<AST::Node>> = vec![];
          while self.at(tokens).tokentype == TokenType::Case {
            impls.push(Box::new(self.parse_stmt(tokens, depth)));
          }
          AST::Node{kind: AST::NodeKind::Match{left: Box::new(left), values: impls}}
        },
        TokenType::Case => {
          self.eat(tokens);
          let mut assumptions: Vec<Box<AST::Node>> = vec![Box::new(self.parse_stmt(tokens, depth))];
          while self.at(tokens).tokentype == TokenType::SemiColon {
            self.eat(tokens);
            assumptions.push(Box::new(self.parse_stmt(tokens, depth)));
          }
          AST::Node{kind: AST::NodeKind::AdvancedDeclaration{body: Box::new(left), assumptions}}
        },
        TokenType::Operator => {
          AST::Node{kind: AST::NodeKind::BinaryExpression{
            left: Box::new(left),
            operator: match self.eat(tokens).tokenvalue {
              TokenValue::Operator(o) => o,
              _ => panic!("A")
            },
            right: Box::new(self.parse_multiplicative_expr(tokens, depth)),
          }}
        }
        _ => panic!("impossible")
      };
    }
    return left
  }
  pub fn parse_expr(&mut self, tokens: &mut Vec<Token>, depth: i32) -> AST::Node {
    let mut left = self.parse_additive_expr(tokens, depth);

    while self.at(tokens).tokenvalue==TokenValue::Operator(Operator::Comparision)   ||
          self.at(tokens).tokenvalue==TokenValue::Operator(Operator::Greater)       || 
          self.at(tokens).tokenvalue==TokenValue::Operator(Operator::Lower)         ||
          self.at(tokens).tokenvalue==TokenValue::Operator(Operator::GreaterEqual)  ||
          self.at(tokens).tokenvalue==TokenValue::Operator(Operator::LowerEqual)    ||
          self.at(tokens).tokenvalue==TokenValue::Operator(Operator::DoubleDot)     ||
          self.at(tokens).tokenvalue==TokenValue::Operator(Operator::RightArrow)    ||
          self.at(tokens).tokenvalue==TokenValue::Operator(Operator::RightFatArrow) {
      left = AST::Node{kind: AST::NodeKind::BinaryExpression{
        left: Box::new(left),
        operator: match self.eat(tokens).tokenvalue {
          TokenValue::Operator(o) => o,
          _ => panic!("A")
        },
        right: Box::new(match tokens[0].tokenvalue {TokenValue::Nullus => AST::Node{kind: AST::NodeKind::NullLiteral{value: AST::NodeValue::Nullus}}, _ => self.parse_additive_expr(tokens, depth)}),
      }};
    }
    return left

  }
  pub fn parse_additive_expr(&mut self, tokens: &mut Vec<Token>, depth: i32) -> AST::Node {
    let mut left = self.parse_multiplicative_expr(tokens, depth);
    while self.at(tokens).tokenvalue==TokenValue::Operator(Operator::Addition) || 
          self.at(tokens).tokenvalue==TokenValue::Operator(Operator::Substraction) {

      left = AST::Node{kind: AST::NodeKind::BinaryExpression {
        left: Box::new(left),
        operator: match self.eat(tokens).tokenvalue {
            TokenValue::Operator(Operator::Addition) => Operator::Addition,
            _ => Operator::Substraction
        },
        right: Box::<AST::Node>::new(self.parse_multiplicative_expr(tokens, depth)),
      }};
    }
    return left
    //AST::Node{kind: AST::NodeKind::NullLiteral, ..Default::default()}//while self.at(tokens).tokentype
  }
  pub fn parse_multiplicative_expr(&mut self, tokens: &mut Vec<Token>, depth: i32) -> AST::Node {
    let mut left = self.parse_exponential_expr(tokens, depth);
    while self.at(tokens).tokenvalue==TokenValue::Operator(Operator::Multiplication) ||
          self.at(tokens).tokenvalue==TokenValue::Operator(Operator::Division) ||
          self.at(tokens).tokenvalue==TokenValue::Operator(Operator::DivideRest) {
      left = AST::Node{kind: AST::NodeKind::BinaryExpression{
        left: Box::new(left),
        operator: match self.eat(tokens).tokenvalue {
            TokenValue::Operator(Operator::Multiplication) => Operator::Multiplication,
            TokenValue::Operator(Operator::DivideRest) => Operator::DivideRest,
            _ => Operator::Division,
        },
        right: Box::<AST::Node>::new(self.parse_exponential_expr(tokens, depth)),
      }};
    }

    return left
  }
  pub fn parse_exponential_expr(&mut self, tokens: &mut Vec<Token>, depth: i32) -> AST::Node {
    let mut left = self.parse_secondary_expr(tokens, depth);
    while self.at(tokens).tokenvalue==TokenValue::Operator(Operator::Exponentiation) /*||
          self.at(tokens).tokenvalue==TokenValue::Operator(Operator::Pierwiastekidk)*/ {
      left = AST::Node{kind: AST::NodeKind::BinaryExpression{
        left: Box::new(left),
        operator: match self.eat(tokens).tokenvalue {
            TokenValue::Operator(Operator::Exponentiation) => Operator::Exponentiation,
            _ => Operator::Exponentiation,
        },

        right: Box::<AST::Node>::new(self.parse_secondary_expr(tokens, depth)),
      }};
    }
    return left

  }
  pub fn parse_secondary_expr(&mut self, tokens: &mut Vec<Token>, depth: i32) -> AST::Node {
    let mut left = self.parse_primary_expr(tokens, depth);
    while self.at(tokens).tokenvalue==TokenValue::Operator(Operator::SingleDot) || 
          self.at(tokens).tokenvalue==TokenValue::Operator(Operator::ListSplitter) {
      left = match self.eat(tokens).tokenvalue {
        TokenValue::Operator(Operator::ListSplitter) =>
          AST::Node{kind: AST::NodeKind::ListConcat{
            item: Box::new(left),
            list: Box::<AST::Node>::new(self.parse_primary_expr(tokens, depth)),
          }},
        TokenValue::Operator(Operator::SingleDot) =>
          AST::Node{kind: AST::NodeKind::Access{
            parent: Box::new(left),
            value: Box::new(self.parse_primary_expr(tokens, 0)),
          }},
        _ => panic!("impossible hapnd")
      };
    }
    return left
  }

  pub fn parse_primary_expr(&mut self, tokens: &mut Vec<Token>, depth: i32) -> AST::Node {
    let token = self.at(tokens).tokentype;
    let eat = self.eat(tokens);
    let empty_node = Box::<AST::Node>::new(AST::Node{kind: AST::NodeKind::NullLiteral{value: AST::NodeValue::Nullus}});
    match token {
      TokenType::Bool => {
        match eat.tokenvalue {
          TokenValue::Bool(b) => {
            AST::Node{kind: AST::NodeKind::Bool{value: AST::NodeValue::Bool(b)}}
          }
          _ => panic!("?")
        }
      },
      TokenType::Identifier => {
        let mut childs: Vec<Box<AST::Node>> = vec![];
        if depth>0 {
          while self.at(tokens).tokentype==TokenType::Identifier ||  self.at(tokens).tokentype==TokenType::Integer
                || self.at(tokens).tokentype==TokenType::OpenSParen || self.at(tokens).tokentype==TokenType::Bool || self.at(tokens).tokentype==TokenType::OpenParen || self.at(tokens).tokentype==TokenType::ArgumentDivisor
                || self.at(tokens).tokentype==TokenType::String || self.at(tokens).tokentype == TokenType::Char || self.at(tokens).tokentype==TokenType::OpenSParen || self.at(tokens).tokentype==TokenType::OpenCParen {
            match self.at(tokens).tokentype {
              /*TokenType::Identifier => {
                childs.push(Box::new(AST::Node{kind: AST::NodeKind::Identifier{symbol: self.eat(tokens).tokenvalue.to_string(), childs: vec![]}}));
              },*/
              TokenType::ArgumentDivisor =>  {
                self.eat(tokens);
                childs.push(Box::new(self.parse_hstmts(tokens, 1)));
              },
              /*
              TokenType::Integer => {
                childs.push(Box::new(self.parse_expr(tokens)));
              },
              TokenType::OpenSParen => {
                childs.push(Box::new(self.parse_expr(tokens)));
              },
              */
              _ => {childs.push(Box::new(self.parse_hstmts(tokens, 0)));},
              //_ => {break;}
            }
          }
        }
        /*while self.at(tokens).tokentype!=TokenType::SemiColon || self.at(tokens).tokentype!=TokenType::Let || self.at(tokens).tokentype!=TokenType::EOF || self.at(tokens).tokentype!=TokenType::Operator {
          match self.at(tokens).tokentype {
            TokenType::Identifier => {
              childs.push(Box::new(AST::Node{kind: AST::NodeKind::Identifier{symbol: self.eat(tokens).tokenvalue.to_string(), childs: vec![]}}));
            },
            _ => {childs.push(Box::new(self.parse_expr(tokens)));}
          }
        }*/
        AST::Node {kind: AST::NodeKind::Identifier{symbol: eat.tokenvalue.to_string(), childs}}
      },

      TokenType::Nullus => AST::Node {kind: AST::NodeKind::NullLiteral{value: AST::NodeValue::Nullus}},
      
      TokenType::Operator => AST::Node {kind: AST::NodeKind::BinaryExpression{ operator: 
          match eat.tokenvalue {TokenValue::Operator(op) => op, _ => panic!("Operator is not an operator!")}, left: empty_node.clone(), right: empty_node}},
      
      TokenType::Integer => AST::Node {kind: AST::NodeKind::NumericLiteral{ value: AST::NodeValue::Integer(
          match eat.tokenvalue { TokenValue::Int(integer) => integer, _ => panic!("???") })}},
      
      TokenType::OpenParen => {
          let value = self.parse_stmt(tokens, 1);
          self.eatExpect(TokenType::CloseParen, "Invalid Token found, expected CloseParen `)`".to_string(), tokens);
          return value;
      },
      TokenType::OpenSParen => {
        let mut args: Vec<Box<AST::Node>> = vec![];
        while self.at(tokens).tokentype!=TokenType::CloseSParen {
          args.push(Box::new(self.parse_stmt(tokens, 1)));
        }
        self.eatExpect(TokenType::CloseSParen, "Invalid token".to_string(), tokens);
        return AST::Node{kind: AST::NodeKind::List {body: args}};
      },
      TokenType::OpenCParen => {
        //make it own type tbh
        let mut args: Vec<(Box<AST::Node>, Box<AST::Node>)> = vec![];
        let mut flags: Vec<AST::ConfigFlag> = vec![];
        //let is_config = true;
        while self.at(tokens).tokentype!=TokenType::CloseCParen {
          let eval =  self.parse_stmt(tokens, 1);
          match eval.kind {
            AST::NodeKind::FunctionDeclaration{identifier, statement} => {
              let left = *identifier.clone();
              let right = *statement.clone();
              args.push((Box::new(left), Box::new(right) ));
            },
            AST::NodeKind::NullLiteral{..} => {},
            //_ => panic!("Passed a rather weird value to a config: {:#?}", left)//AST::Node{kind: AST::NodeKind::NullLiteral{value:AST::NodeValue::Nullus}}
            _ => {
              if !flags.contains(&AST::ConfigFlag::CodeBlock) {
                flags.push(AST::ConfigFlag::CodeBlock);
              }
              let left = AST::Node{kind: AST::NodeKind::Identifier{symbol: String::from("FOKO_EVALUATE_NODE_I_WILL_KRILL"), childs: vec![]}};
              let right = eval;
              args.push((Box::new(left), Box::new(right) ));
            },
          };
        }
        self.eatExpect(TokenType::CloseCParen, "Invalid token".to_string(), tokens);
        AST::Node{kind: AST::NodeKind::Config {arguments: args, flags}}
      },
      TokenType::Char => {
          match eat.tokenvalue {
            TokenValue::Char(c) => {
              return AST::Node{kind: AST::NodeKind::Char {value: AST::NodeValue::Char(c)}}
            }
            _ => panic!("A")
          }
      },
      TokenType::String => {
        let mut  cvec: Vec<Box<AST::Node>> = vec![];
        match eat.tokenvalue {
          TokenValue::String(s) => {
            for c in s.chars() {
              cvec.push(Box::new(AST::Node{kind: AST::NodeKind::Char {value: AST::NodeValue::Char(c)}}));
            }
          }
          _ => panic!("n")
        }

        return AST::Node{kind: AST::NodeKind::List {body: cvec}}
      },
      TokenType::Case => {
          let implication = self.parse_stmt(tokens, depth);
          AST::Node{kind: AST::NodeKind::Case{value: Box::new(implication)}}
      },
      TokenType::Match => {
        
        let left = AST::Node{kind: AST::NodeKind::NullLiteral{value: AST::NodeValue::Nullus}};
        let mut impls: Vec<Box<AST::Node>> = vec![];
        while self.at(tokens).tokentype == TokenType::Case {
          impls.push(Box::new(self.parse_stmt(tokens, depth)));
        }
        AST::Node{kind: AST::NodeKind::Match{left: Box::new(left), values: impls}}
      },
      TokenType::SemiColon => {
        //self.parse_stmt(tokens)
        AST::Node{kind: AST::NodeKind::NullLiteral {value: AST::NodeValue::Nullus}}
      },
      _ => panic!("Invalid Token Found: {:#?}", eat)
    }
  }
}


