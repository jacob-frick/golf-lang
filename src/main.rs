

pub mod lexer {
    #[derive(Debug)]
    pub struct Lexer {
        pub source: String,
        pub current_char: char,
        pub current_pos: i64,
        pub source_length: i64
    }

    impl Lexer {
        pub fn new(source_code: String) -> Lexer {
            let mut lex = Lexer {
                source: source_code.clone(),
                current_char: ' ',
                current_pos: -1,
                source_length: source_code.chars().count() as i64
            };
            lex.next_char();
            return lex;
        }

        pub fn next_char(&mut self) {
            self.current_pos += 1;
            if self.current_pos >= self.source_length {
                self.current_char = '\0';
            } else {
                self.current_char = self.source.chars().nth(self.current_pos as usize).unwrap();
            }
        }
        pub fn peek(&self) -> char {
            if self.current_pos + 1 > self.source_length {
                '\0'
            } else {
                self.source.chars().nth(self.current_pos as usize).unwrap()
            }
        }
    }
}

fn main() {
    let mut lex = lexer::Lexer::new("Hello World".to_string());
    println!("{:?}", lex);
    while lex.peek() != '\0' {
        println!("{}", lex.current_char);
        lex.next_char();
    }
}