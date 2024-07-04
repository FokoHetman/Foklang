mod core;
use std::{
  io,
  io::{Write},
  fs,
  env,
};

fn main() {

  let tokenizer = core::tokenizer::Tokenizer {};
  let mut parser = core::parser::Parser {};
  let error_handler = core::error_handler::ErrorHandler {};
  let mut env = core::asm_env::Environment{ error_handler: error_handler, ..Default::default() };



  core::compiler::declare_builtins(&mut env);
  let mut compiler = core::compiler::Compiler {stack_size: 0};//error_handler: error_handler};
   
  let args = env::args().collect::<Vec<String>>();
  let input = fs::read_to_string(args[1].clone()).unwrap();



  let mut tokenized_input = tokenizer.tokenize(input);
  //println!("Tokenizer Out: {:#?}", tokenized_input);


  let mut parsed_input = parser.parse(tokenized_input);
  //println!("Parser Out: {:#?}", parsed_input);


  let mut compiled_input = compiler.compile(parsed_input, &mut env, args[1].clone());
  println!("Compiler Out: \n{}", compiled_input);
  if args.len()>3 {
    if args[2].clone()=="-o".to_string() {
      fs::write(args[3].clone(), compiled_input).unwrap();
    }
  }else {
    fs::write(args[1].clone().replace(".fok", ".asm"), compiled_input).unwrap();
    println!("Saving to {}", args[1].clone().replace(".fok", ".asm"));
  }
  //core::builtins::println(core::builtins::Arguments{function: core::builtins::FunctionArgs::print(vec![compiled_input])});
}
