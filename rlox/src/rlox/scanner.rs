
use std::fmt::{self, Error};
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Literal{
    String(String),
    Double(f64)
}

impl fmt::Display for Literal{
    fn fmt(&self, f: &mut fmt::Formatter)->fmt::Result{
        write!(f, "{:?}", self)
    }
}
#[derive(Debug, Copy, Clone)]
pub enum TokenType {
    
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
    
    Bang, BangEqual, Equal, EqualEqual,
    Greater, GreaterEqual, Less, LessEqual,
    
    Identifier, String, Number,
    
    And, Class, Else, False, Func, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While, Eof
}

impl fmt::Display for TokenType{
    fn fmt(&self, f: &mut fmt::Formatter)->fmt::Result{
        write!(f, "{:?}", self)
    }
}

impl std::ops::Add<String> for TokenType{
    type Output = String;
    fn add(self, rhs:String)->String{
        return self + rhs;
    }
}
#[derive(Clone)]
pub struct Token{
    token_type: TokenType,
    lexeme: String,
    line: usize,
    literal: Literal,
}

impl Token{
    pub fn new( token_type:TokenType, lexeme:String, line:usize, literal:Literal) -> Token{
        return Token {
            token_type: token_type,
            lexeme: lexeme,
            line: line,
            literal: literal
        };
    }

    pub fn to_string(&self)->String{
        return self.token_type + " ".to_string() + &self.lexeme + &self.literal.to_string();
    }
}

// impl Copy for Token{}

// impl Clone for Token{
//     fn clone(&self)->Token{
//         *self
//     }
// }

impl fmt::Display for Token{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.lexeme)
    }
}

pub struct Scanner<'a>{
    parent: Rc<RefCell<&'a mut super::Rlox>>,
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>


}


impl<'a> Scanner<'a>{
    pub fn new(source: &String, parent: Rc<RefCell<&'a mut super::Rlox>>)->Scanner<'a>{

        let mut keywords: HashMap<String, TokenType> = HashMap::new();
        keywords.insert("and".to_string(),    TokenType::And);
        keywords.insert("class".to_string(),  TokenType::Class);
        keywords.insert("else".to_string(),   TokenType::Else);
        keywords.insert("false".to_string(),  TokenType::False);
        keywords.insert("for".to_string(),    TokenType::For);
        keywords.insert("func".to_string(),    TokenType::Func);
        keywords.insert("if".to_string(),     TokenType::If);
        keywords.insert("nil".to_string(),    TokenType::Nil);
        keywords.insert("or".to_string(),     TokenType::Or);
        keywords.insert("print".to_string(),  TokenType::Print);
        keywords.insert("return".to_string(), TokenType::Return);
        keywords.insert("super".to_string(),  TokenType::Super);
        keywords.insert("this".to_string(),   TokenType::This);
        keywords.insert("true".to_string(),   TokenType::True);
        keywords.insert("var".to_string(),    TokenType::Var);
        keywords.insert("while".to_string(),  TokenType::While);

        Scanner::<'a>{
            parent: parent,
            source: source.to_string(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords: keywords
        }
    }

    fn at_end(&self)->bool{
        return self.current >= self.source.len();
    }

    fn advance(&mut self)->char{
        self.current +=1;
        return self.source.chars().nth(self.current).unwrap();
    }

    fn add_token(&self, token:TokenType){
        unimplemented!();
    }

    fn add_token_with_literal(&mut self, token:TokenType, literal:Literal){
        let text:String = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(token, text, self.line,literal))
    }

