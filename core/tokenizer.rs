use std::{
  str::FromStr,
  collections::HashMap,
};
#[derive(PartialEq,Clone,Debug)]
pub enum Operator {
  Addition,             // +    -> a + b
  Substraction,         // -    -> x - y
  Multiplication,       // *    ->  x * 2
  Division,             // /    ->  e / 2
  Exponentiation,       // ^    ->  e ^ x
  DivideRest,           // %    -> a % b
  Equal,                // =    ->  x = 4
  
  RightArrow,           // ->   ->  x -> idk
  LeftArrow,            // <-   ->  x <- [1..10]
  
  RightFatArrow,        // =>

  Comparision,          // ==   ->  2+2==4

  Greater,              // >    ->  2 > 1
  Lower,                // <    ->  2 <  4
  GreaterEqual,         // >=   ->  2 >= 1
  LowerEqual,           // <=   ->  2 <= 4

  DoubleDot,            // ..   ->  [1..10]
  SingleDot,            // .    -> help.me  (get help me)

  ListSplitter,         // :    ->  (x:xs)
  DoubleColon,          // ::   -> function :: Type -> Type...
  

}
#[derive(Debug,Clone,PartialEq)]
pub enum TokenValue {
  Nullus,
  Char(char),
  String(String),
  Int(i32),
  Operator(Operator),
  Identifier(String),
  Bool(bool),
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
  Identifier,
  Bool,

  EOF,
  Nullus,
  SemiColon,


