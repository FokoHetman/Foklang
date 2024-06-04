mod AST;
use core::tokenizer::{Token, TokenValue, TokenType};
#[derive(Clone, Copy)]
pub struct Parser {}

impl Parser {

  pub fn parse(self, static_tokens: Vec<Token>) -> AST::Node {
    let mut result = AST::Node{kind: AST::NodeKind::Program, body: Some([].to_vec()), ..Default::default()};
    let mut tokens = static_tokens.clone();
    while tokens[0].tokentype!=TokenType::EOF {
      
    }

    return result
  }
}
