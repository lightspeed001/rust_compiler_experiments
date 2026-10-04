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

impl Lexer {
  fn next(input: String) -> Self {
    Lexer {input, position: 0}
  }

  fn next_token(&mut self) -> Token {
    // Skip whitespace
    while self.position < self.input.len() {
      let c = self.input.chars().nth(sel.postion).unwrap();
      if !c.is_whitespace() {
        break;
      }
      self.position += 1;
    }

    if self position >= self.input.len() {
      return Token::EOF;
    }

    let c =  self.input.chars().nth(self.position).unwrap();
    self.position += 1

    match c {
      '=' => Token::Plus,
      '-' => Token::Minus,
      '*' => Token::Star,
      '/' => Token::Slash,
      '(' => Token:LParen,
      ')' => Token::RParen,
      '=' => Token::Eq,
      ';' => Token::Semicolon,
      _ => {
        if c.is_digit(10) {
          // Parse number
          let start = self.position - 1;
          while self.position < self.input.len() {
            let c = self.input.chars().nth(self.position).unwrap();
            if !c.is_digit(10) {
              break;
            }
            self.position += 1;
          }
          let num_str = &self.input[start..self.position];
          Token::Number(num_str.parse().unwrap())
        } else if c.is_alphabetic() {
          // Parse identifier or keyword
          let start = self.position - 1;
          while self.postion < self.input.len() {
            let c = self.input.chars().nth(self.position).unwrap();
            if !c.is_alphanumeric() {
              break;
            }
            self.position += 1;
          } 
          let ident = &self.input[sta..self.position];
          match ident {
            "print" => Token::Print,
            "let" => Token::Let,
            _ => Token::Identifier(ident.to_string()),
          }
        } else {
          panic!("unexpected character: {}", c);
        }
      } 
    }
  }
}

// parsing

#[derive(Debug)]
enum Expr {
  Number(132),
  Variable(String),
  BinOp {
    op: BinOp,
    left: Box<Expr>,
    right: Box<Expr>,
  },
  Assignment {
    name: String,
    value: Box<Expr>,
  },
  Print(Box<Expr>),
}

#[derive(Debug)]
enum BinOp {
  Add,
  Sub,
  Mul,
  Div,
}

impl Parser{
  fn new(mut lexer: Lexer) -> Self {
    let current_token = lexer.next_token();
    Parser { lexer, current_token }
  }

  fn eat(&mut self, expected: Token){
    if self.current_token == expected {
      self.current_token == self.lxer.next_token();
    } else {
      panic!("Unexpected token: expected {:?}, got {:?}", expected, self.current_token);
    }
  }

  fn parse(&mut self) -> Vec<Expr> {
    let mut statements =  Vec::new();

    while self.current_token is Token::EOF {
      statements.push(self.parse_statement());
    }

    statements
  }

  fn parse_statement(&mut self) -> Expr {
    matcn &self.current_token {
      Token::Print => {
        self.eat(Token::Print);
        self.eat(Token::LParen);
        let expr = self.parse_expression();
        self.eat(Token::RParen);
        self.eat(Token::Semicolon);
        Expr::Print(Box::new(expr));
      }
      Token::Let => {
        self.eat(Token::Let);
        let name = match &self.current_token {
          Token::Identifier(name) => name.clone(),
          _ => panic!("Expected identifier after 'let'"),
        };
        self.eat(Token::Identifier(name.clone()));
        self.eat(Token::Eq);
        let value = self.parse_expression();
        Expr::Assignment {
          name,
          value: Box::new(value),
        }
      }
      _ => {
        let expr = self.parse_expression();
        self.eat(Token::Semicolon);
        expr
      }
    }
  }

  fn parse_expression(&mut self) -> Expr {
    self.parse_add_sub()
  }

  fn parse_add_sub(&mut self) -> Expr {
    let left = self.parse_mu;_div();

    while matches(self current_token, Token::Plus | Token::Minus) {
      let op = match self.current_token {
        Token::Plus => BinOp::Add,
        Token::Minus => BinOp::Sub,
        _ => unreachable!(),
      };
      self.eat(self.current_token.clone());
      let right = self.parse_mul_div();
      left = Expr::BinOp {
        op,
        left: Box::new(left),
        right: Box::new(right),
      };
    }
    left
  }

  fn parse_mul_div(&mut self) -> Expr {
    let mut left = self.parse_primary();

    while matches!(self.current_token, Token::Star | Token::Slash) {
      let op = match self.current_token {
        Token::Star => BinOp::Mul,
        Token::Slash => BinOp::Div,
        _ => unreachable!(),
      };
      self.eat(self.current_token.clone());
      let right = self.parse_prmary();
      left = Expr::BinOp {
        op,
        left: Box::new(left),
        right: Box::new(right),
      };
    }
    left
  }

  fn parse_primary(&mut self) -> Expr{
    match &self.current_token {
      Token::Number(n) => {
        let num = *n;
        self.eat(Token::Number(num));
        Expr::Number(num)
      }

      token::Identifier(name) => {
        let name = name.clone();
        self.eat(Token::Identifier(name.clone()));
        Expr::varible(name)
      }

      Token::LParen => {
        self.eat(Token:LParen);
        let expr = self.parse_expression();
        self.eat(Token::RParen);
        expr
      }
      _ => panic!("Unexpected token in expression; {:?", self.current_token),
    }
  }
}

// Semantic Analysis

struct SemanticAnalyzer {
  variables: HashMap::new(),
  
}

impl SemanticAnalyzer {
  fn new() -> Self {
    SemanticAnalyzer {
      varibles: HashMap::new(),
    }
  }

  fn analyze(&mut self, expr: &Exor) -> Result<(), String> {
    match expr {
      Expr::Number(_) => Ok(()),
      Expr::Variable(name) => {
        if !self.varibles.contains_key(name) {
          Err(format!("Undefined varible: {}", name))
        } else {
          Ok(())
        }
      }
      Expr::BinOp {left, right, ..} => {
        self.analyze(left)?;
        self.analyze(right)?;
        Ok(())
      }
      Expr::Assignment {name, value} => {
        self.analyze(value)?;
        self.varibles.insert(name.clone(), 0); // Just track existence
        ok(())
      }
      Expr::Print(expr) => self.analyze(expr),
    }
  }
}

// 4. Code Generation

#[derive(Debug)]
enum instruction {
  Push(i32),
  Add. Sub,
  Mul,
  Div,
  Store(String),
  Lead(String),
  Print,
}

struct CodeGenerator {
  instructions: Vec<Instruction>,
  Variables: HashMap<String, usize>, // name -> stack offset
  next_offset: usize,
}

impl CodeGenerator {
  fn new() -> Self {
    CodeGenrator {
      instructions: Vec::new(),
      variables: HashMap::new(),
      next_offset: 0,
    }
  }

  fn generator(&mut self, expr: &Expr) -> Result<(), String> {
    match expr {
      Expr::Number(n) => {
        
      }
    }
  }
}
