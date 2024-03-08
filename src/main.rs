/*
This file belongs to rusty-brain, a blazingly fast brainfuck interpreter.
Copyright (c) 2024 Liam Dvorscak, MIT License

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
 */

use std::collections::HashMap;
use std::time::Instant;
use crate::interpreter::Interpreter;
use crate::lexer::tokenize;
use crate::parser::parse;

use repl_rs::{Command, Convert, Parameter, Repl, Result, Value};

mod lexer;
mod parser;
mod interpreter;

fn interpret<T>(args: HashMap<String, Value>, _context: &mut T) -> Result<Option<String>> {
    let file_name: String = args["file"].convert()?;
    // assumes file_name is in the same directory as the executable
    // get the contents into one single string
    let contents = std::fs::read_to_string(&file_name).unwrap();

    // timings start
    let start_time = Instant::now();

    // tokenize the brainfuck code
    let tokens = tokenize(&contents);
    // generate ast
    let ast = parse(tokens);
    // create new interpreter instance and walk the ast
    let mut interpreter = Interpreter::new();
    interpreter.execute(ast);

    // timings end
    let elapsed_time = start_time.elapsed();

    // finished
    Ok(Some(format!("Finished executing {}, took {}ns to complete", file_name, elapsed_time.as_nanos())))
}

fn main() -> Result<()> {
    let mut repl = Repl::new(())
        .with_name("Rusty_Brain")
        .with_version("0.1.0")
        .with_description("Brainf*ck parser implemented in rust")
        .add_command(
            Command::new("interpret", interpret)
                .with_parameter(Parameter::new("file").set_required(true)?)?
                .with_help("The file name to interpret the contents of")
        );

    repl.run()
}
