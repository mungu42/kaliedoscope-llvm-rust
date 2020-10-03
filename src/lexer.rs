#![feature(plugin)]
extern crate regex;
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Def,
    Extern,
    Delimiter,
    OpeningParenthesis,
    ClosingParenthesis,
    Comma,
    Ident(String),
    Number(f64),
    Operator(String),
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let comment_re = Regex::new(r"(?m)#.*\n").expect("Invalid Regex!");
    println!("{}", input);
    let preprocessed = comment_re.replace(input, "\n");
    let comments = comment_re.captures(input).unwrap();
    for comment in comments.iter() {
        println!("{:?}", comment);
    }

    let mut result: Vec<Token> = Vec::new();
    let token_re = Regex::new(concat!(
        r"(?P<ident>\p{Alphabetic}\w*)|",
        r"(?P<number>\d+\.?\d*)|",
        r"(?P<delimiter>;)|",
        r"(?P<oppar>\()|",
        r"(?P<clpar>\))|",
        r"(?P<comma>,)|",
        r"(?P<operator>\S)"
    ))
    .expect("Invalid Regex for Tokens!");

    for cap in token_re.captures_iter(preprocessed.as_str()) {
        let token: Token = if let Some(inner) = cap.name("ident") {
            match inner {
                "def" => Token::Def,
                "extern" => Token::Extern,
                ident => Token::Ident(ident.to_string()),
            }
        } else if let Some(inner) = cap.name("number") {
            let number = inner.parse::<f64>().expect("Parse Error for Number");
            Token::Number(number)
        } else if cap.name("delimiter").is_some() {
            Token::Delimiter
        } else if cap.name("oppar").is_some() {
            Token::OpeningParenthesis
        } else if cap.name("clpar").is_some() {
            Token::ClosingParenthesis
        } else {
            Token::Operator(cap.name("operator").unwrap().to_string())
        };
        result.push(token);
    }
    result
}
