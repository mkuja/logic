use lazy_static::lazy_static;
use std::str;
use fancy_regex as re;

#[derive(Debug)]
pub enum TokenType {
    And,
    Or,
    Not,
    Xor,
    LeftParen,
    RightParen,
    Name(String),
    Whitespace(String),
    Error(String),
}

pub fn tokenize(source: &str) -> Vec<&str> {
    let mut tokens = Vec::new();
    for tok in get_regex().captures_iter(source) {
        tokens.push(tok.unwrap().get(0).unwrap().as_str());
    }
        
    tokens
}

fn get_regex() -> &'static re::Regex {
    lazy_static! {
        static ref RE: re::Regex = re::Regex::new(
        r"\Gand|\Gxor|\Gor|\G~|\G!|\G[ \t\n]+|\G[[:alpha:]]|\G\(|\G\)"
    ).unwrap();
    }
    &RE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_tokenize() {
        let src = "P and Q";
        let tokens = tokenize(src);
        assert_eq!(tokens, vec!["P", " ", "and", " ", "Q"]);
    }

}
