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

use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[token(">")]
    IncrementPointer,

    #[token("<")]
    DecrementPointer,

    #[token("+")]
    IncrementValue,

    #[token("-")]
    DecrementValue,

    #[token(".")]
    OutputChar,

    #[token(",")]
    InputChar,

    #[token("[")]
    LoopStart,

    #[token("]")]
    LoopEnd,

    #[regex(r"[^<>+\-.,\[\]]")]
    Error,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut lexer = Token::lexer(input);

    while let Some(token) = lexer.next() {
        tokens.push(token.unwrap())
    }

    tokens
}