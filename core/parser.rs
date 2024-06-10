use core::AST;
use core::tokenizer::{Token, TokenValue, TokenType, Operator};
#[derive(Clone, Copy)]
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
      result.appendToBody(self.parse_stmt( &mut tokens));
    }

    return result
  }

  pub fn parse_stmt(&mut self, tokens: &mut Vec<Token>) -> AST::Node {
    match self.at(tokens).tokentype {
      //TokenType::Let => self.parse_var_declaration()
      _ => self.parse_expr(tokens)
    }
  }
  pub fn parse_expr(&mut self, tokens: &mut Vec<Token>) -> AST::Node {
    self.parse_additive_expr(tokens)

  }
  pub fn parse_additive_expr(&mut self, tokens: &mut Vec<Token>) -> AST::Node {
    let mut left = self.parse_multiplicative_expr(tokens);
    while self.at(tokens).tokenvalue==TokenValue::Operator(Operator::Addition) || 
          self.at(tokens).tokenvalue==TokenValue::Operator(Operator::Substraction) {

      left = AST::Node{kind: AST::NodeKind::BinaryExpression {
        left: Box::new(left),
        operator: match self.eat(tokens).tokenvalue {
            TokenValue::Operator(Operator::Addition) => Operator::Addition,
            _ => Operator::Substraction
        },
        right: Box::<AST::Node>::new(self.parse_multiplicative_expr(tokens)),
      }};
    }
    return left
    //AST::Node{kind: AST::NodeKind::NullLiteral, ..Default::default()}//while self.at(tokens).tokentype
  }
  pub fn parse_multiplicative_expr(&mut self, tokens: &mut Vec<Token>) -> AST::Node {
    let mut left = self.parse_exponential_expr(tokens);
    while self.at(tokens).tokenvalue==TokenValue::Operator(Operator::Multiplication) ||
          self.at(tokens).tokenvalue==TokenValue::Operator(Operator::Division) {
      left = AST::Node{kind: AST::NodeKind::BinaryExpression{
        left: Box::new(left),
        operator: match self.eat(tokens).tokenvalue {
            TokenValue::Operator(Operator::Multiplication) => Operator::Multiplication,
            _ => Operator::Division,
        },
        right: Box::<AST::Node>::new(self.parse_exponential_expr(tokens)),
      }};
    }

    return left
  }
  pub fn parse_exponential_expr(&mut self, tokens: &mut Vec<Token>) -> AST::Node {
    let mut left = self.parse_primary_expr(tokens);
    while self.at(tokens).tokenvalue==TokenValue::Operator(Operator::Exponentiation) /*||
          self.at(tokens).tokenvalue==TokenValue::Operator(Operator::Pierwiastekidk)*/ {
      left = AST::Node{kind: AST::NodeKind::BinaryExpression{
        left: Box::new(left),
        operator: match self.eat(tokens).tokenvalue {
            TokenValue::Operator(Operator::Exponentiation) => Operator::Exponentiation,
            _ => Operator::Exponentiation,
        },

        right: Box::<AST::Node>::new(self.parse_primary_expr(tokens)),
      }};
    }
    return left

  }
 /* pub fn parse_function_declaration(&mut self, tokens: &mut Vec<Token>) -> AST::Node {
    let mut left = self.parse_primary_expr(tokens);

    if self.at(tokens).tokentype==TokenType::Let {
      let _ = self.eat(tokens);
      let function_id = self.eat(tokens);
      let mut args = Vec::<Token>::new();
      while self.at(tokens).tokentype==TokenType::Identifier {
        args.push(self.eat(tokens));
      }

      //self.eatExpectValue(TokenValue::Operator(Operator::Equal), "expected equal sign".to_string(), tokens);
      println!("FUNCTINO: {:#?}, {:#?}", function_id, args);
    }
    return left
  }*/
  pub fn parse_primary_expr(&mut self, tokens: &mut Vec<Token>) -> AST::Node {
    let token = self.at(tokens).tokentype;
    let eat = self.eat(tokens);
    let empty_node = Box::<AST::Node>::new(AST::Node{kind: AST::NodeKind::NullLiteral{value: AST::NodeValue::Nullus}});
    match token {
      TokenType::Identifier => AST::Node {kind: AST::NodeKind::Identifier{symbol: eat.tokenvalue.to_string()}},

      TokenType::Nullus => AST::Node {kind: AST::NodeKind::NullLiteral{value: AST::NodeValue::Nullus}},
      
      TokenType::Operator => AST::Node {kind: AST::NodeKind::BinaryExpression{ operator: 
          match eat.tokenvalue {TokenValue::Operator(op) => op, _ => panic!("Operator is not an operator!")}, left: empty_node.clone(), right: empty_node}},
      
      TokenType::Integer => AST::Node {kind: AST::NodeKind::NumericLiteral{ value: AST::NodeValue::Integer(
          match eat.tokenvalue { TokenValue::Int(integer) => integer, _ => panic!("???") })}},
      
      TokenType::OpenParen => {
          let value = self.parse_expr(tokens);
          self.eatExpect(TokenType::CloseParen, "Invalid Token found, expected CloseParen `)`".to_string(), tokens);
          return value;
      },
      
      TokenType::OpenCParen => {
          //make it own type tbh
          let value = self.parse_expr(tokens);
          self.eatExpect(TokenType::CloseCParen, "Invalid Token found, expected CloseCParen `}`".to_string(), tokens);
          //println!("{:#?}", value);
          return value
      },
      TokenType::Let => {
          let function_id = self.parse_expr(tokens);
          let mut args = Vec::<Box<AST::Node>>::new();
          while self.at(tokens).tokentype==TokenType::Identifier {
            args.push(Box::new(self.parse_expr(tokens)));
          }
          self.eatExpectValue(TokenValue::Operator(Operator::Equal), "expected =".to_string(), tokens);
          let statement = self.parse_expr(tokens);
          //println!("FUNCTION PARS:  {:#?}, {:#?}, {:#?}",function_id,args, statement);
        return AST::Node{kind: AST::NodeKind::FunctionDeclaration {identifier: Box::new(function_id), arguments: args, statement: Box::new(statement)}    }
        //panic!("impl a Function here");
      }
      _ => panic!("Invalid Token Found: {:#?}", eat)
    }
  }
}









