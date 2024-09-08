mod core;
use std::{
  io,
  io::{Read,Write},
  env,
  fs,
};

fn main() {
  let shell: &str = "foklang";
  let mut input = String::new();
  let tokenizer = core::tokenizer::Tokenizer {};
  let mut parser = core::parser::Parser {};
  let error_handler = core::error_handler::ErrorHandler {};
  let mut env = core::env::Environment{ error_handler, ..Default::default() };

  core::builtins::declare_builtins(&mut env);
  let mut interpreter = core::interpreter::Interpreter {error_handler};


  if env::args().len() > 1 {
    input = fs::read_to_string(env::args().collect::<Vec<String>>()[1].clone()).unwrap();
    let exit_code = match interpreter.evaluate(parser.parse(tokenizer.tokenize(input)), &mut env).value {
      core::AST::Fructa::Numerum(i) => i,
      _ => 0,
    };
    return;
  }

  
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
    //println!("Interpreter Out: {:#?}", interpreted_input);
    core::builtins::println(core::builtins::Arguments{function: core::builtins::FunctionArgs::many(vec![interpreted_input])});
  }
}
