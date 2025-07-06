use std::io::{self, Read};

// Enum to represent different SQL tokens
#[derive(Debug, PartialEq, Clone)]
enum Token {
    Keyword(String),
    Identifier(String),
    Literal(String),
    Operator(String),
    Punctuation(char),
    Whitespace,
    EOF,
}

// Lexer struct to handle main tokenization
struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    // Create a new Lexer instance
    fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    // Gets next character from input string
    fn next_token(&mut self) -> Token {
        if self.position >= self.input.len() {
            return Token::EOF;
        }

        let ch: char = self.input[self.position];
        self.position += 1;

        match ch {
            ' ' | '\t' | '\n' | '\r' => Token::Whitespace,
            ',' | ';' | '(' | ')' => Token::Punctuation(ch),
            '+' | '-' | '*' | '/' | '=' | '<' | '>' => Token::Operator(ch.to_string()),
            '\'' => self.read_string_literal(),
            _ if ch.is_alphabetic() => self.read_identifier(ch),
            _ if ch.is_digit(10) => self.read_number_literal(ch),
            _ => Token::Identifier(ch.to_string()),
        }
    }

    // Helper function to read a complete idetifier or keyword
    fn read_identifier(&mut self, first_char: char) -> Token {
        let mut ident: String = String::new();
        ident.push(first_char);

        while self.position < self.input.len() && self.input[self.position].is_alphanumeric() {
            ident.push(self.input[self.position]);
            self.position += 1;
        }

        match ident.to_uppercase().as_str() {
            "SELECT" | "FROM" | "WHERE" | "AND" | "OR" | "INSERT" | "INTO" | "UPDATE" | "SET"
            | "DELETE" | "JOIN" | "LEFT" | "RIGHT" | "OUTER" | "INNER" | "ON" | "GROUP" | "BY"
            | "ORDER" | "HAVING" | "AS" | "CREATE" | "TABLE" | "DROP" | "ALTER" => {
                Token::Keyword(ident.to_uppercase())
            }
            _ => Token::Identifier(ident),
        }
    }

    // Helper funciton to read a string literal enclosed in single quotes
    fn read_string_literal(&mut self) -> Token {
        let mut literal: String = String::new();
        literal.push('\'');

        while self.position < self.input.len() && self.input[self.position] != '\'' {
            literal.push(self.input[self.position]);
            self.position += 1;
        }

        if self.position < self.input.len() {
            literal.push(self.input[self.position]);
            self.position += 1;
        }
        Token::Literal(literal)
    }

    // Helper function to read a numeric literal
    fn read_number_literal(&mut self, first_char: char) -> Token {
        let mut literal: String = String::new();
        literal.push(first_char);

        while self.position < self.input.len() && self.input[self.position].is_digit(10) {
            literal.push(self.input[self.position]);
            self.position += 1;
        }

        Token::Literal(literal)
    }
}

struct Formatter {
    tokens: Vec<Token>,
    indent_level: usize,
    output: String,
}

impl Formatter {
    // Creates new formatter instance
    fn new(tokens: Vec<Token>) -> Self {
        Formatter {
            tokens,
            indent_level: 0,
            output: String::new(),
        }
    }

    // Function to format SQL
    fn format(&mut self) -> String {
        let mut last_token: Option<Token> = None;
        let tokens: Vec<Token> = self.tokens.clone();

        for token in &tokens {
            match token {
                Token::Keyword(kw) => match kw.as_str() {
                    "SELECT" | "FROM" | "WHERE" | "UPDATE" | "SET" | "GROUP" | "ORDER" | "LEFT"
                    | "RIGHT" | "INNER" => {
                        self.new_line();
                        self.append_token(token);
                        self.new_line();
                        self.indent_level += 1;
                    }
                    "AND" | "OR" => {
                        self.new_line();
                        self.append_token(token);
                    }
                    _ => {
                        self.append_with_space(token, &last_token);
                    }
                },
                Token::Punctuation('(') => {
                    self.append_with_space(token, &last_token);
                    self.indent_level += 1;
                    self.new_line();
                }
                Token::Punctuation(')') => {
                    self.indent_level -= 1;
                    self.new_line();
                    self.append_token(token);
                }
                Token::Punctuation(',') => {
                    self.append_token(token);
                    self.new_line();
                }
                Token::Whitespace => { /* IGNORE WHITESPACE */ }
                _ => {
                    self.append_with_space(token, &last_token);
                }
            }

            if token != &Token::Whitespace {
                last_token = Some(token.clone());
            }
        }

        self.output.trim().to_string()
    }

    fn append_token(&mut self, token: &Token) {
        let _: String = "\t".repeat(self.indent_level);
        match token {
            Token::Keyword(s) | Token::Identifier(s) | Token::Literal(s) | Token::Operator(s) => {
                self.output.push_str(s)
            }
            Token::Punctuation(c) => self.output.push(*c),
            _ => {}
        }
    }

    fn append_with_space(&mut self, token: &Token, last_token: &Option<Token>) {
        if let Some(last) = last_token {
            match last {
                Token::Punctuation('(') => {}
                _ => self.output.push(' '),
            }
        }
        self.append_token(token);
    }

    fn new_line(&mut self) {
        if !self.output.is_empty() && self.output.ends_with('\n') {
            self.output.push('\n');
        }

        let indent: String = "\t".repeat(self.indent_level);
        self.output.push_str(&indent);
    }
}

fn main() {
    println!(
        "Enter SQL statement to format and press ctrl+D (mac/linux) or ctrl+Z (windows) when done:"
    );

    let mut buffer: String = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    if buffer.trim().is_empty() {
        println!("No input provided!!");
        return;
    }

    let mut lexer: Lexer = Lexer::new(&buffer);
    let mut tokens: Vec<_> = Vec::new();

    loop {
        let token: Token = lexer.next_token();
        if token == Token::EOF {
            break;
        }
        tokens.push(token);
    }

    let mut formatter: Formatter = Formatter::new(tokens);
    let formatted_sql: String = formatter.format();

    println!("\n---Formatted SQL---\n");
    println!("{}", formatted_sql);
}