  ArgumentDivisor,      // $    ->  f x $ g y  -> f(x, (g(y)))
  Case,                 // |    -> f x = match | x==2 => 2 | x==3 => 3
  Match,                // match
  Char,
  String,
  If,
}
#[derive(Debug,Clone,PartialEq)]
pub struct Token {
  pub tokentype: TokenType,
  pub tokenvalue: TokenValue,
}
#[derive(Debug,Clone,Copy)]
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
  pub fn is_identifier(self, string: String) -> bool {
    return string.to_lowercase()!=string.to_uppercase() || self.is_numeric(string.clone()) || [String::from("_")].contains(&string);
  }

  pub fn tokenize(self, input: String) -> Vec<Token> {
    let mut list_input: Vec<char> = input.replace("\\n", "\n").replace("\\t", "\t").replace("\\\"", "\"").replace("\\'", "\'").replace("\\x1b", "\x1b").chars().collect();
    let mut tokens: Vec<Token> = [].to_vec();
    let mut pass;
    let speciales: HashMap<String, (TokenType, TokenValue)> = HashMap::from([(String::from("true"), (TokenType::Bool, TokenValue::Bool(true))), (String::from("false"), (TokenType::Bool, TokenValue::Bool(false))), (String::from("if"), (TokenType::If, TokenValue::Nullus)), (String::from("match"), (TokenType::Match, TokenValue::Nullus))]);
    while list_input.len()>0 {
      pass = false;
      let current_char = list_input[0];
      match current_char {
        '[' => {
          tokens.push(Token{tokentype: TokenType::OpenSParen, tokenvalue: TokenValue::Nullus});
        },
        ']' => {
          tokens.push(Token{tokentype: TokenType::CloseSParen, tokenvalue: TokenValue::Nullus});
        },
        '{' => {
          tokens.push(Token{tokentype: TokenType::OpenCParen, tokenvalue: TokenValue::Nullus});
        },
        '}' => {
          tokens.push(Token{tokentype: TokenType::CloseCParen, tokenvalue: TokenValue::Nullus});
        },
        '(' => {
          tokens.push(Token{tokentype: TokenType::OpenParen, tokenvalue: TokenValue::Nullus});
        },
        ')' => {
          tokens.push(Token{tokentype: TokenType::CloseParen, tokenvalue: TokenValue::Nullus});
        },
        '.' => {
          match list_input[1] {
            '.' => {
              list_input.remove(0);
              tokens.push(Token{tokentype: TokenType::Operator, tokenvalue: TokenValue::Operator(Operator::DoubleDot)});
            }
            _ => {tokens.push(Token{tokentype: TokenType::Operator, tokenvalue: TokenValue::Operator(Operator::SingleDot)});}
          }
        },
        '=' => {
          match list_input[1] {
            '=' => {
              list_input.remove(0);
              tokens.push(Token{tokentype: TokenType::Operator, tokenvalue: TokenValue::Operator(Operator::Comparision)});
            },
            '>' => {
              list_input.remove(0);
              tokens.push(Token{tokentype: TokenType::Operator, tokenvalue: TokenValue::Operator(Operator::RightFatArrow)});
            },
            _ => {
              tokens.push(Token{tokentype: TokenType::Operator, tokenvalue: TokenValue::Operator(Operator::Equal)});
            }
          }
        },
        '+' => {
          tokens.push(Token{tokentype: TokenType::Operator, tokenvalue: TokenValue::Operator(Operator::Addition)});
        },
        '-' => {
          match list_input[1] {
            '>' => {
              list_input.remove(0);
              tokens.push(Token{tokentype: TokenType::Operator, tokenvalue: TokenValue::Operator(Operator::RightArrow)});
            },
            _ => {
              tokens.push(Token{tokentype: TokenType::Operator, tokenvalue: TokenValue::Operator(Operator::Substraction)});
            }
          }
        },
        '*' => {
          tokens.push(Token{tokentype: TokenType::Operator, tokenvalue: TokenValue::Operator(Operator::Multiplication)});
        },
        '/' => {
          tokens.push(Token{tokentype: TokenType::Operator, tokenvalue: TokenValue::Operator(Operator::Division)});
        },
        '%' => {
          tokens.push(Token{tokentype: TokenType::Operator, tokenvalue: TokenValue::Operator(Operator::DivideRest)});
        },
        '^' => {
          tokens.push(Token{tokentype: TokenType::Operator, tokenvalue: TokenValue::Operator(Operator::Exponentiation)});  
        },
        ';' => {
          tokens.push(Token{tokentype: TokenType::SemiColon, tokenvalue: TokenValue::Nullus});
        },

        '\'' => {
          list_input.remove(0);
          let chr = list_input[0];
          list_input.remove(0);
          /*match chr {
            '\\' => {
              chr = match list_input[0] {
                'n' => '\n',
                't' => '\t',
                '
              };
              list_input.remove(0);
            },
            _ => {}
          }*/
          if list_input[0]!='\'' {
            panic!("tf you doing");
          }
          tokens.push(Token{tokentype: TokenType::Char, tokenvalue: TokenValue::Char(chr)});
        },
        '"' => {
          let mut deval = String::new();
          list_input.remove(0);
          while list_input[0]!='"' {
            deval+=&list_input[0].to_string();
            list_input.remove(0);
          }
          tokens.push(Token{tokentype: TokenType::String, tokenvalue: TokenValue::String(deval)});
        },
        '>' => {

          match list_input[1] {
            '=' => {
              list_input.remove(0);
              tokens.push(Token{tokentype: TokenType::Operator, tokenvalue: TokenValue::Operator(Operator::GreaterEqual)});
            },
            _ => {
              tokens.push(Token{tokentype: TokenType::Operator, tokenvalue: TokenValue::Operator(Operator::Greater)});
            }
          }
        },
        '<' => {
          match list_input[1] {
            '=' => {
              list_input.remove(0);
              tokens.push(Token{tokentype: TokenType::Operator, tokenvalue: TokenValue::Operator(Operator::LowerEqual)});
            },
            '-' => {
              list_input.remove(0);
              tokens.push(Token{tokentype: TokenType::Operator, tokenvalue: TokenValue::Operator(Operator::LeftArrow)});
            },
            _ => {
              tokens.push(Token{tokentype: TokenType::Operator, tokenvalue: TokenValue::Operator(Operator::Lower)});
            }
          }
        },
        '$' => {
          tokens.push(Token{tokentype: TokenType::ArgumentDivisor, tokenvalue: TokenValue::Nullus});
        },
        '|' => {
          tokens.push(Token{tokentype: TokenType::Case, tokenvalue: TokenValue::Nullus});
        },
        ':' => {
          match list_input[1] {
            ':' => {
              list_input.remove(0);
              tokens.push(Token{tokentype: TokenType::Operator, tokenvalue: TokenValue::Operator(Operator::DoubleColon)});
            },
            _ => {
              tokens.push(Token{tokentype: TokenType::Operator, tokenvalue: TokenValue::Operator(Operator::ListSplitter)});
            }
          }
        },
        '#' => {
          match list_input[1] {
            '!' => {
              list_input.remove(0);
              list_input.remove(0);
              while list_input.len()>1 && list_input[0]!='\n' {
                list_input.remove(0);
              }
            },
            _ => {
              
            }
          }
        }
        _ => {
          pass = self.is_numeric(list_input[0].to_string()) || self.is_alpha(list_input[0].to_string());
          if self.is_numeric(list_input[0].to_string()) {
            let mut tmp_num: String = String::new();
            while list_input.len()>0 && list_input[0].is_numeric() {
              tmp_num+=&list_input[0].to_string();
              list_input.remove(0);
            }
            tokens.push(Token{tokentype: TokenType::Integer, tokenvalue: TokenValue::Int(tmp_num.parse::<i32>().unwrap())});
          }
          else if self.is_alpha(list_input[0].to_string()) {
            let mut tmp_ident: String = String::new();
            while list_input.len()>0 && self.is_identifier(list_input[0].to_string()) {
              tmp_ident+=&list_input[0].to_string();
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
