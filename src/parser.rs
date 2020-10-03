use std::collections::HashMap;

use crate::lexer::*;
use Token::*;
pub type ParsingResult = Result<(Vec<ASTNode>, Vec<Token>), String>;

#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
    ExternNode(Prototype),
    FunctionNode(Function),
}
#[derive(PartialEq, Clone, Debug)]
pub struct Function {
    pub prototype: Prototype,
    pub body: Expression,
}
#[derive(PartialEq, Clone, Debug)]
pub struct Prototype {
    pub name: String,
    pub args: Vec<String>,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    LiteralExpr(f64),
    VariableExpr(String),
    BinaryExpr(String, Box<Expression>, Box<Expression>),
    CallExpr(String, Vec<Expression>),
}

pub enum PartParsingResult<T> {
    Good(T, Vec<Token>),
    NotComplete,
    Bad(String),
}
pub use PartParsingResult::*;


macro_rules! parse_try {
    ($function:ident,$tokens:ident,$settings:ident,$parsed_tokens:ident) => {
        parse_try!($function,$tokens,$settings,$parsed_tokens,);
    };
    ($function:ident, $tokens:ident, $settings:ident, $parsed_tokens:ident, $($arg:expr),*) => {
        match $function($tokens,$setting,$(arg:expr),*)=>{
            Good(ast,toks)=> {
                $parsed_tokens.extend(toks.into_iter());
                ast
            }
            NotComplete => {
                $parsed_tokens.reverse();
                $tokens.extend($parsed_tokens.into_iter());
                return NotComplete;
            }
            Bad(message) => Bad(message)
        }
    }

}

pub struct ParserSettings {
    operator_precedence: HashMap<String, i32>
}
pub fn default_parser_settings() -> ParserSettings {
    let mut operator_precedence = HashMap::new();
//> ch-1 ch-4 ch-5 parser-default-settings
    operator_precedence.insert("=".to_string(), 2);
//< ch-1 ch-4 ch-5 parser-default-settings
    operator_precedence.insert("<".to_string(), 10);
    operator_precedence.insert("+".to_string(), 20);
    operator_precedence.insert("-".to_string(), 20);
    operator_precedence.insert("*".to_string(), 40);

    ParserSettings{operator_precedence: operator_precedence}
}

fn parse_extern(tokens:&mut Vec<Token>,settings:&mut ParserSettings)-> PartParsingResult<ASTNode>{
    
    tokens.pop();
    let mut parsed_tokens = vec![Extern];
    



    PartParsingResult::
}





pub fn parse(tokens: &[Token], parsed_tree: &[ASTNode]) -> ParsingResult {
    let mut rest = tokens.to_vec();
    rest.reverse();
    let mut ast = parsed_tree.to_vec();

    loop {
        let curr_token = match rest.last() {
            Some(token) => token.clone(),
            None => break,
        };

        let result: PartParsingResult<ASTNode> = match curr_token {
            // placeholder type
            Token::Def => PartParsingResult::NotComplete,    // parse function
            Token::Extern => PartParsingResult::NotComplete, // parse Extern
            Token::Delimiter => {
                rest.pop();
                continue;
            }
            _ => PartParsingResult::NotComplete, // parse expression
        };

        match result {
            Good(ast_node,_) => ast.push(ast_node),
            NotComplete => break,
            Bad(message) => return Err(message)
        }

    }

    rest.reverse();
    Ok((ast, rest))
}
