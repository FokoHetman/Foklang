use core::tokenizer::{Operator,TokenType};
use core::AST::{Node,NodeValue};

#[derive(Clone,Copy)]
pub struct ErrorHandler {}

pub struct Peritia {
  pub bool: bool,
  pub error_msg: String,
}

impl ErrorHandler {

  pub fn check_binary_expression(self, left: i32, right: i32) -> Peritia {
    Peritia{bool: false, error_msg: String::new()}

  }
  pub fn check_binary_expression_division(self, left: i32, right: i32) -> Peritia {
    Peritia{bool: right==0, error_msg: String::from("Can't divide by 0!")}
  }
}