    fn scan_token(&mut self,){
        // could be some issues w/ the char being 4 bytes in rust vs. 2 in most languages
        let  c: char = self.advance();

        match c {
            '('=>self.add_token(TokenType::LeftParen),
            ')'=>self.add_token(TokenType::RightParen),
            '{'=>self.add_token(TokenType::LeftBrace),
            '}'=>self.add_token(TokenType::RightBrace),
            ','=>self.add_token(TokenType::Comma),
            '.'=>self.add_token(TokenType::Dot),
            '-'=>self.add_token(TokenType::Minus),
            '+'=>self.add_token(TokenType::Plus),
            ';'=>self.add_token(TokenType::Semicolon),
            '*'=>self.add_token(TokenType::Star),
            '!'=>if self.check_match('=') {self.add_token(TokenType::BangEqual)} else {self.add_token(TokenType::Bang)},
            '='=>if self.check_match('=') {self.add_token(TokenType::EqualEqual)} else {self.add_token(TokenType::Equal)},
            '<'=>if self.check_match('=') {self.add_token(TokenType::LessEqual)} else {self.add_token(TokenType::Less)},
            '>'=>if self.check_match('=') {self.add_token(TokenType::GreaterEqual)} else {self.add_token(TokenType::Greater)},
            '/'=>{if self.check_match('/') {
               while self.peek() != '\n' && !self.at_end() {
                self.advance();
               }
            } else {
                self.add_token(TokenType::Slash);
            }
            },
            ' ' | '\r' | '\t'=>{},
            '\n'=>{
                self.line+=1;
            },
            '"'=> self.stringy(),
            'o'=>{
                if self.peek() == 'r' {
                    self.add_token(TokenType::Or)
                }
            }
            _=>{
                if self.is_digit(c){
                    self.number();
                } else if self.is_alpha(c){
                    self.identifier()
                }
                 else {
                    let mut reference = self.parent.borrow_mut();
                    reference.error(self.line, "Unexpected character");
            }
        }
        }
    }

    fn identifier(&mut self){
        while self.is_alphanum(self.peek()) {self.advance();};
        let text = self.source[self.start..self.current+1].to_string();
        let token_type = self.keywords.get(&text);

        if token_type.is_none() {self.add_token(TokenType::Identifier)} else{
            self.add_token(*token_type.unwrap());
        } 
    }

    fn is_alphanum(&self, c: char)->bool{
        return self.is_alpha(c) || self.is_digit(c);
    }

    fn is_alpha(&self, c: char)-> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_digit(&self, c: char)->bool{
        return c >= '0' && c <= '9';
    }

    fn number(&mut self){
        while self.is_digit(self.peek()){
            self.advance();
        }

        if self.peek() == '.'  && self.is_digit(self.peek_next()){
            self.advance();
            while self.is_digit(self.peek()){
                self.advance();
            }
        }

        let value = Literal::Double(self.source[self.start..self.current+1].parse().unwrap());

        self.add_token_with_literal(TokenType::Number, value);


    }

    fn stringy(&mut self){
        while self.peek() != '"' && !self.at_end(){
            if self.peek() == '\n' {self.line+=1}
            self.advance();
        }

        if self.at_end(){
            let mut reference = self.parent.borrow_mut();
            reference.error(self.line, "Unterminated string");
            return;
        }

        self.advance();

        let value = Literal::String(self.source[self.start+1..self.current+1].to_string());
        self.add_token_with_literal(TokenType::String, value);
    }
    fn peek(&self)->char {
        if self.at_end() {
            return '\0';
        } else {
            return self.source.chars().nth(self.current).unwrap();
        }

    }

    fn peek_next(&self)->char{
        if self.current+1 >= self.source.len() {return '\0'}
        return self.source.chars().nth(self.current+1).unwrap();
    }

    fn check_match(&mut self, check_for: char)->bool{
        if self.at_end() {return false}
        let char_at: char = self.source.chars().nth(self.current).unwrap();
        if char_at != check_for {return false};
        self.current+=1;
        return true;
    }

    pub fn get_tokens(&mut self)->Vec<Token>{
        loop {
            if !self.at_end(){
                self.start = self.current;
                self.scan_token()
            }
            break;
        }

        return self.tokens.clone();

    }



}
