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

use crate::parser::AstNode;

#[derive(Debug, Clone)]
pub struct Interpreter {
    tape: [u8; 30000],
    pointer: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            tape: [0; 30000],
            pointer: 0,
        }
    }

    pub fn execute(&mut self, ast_nodes: Vec<AstNode>) {
        for ast_node in ast_nodes {
            match ast_node {
                AstNode::IncrementPointer => self.pointer += 1,
                AstNode::DecrementPointer => self.pointer -= 1,
                AstNode::IncrementValue => {
                    self.tape[self.pointer] = self.tape[self.pointer].wrapping_add(1);
                }
                AstNode::DecrementValue => {
                    self.tape[self.pointer] = self.tape[self.pointer].wrapping_sub(1);
                }
                AstNode::OutputChar => print!("{}", self.tape[self.pointer] as char),
                AstNode::InputChar => {
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).expect("Failed to read line");
                    self.tape[self.pointer] = input.chars().next().unwrap_or('\0') as u8;
                }
                AstNode::Loop(inner_nodes) => {
                    while self.tape[self.pointer] != 0 {
                        self.execute(inner_nodes.clone());
                    }
                }
            }
        }
    }
}