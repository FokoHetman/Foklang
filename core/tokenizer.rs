use std::{
  str::FromStr,
  collections::HashMap,
};
#[derive(PartialEq,Clone,Debug)]
pub enum Operator {
  Addition,
  Substraction,
  Multiplication,
  Division,
  Exponentiation,
  Equal,
}
#[derive(PartialEq,Clone,Debug)]
pub enum Bool {
  True,
  False,
}
#[derive(Debug,Clone,PartialEq)]
pub enum TokenValue {
  Nullus,
  Str(String),
  Int(i32),
  Operator(Operator),
  Identifier(String),
  Let,
  Bool(Bool),
}
#[derive(Debug,Clone,PartialEq)]
pub enum TokenType {
  CloseParen,
  OpenParen,
  CloseCParen,
  OpenCParen,
  CloseSParen,
  OpenSParen,
  Integer,
  Operator,
  Let,
  Identifier,
  Bool,
  EOF,
  Nullus,
  SemiColon,
  Space,
  Apostroph,
  Quotation,
}
#[derive(Debug,Clone,PartialEq)]
pub struct Token {
  pub tokentype: TokenType,
  pub tokenvalue: TokenValue,
}
#[derive(Clone,Copy)]
pub struct Tokenizer {}


impl TokenValue {
  pub fn to_string(self) -> String {
    match self {
      TokenValue::Identifier(s) => s,
      _ => String::new()
    }
  }
}


impl Tokenizer {
  pub fn is_numeric(self, string: String) -> bool {
    if string.len()>0 {
      return char::from_str(&string).unwrap().is_numeric()
    }
    return false
  }
  pub fn is_alpha(self, string: String) -> bool {
    return string.to_lowercase()!=string.to_uppercase();
  }

  pub fn tokenize(self, input: String) -> Vec<Token> {
    let mut list_input = input.split("").collect::<Vec<&str>>();
    let mut tokens: Vec<Token> = [].to_vec();
    let mut pass;
    let speciales: HashMap<String, (TokenType, TokenValue)> = HashMap::from([(String::from("let"), (TokenType::Let, TokenValue::Let)), (String::from("true"), (TokenType::Bool, TokenValue::Bool(Bool::True)))]);
    while list_input.len()>0 {
      pass = false;
      let current_char = list_input[0];
      match current_char {
        "[" => {
          tokens.push(Token{tokentype: TokenType::OpenSParen, tokenvalue: TokenValue::Nullus});
        },
        "]" => {
          tokens.push(Token{tokentype: TokenType::CloseSParen, tokenvalue: TokenValue::Nullus});
        },
        "{" => {
          tokens.push(Token{tokentype: TokenType::OpenCParen, tokenvalue: TokenValue::Nullus});
        },
        "}" => {
          tokens.push(Token{tokentype: TokenType::CloseCParen, tokenvalue: TokenValue::Nullus});
        },
        "(" => {
          tokens.push(Token{tokentype: TokenType::OpenParen, tokenvalue: TokenValue::Nullus});
        },
        ")" => {
          tokens.push(Token{tokentype: TokenType::CloseParen, tokenvalue: TokenValue::Nullus});
        },
        "=" => {
          tokens.push(Token{tokentype: TokenType::Operator, tokenvalue: TokenValue::Operator(Operator::Equal)});
        },
        "+" => {
          tokens.push(Token{tokentype: TokenType::Operator, tokenvalue: TokenValue::Operator(Operator::Addition)});
        },
        "-" => {
          tokens.push(Token{tokentype: TokenType::Operator, tokenvalue: TokenValue::Operator(Operator::Substraction)});
        },
        "*" => {
          tokens.push(Token{tokentype: TokenType::Operator, tokenvalue: TokenValue::Operator(Operator::Multiplication)});
        },
        "/" => {
          tokens.push(Token{tokentype: TokenType::Operator, tokenvalue: TokenValue::Operator(Operator::Division)});
        },
        "^" => {
          tokens.push(Token{tokentype: TokenType::Operator, tokenvalue: TokenValue::Operator(Operator::Exponentiation)});  
        },
        ";" => {
          tokens.push(Token{tokentype: TokenType::SemiColon, tokenvalue: TokenValue::Nullus});
        },
        "'" => {
          tokens.push(Token{tokentype: TokenType::Apostroph, tokenvalue: TokenValue::Nullus});
        },
        "\"" => {
          tokens.push(Token{tokentype: TokenType::Quotation, tokenvalue: TokenValue::Nullus});
        },
        " " => {
          tokens.push(Token{tokentype: TokenType::Space, tokenvalue: TokenValue::Nullus});
        },
        _ => {
          pass = self.is_numeric(list_input[0].to_string()) || self.is_alpha(list_input[0].to_string());
          if self.is_numeric(list_input[0].to_string()) {
            let mut tmp_num: String = String::new();
            while list_input.len()>0 && char::from_str(list_input[0]).unwrap().is_numeric() {
              tmp_num+=list_input[0];
              list_input.remove(0);
            }
            tokens.push(Token{tokentype: TokenType::Integer, tokenvalue: TokenValue::Int(tmp_num.parse::<i32>().unwrap())});
          }
          else if self.is_alpha(list_input[0].to_string()) {
            let mut tmp_ident: String = String::new();
            while list_input.len()>0 && self.is_alpha(list_input[0].to_string()) {
              tmp_ident+=list_input[0];
              list_input.remove(0);
            }
            if speciales.contains_key(&tmp_ident) {
              tokens.push(Token{tokentype: speciales.get(&tmp_ident).unwrap().0.clone(), tokenvalue: speciales.get(&tmp_ident).unwrap().1.clone()});
            }
            else {
              tokens.push(Token{tokentype: TokenType::Identifier, tokenvalue: TokenValue::Identifier(tmp_ident.to_string())});
            }
/*            match speciales.get(&tmp_ident).unwrap() {
              Ok((tmptype,value)) => tokens.push(Token{tokentype: tmptype, tokenvalue: value(tmp_ident)}),
              Err(_) => tokens.push(Token{tokentype: TokenType::Identifier, tokenvalue: TokenValue::Identifier(tmp_ident)})
            }*/
          }
        }
      }
      if !pass {
        list_input.remove(0);
      }
    }
    tokens.push(Token{tokentype: TokenType::EOF, tokenvalue: TokenValue::Nullus});
    return tokens
  }
}
