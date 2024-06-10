use core::tokenizer::{Operator,TokenType};
use core::AST::{Node,NodeValue};

#[derive(Debug,Clone,Copy)]
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
  pub fn environment(self, error_id: &str) -> Peritia {
    let header = "[Environment Error]";
    Peritia{bool: false, error_msg: match error_id {
      "nonidentifier_node" => format!("{} Non-Identifier Node was passed as an Identifier!", header),
      "already_defined" => format!("{} Identifier was already defined!", header),
      _ => String::new()
    }}
  }
  pub fn interpreter(self, error_id: &str) -> Peritia {
    let header = "[Interpreter Error]";
    Peritia{bool: false, error_msg: match error_id {
      "unknown_node" => format!("{} Unknown Node!", header),
      "nonfunctiondeclaration_node" => format!("{} Tried to evaluate a non-FunctionDeclaration as FunctionDeclaration!", header),
      _ => String::new()
    }}
  }
}
