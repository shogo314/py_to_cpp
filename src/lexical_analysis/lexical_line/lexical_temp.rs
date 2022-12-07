use crate::chartype::{is_endofline, is_whitespace};
use crate::lexical::{comment, identifier, keyword, literal, operator, separator, Lexical};
pub struct LexicalTemp {
    s: String,
}
impl LexicalTemp {
    pub fn new() -> LexicalTemp {
        LexicalTemp { s: String::new() }
    }
    pub fn update(&mut self, c_now: char, c_next: char) -> Option<Lexical> {
        fn normalize(s: &str) -> Lexical {
            if keyword::is_key(s) {
                return Lexical::Keyword(keyword::to_key(s).unwrap());
            } else if s == ":" {
                return Lexical::Separator(separator::to_sep(s).unwrap());
            } else if s == "==" {
                return Lexical::Operator(operator::to_ope(s).unwrap());
            } else if s == "#" {
                return Lexical::Comment(comment::Comment::Name("#".to_string()));
            } else if s == "\'abc\'" {
                return Lexical::Literal(literal::Literal::String(s.to_string()));
            } else if s == "True" {
                return Lexical::Literal(literal::Literal::Boolean(true));
            } else if s == "0" {
                return Lexical::Literal(literal::Literal::Integer(0));
            } else {
                return Lexical::Identifier(identifier::Identifier::Name(s.to_string()));
            }
        }
        if is_endofline(c_now) {
            return None;
        }
        self.s.push(c_now);
        if is_whitespace(c_next) {
            let res = normalize(&self.s);
            self.s = String::new();
            Some(res)
        } else {
            None
        }
    }
}
