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

use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum AstNode {
    IncrementPointer,
    DecrementPointer,
    IncrementValue,
    DecrementValue,
    OutputChar,
    InputChar,
    Loop(Vec<AstNode>),
}

pub fn parse(tokens: Vec<Token>) -> Vec<AstNode> {
    let mut token_iter = tokens.into_iter();
    let mut ast_nodes: Vec<AstNode> = Vec::new();

    while let Some(token) = token_iter.next() {
        let ast_node = match token {
            Token::IncrementPointer => AstNode::IncrementPointer,
            Token::DecrementPointer => AstNode::DecrementPointer,
            Token::IncrementValue => AstNode::IncrementValue,
            Token::DecrementValue => AstNode::DecrementValue,
            Token::OutputChar => AstNode::OutputChar,
            Token::InputChar => AstNode::InputChar,
            Token::LoopStart => AstNode::Loop(parse(stream_until_match(&mut token_iter))),
            _ => panic!("Unexpected token: {:?}", token),
        };

        ast_nodes.push(ast_node);
    }

    ast_nodes
}

fn stream_until_match(token_iter: &mut std::vec::IntoIter<Token>) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut nesting_level = 1;

    while let Some(token) = token_iter.next() {
        match token {
            Token::LoopStart => nesting_level += 1,
            Token::LoopEnd => {
                nesting_level -= 1;
                if nesting_level == 0 {
                    break;
                }
            }
            _ => {}
        }

        tokens.push(token);
    }

    tokens
}