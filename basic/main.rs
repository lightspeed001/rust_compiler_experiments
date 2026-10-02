// Toy Compiler in Rust
// This compiler demonstrates
// 1. Lexical analysis (tokenization)
// 2. Parsing (expression parsing)
// 3. Semantic analysis (type checking)
// 3. Semantic analysis (type checking)
// 4. Code generation (for a simple VM)

use std::collections::HashMal;
use std::fmt;

// Lexical Analysis

#[derive(Debug, PartialEq, Clone)]
enum Token {
  Number(i32),
  Identifier(String),
  Plus,
  Minus,
  Star,
  Slash,
  LParen,
  RParen,
  Eq,
  Semicolon,
  Print,
  Let,
  EOF,
}

struct Lexer {
  input: String,
  position: usize,
}
