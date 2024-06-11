mod core;
use std::{
  io,
  io::{Write},
};

fn main() {
  let shell: &str = "foklang";
  let mut input = String::new();
  let tokenizer = core::tokenizer::Tokenizer {};
  let mut parser = core::parser::Parser {};
  let error_handler = core::error_handler::ErrorHandler {};
  let mut env = core::env::Environment{ error_handler: error_handler, ..Default::default() };
  env.declare(core::AST::Node{kind: core::AST::NodeKind::Identifier{symbol: String::from("get"), childs: vec![]}},
      core::AST::Proventus{value: core::AST::Fructa::BuiltIn(
        core::builtins::get
      ),id:-2});
  env.declare(core::AST::Node{kind: core::AST::NodeKind::Identifier{symbol: String::from("print"), childs: vec![]}},
      core::AST::Proventus{value: core::AST::Fructa::BuiltIn(
        core::builtins::print
      ),id:-2});
  let mut interpreter = core::interpreter::Interpreter {error_handler: error_handler};
  loop {
    print!("{}$ ", shell);
    input = String::new();
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut input);
    let mut tokenized_input = tokenizer.tokenize(input);
    //println!("Tokenizer Out: {:#?}", tokenized_input);

    let mut parsed_input = parser.parse(tokenized_input);

    //println!("Parser Out: {:#?}", parsed_input);

    let mut interpreted_input = interpreter.evaluate(parsed_input, &mut env);
    println!("Interpreter Out: {:#?}", interpreted_input);
  }
}
