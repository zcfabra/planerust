use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;

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
    literal: String,
}

impl Token{
    pub fn new( token_type:TokenType, lexeme:String, line:usize, literal:String) -> Token{
        return Token {
            token_type: token_type,
            lexeme: lexeme,
            line: line,
            literal: literal
        };
    }

    pub fn to_string(&self)->String{
        return self.token_type + " ".to_string() + &self.lexeme + &self.literal;
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
    line: usize


}


impl<'a> Scanner<'a>{
    pub fn new(source: &String, parent: Rc<RefCell<&'a mut super::Rlox>>)->Scanner<'a>{
        Scanner::<'a>{
            parent: parent,
            source: source.to_string(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1
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

    fn add_token_with_literal(&mut self, token:TokenType, literal:String){
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
            
            _=>{
                let mut reference = self.parent.borrow_mut();
                reference.error(self.line, "Unexpected Charcater".to_string())}
        }
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
