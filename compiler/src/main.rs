// HyzeScript/compiler/src/main.rs

use std::env;
use std::fs;

mod lexer;
mod parser;
mod ast;
mod ir;
mod codegen;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: hyzec <input.hyze>");
        std::process::exit(1);
    }

    let input_path = &args[1];
    let source = fs::read_to_string(input_path)?;

    // 1. Lexer
    let mut tokens = lexer::tokenize(&source)?;

    // 2. Parser
    let ast = parser::parse(&tokens)?;

    // 3. Generate IR (very simple for now)
    let ir = ir::generate_ir(&ast)?;

    // 4. Emit C++
    let cpp_code = codegen::cpp::generate(&ir)?;

    // 5. Write output
    let output_path = "out.cpp";
    fs::write(output_path, cpp_code)?;

    println!("✅ HyzeScript compiled to {}", output_path);
    println!("🔧 Next: compile {} with a C++ compiler.", output_path);

    Ok(())
}
