use crate::core::AST;
use crate::core::error_handler::ErrorHandler;
use crate::core::tokenizer::{Operator};
use std::{convert::TryInto};
use crate::core::env::Environment;
use crate::core::{tokenizer, parser};

use super::AST::{Arithmetics, Concept, NumericalConvertability};


#[derive(Debug,Clone)]
pub struct Interpreter {pub error_handler:ErrorHandler, pub parser: parser::Parser, pub tokenizer: tokenizer::Tokenizer}

impl Interpreter {

  pub fn unwindable(&mut self, concept: Concept) -> bool {
    match concept {
      AST::Concept::Numerical(_) => true, // idk
      _ => false
    }
  }

  pub fn unwind(&mut self, concept: Concept, env: &mut Environment) -> AST::Concept {
    match concept {
      Concept::Operation(O) => {
        match O {
          AST::OperationConcept::BinaryOperation(B) => {
            match B {
              _ => todo!()
            }
          },
          _ => todo!()
        }
      },
      // I think nums don't need re-eval. `elem`'s should be uhh idk we'll see
      _ => concept,
    }
  }


  fn evaluate_program(&mut self, program: AST::Node, env: &mut Environment) -> AST::Concept {
    let mut last_eval = AST::Concept::Nullus;

    match program.kind {
      AST::NodeKind::Program{body,id: _} => {
        env.current_node = 0;
        let mut counter = 0;
        env.node_stack = [].to_vec();
        env.push_args(body.clone());
        for i in body {
          if counter==env.current_node {
            last_eval = self.evaluate(*i, env);
            env.current_node+=1;
          }
          counter+=1;
        }
        //last_eval.id = id;
      }
      _ => panic!("[Interpreter Error] Tried to evaluate non-Program Node as a Program, {:#?}", program)
    }
    return last_eval
  }
  pub fn evaluate(&mut self, node: AST::Node, env: &mut Environment) -> AST::Concept {
    match node.kind {
      AST::NodeKind::Program{body:_,id:_} => self.evaluate_program(node, env),
      AST::NodeKind::NullLiteral{value:_} => AST::Concept::Nullus,
      AST::NodeKind::NumericLiteral{value:i} => AST::Concept::Numerical(AST::NumericalConcept::Real(AST::RealNumericalConcept::Int(AST::Int{value:match i {AST::NodeValue::Integer(i) => i, _ => 0}}))),
      AST::NodeKind::BinaryExpression{left:_,right:_,operator:_} => self.evaluate_binary_expression(node, env),
     _ => panic!("{} {:#?}", self.error_handler.interpreter("unknown_node").error_msg, node)
    }
  }

  fn evaluate_binary_expression(&mut self, node: AST::Node, env: &mut Environment) -> AST::Concept {
    match node.kind {
      AST::NodeKind::BinaryExpression{left: node_left,right: node_right,operator: node_operator} => {
        /*let left = match self.evaluate(*node_left, env).value {
          AST::Fructa::Numerum(i) => i,
          _ => panic!("hi I fucked up yipee!")
        };
        let right = match self.evaluate(*node_right, env).value {
          AST::Fructa::Numerum(i) => i,
          _ => panic!("hi I fucked up big yipee!")
        };*/

        /*if self.error_handler.check_binary_expression(left,right).bool {
          panic!("[Interpreter Error] Binary Expression: {:#?}", self.error_handler.check_binary_expression(left,right).error_msg);
        };*/
        match node_operator {
          Operator::Addition => AST::Concept::Operation(AST::OperationConcept::BinaryOperation(
            AST::BinaryOpConcept::Addition(Box::new(self.evaluate(*node_left, env)), Box::new(self.evaluate(*node_right, env)))
          )),

          Operator::Substraction => AST::Concept::Operation(AST::OperationConcept::BinaryOperation(
            AST::BinaryOpConcept::Addition(Box::new(self.evaluate(*node_left, env)), Box::new(AST::Concept::adversity(self.evaluate(*node_right, env))))
          )),

          Operator::Multiplication =>  AST::Concept::Operation(AST::OperationConcept::BinaryOperation(
            AST::BinaryOpConcept::Multiplication(Box::new(self.evaluate(*node_left, env)), Box::new(self.evaluate(*node_right, env)))
          )),
          
          Operator::Division =>  AST::Concept::Operation(AST::OperationConcept::BinaryOperation(
            AST::BinaryOpConcept::Multiplication(Box::new(self.evaluate(*node_left, env)), Box::new({
                let value = self.evaluate(*node_right, env);
                match value {
                  AST::Concept::Numerical(N) => {
                    match N {
                      AST::NumericalConcept::Real(R) => {
                        AST::Concept::Numerical(AST::NumericalConcept::Real(
                          AST::RealNumericalConcept::Fraction(AST::Fraction{numeral: Box::new(AST::RealNumericalConcept::from_i32(1)), nominal: Box::new(R)})
                        ))
                      },
                      AST::NumericalConcept::Imaginary(I) => {
                        AST::Concept::Numerical(AST::NumericalConcept::Imaginary(
                          AST::ImaginaryNumericalConcept::ComplexNumber(
                            AST::ComplexNumber {real: AST::RealNumericalConcept::from_i32(0), 
                              imaginary: AST::RealNumericalConcept::Fraction(AST::Fraction{numeral: Box::new(AST::RealNumericalConcept::from_i32(1)), 
                                  nominal: Box::new(match I {AST::ImaginaryNumericalConcept::ComplexNumber(c) => c.imaginary, AST::ImaginaryNumericalConcept::ImaginaryNumber() => AST::RealNumericalConcept::from_i32(1)})
                              })
                            })
                        ))
                      }
                    }
                  }
                  _ => panic!("no division implementation for {:#?}", value)
                }
            }))
          )),
          _ => todo!()
        }

        //AST::Proventus{value: AST::Fructa::Numerum(result), id: -1}
      }
      _ => panic!("[Interpreter Error] Tried to evaluate non-BinaryExpression Node as BinaryExpression, {:#?}", node)
    }
  }


}


