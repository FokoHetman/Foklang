use core::AST;
use core::error_handler::ErrorHandler;
use core::tokenizer::{Operator};
use std::convert::TryInto;                                                                      
#[derive(Clone,Copy)]
pub struct Interpreter {pub error_handler:ErrorHandler}

impl Interpreter {                                                                                fn evaluate_program(self, program: AST::Node) -> AST::Proventus {
    let mut last_eval = AST::Proventus{..Default::default()};

    match program.kind {
      AST::NodeKind::Program{body:body,id:id} => {
        for i in body {
          last_eval = self.evaluate(*i);
        }                                                                                               last_eval.id = id;
      }
      _ => panic!("[Interpreter Error] Tried to evaluate non-Program Node as a Program, {:#?}", program)
    }
    return last_eval
  }
  pub fn evaluate(self, node: AST::Node) -> AST::Proventus {
    match node.kind {
      AST::NodeKind::Program{body:_,id:_} => self.evaluate_program(node),                             AST::NodeKind::NullLiteral{value:_} => AST::Proventus{value: AST::Fructa::Nullus, id: -1},
      AST::NodeKind::NumericLiteral{value:i} => AST::Proventus{value: AST::Fructa::Numerum(match i {AST::NodeValue::Integer(i) => i, _ => 0}), id: -1},
      AST::NodeKind::BinaryExpression{left:_,right:_,operator:_} => self.evaluate_binary_expression(node),
      AST::NodeKind::Identifier{symbol:_} => self.evaluate_identifier(node),
      _ => panic!("[Interpreter Error] Unknown Node {:#?}", node)
    }                                                                                             }
  fn evaluate_binary_expression(self, node: AST::Node) -> AST::Proventus {                          match node.kind {
      AST::NodeKind::BinaryExpression{left: node_left,right: node_right,operator: node_operator} => {
        let left = match self.evaluate(*node_left).value {
          AST::Fructa::Numerum(i) => i,
          _ => panic!("hi I fucked up yipee!")
        };
        let right = match self.evaluate(*node_right).value {
          AST::Fructa::Numerum(i) => i,
          _ => panic!("hi I fucked up big yipee!")
        };

        if self.error_handler.check_binary_expression(left,right).bool {
          panic!("[Interpreter Error] Binary Expression: {:#?}", self.error_handler.check_binary_expression(left,right).error_msg);
        }
        let result = match node_operator {
          Operator::Addition => left+right,
          Operator::Substraction => left-right,
          Operator::Multiplication => left*right,
          Operator::Division => {
            if self.error_handler.check_binary_expression_division(left,right).bool {
              panic!("[Interpreter Error] Binary Expression Division: {:#?}",
                  self.error_handler.check_binary_expression_division(left,right).error_msg);
            }
            left/right
          },
          Operator::Exponentiation => {
            left.pow(right.try_into().unwrap())
          }
          _ => panic!("[Interpreter Error] Unknown Operator: {:#?}", node_operator)
        };

        AST::Proventus{value: AST::Fructa::Numerum(result), id: -1}
      }
      _ => panic!("[Interpreter Error] Tried to evaluate non-BinaryExpression Node as BinaryExpression, {:#?}", node)
    }
  }
  fn evaluate_identifier(self, node: AST::Node) -> AST::Proventus {
    AST::Proventus{..Default::default()}

  }
}