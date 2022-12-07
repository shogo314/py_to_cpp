use std::path::is_separator;

use crate::chartype::*;
use crate::lexical::comment::Comment;
use crate::lexical::identifier::Identifier;
use crate::lexical::keyword::{is_key, to_key};
use crate::lexical::literal::Literal;
use crate::lexical::operator::{is_ope,to_ope};
use crate::lexical::separator::to_sep;
use crate::lexical::Lexical;

pub struct LexicalTemp {
    s: String,
}

impl LexicalTemp {
    pub fn new() -> LexicalTemp {
        LexicalTemp { s: String::new() }
    }
    pub fn update(&mut self, c_now: char, c_next: char) -> Option<Lexical> {
        fn normalize(s: &str) -> Lexical {
            let c = &s.chars().next().unwrap();
            if is_letter(c){
                if is_key(s){
                    Lexical::Keyword(to_key(s).unwrap())
                }else if is_ope(s){
                    Lexical::Operator(to_ope(s).unwrap())
                }else{
                    Lexical::Identifier(Identifier::Name(s.to_string()))
                }
            }else if is_numeral(c){
                Lexical::Literal(Literal::Integer(s.parse().unwrap()))
            }else if is_operator(c){
                match to_ope(s) {
                    Some(y)=>Lexical::Operator(y),
                    None=>panic!("{:?} is not ope",s),
                }
            }else if is_separator(c){
                Lexical::Separator(to_sep(s).unwrap())
            }else if is_hash(c){
                Lexical::Comment(Comment::Name(s.to_string()))
            }else{
                panic!("{:?} is not.",s)
            }
        }
        fn decision_add(s: &str, c: &char) -> bool {
            let c0 = s.chars().next().unwrap();
            if is_letter(&c0) && (is_letter(c) || is_numeral(c))
                || is_numeral(&c0) && is_numeral(c)
                || is_operator(&c0) && is_operator(c)
            {
                true
            } else {
                false
            }
        }
        if is_endofline(&c_now) {
            return None;
        }
        self.s.push(c_now);
        if is_whitespace(&c_next) || !decision_add(&self.s, &c_next) {
            let res = normalize(&self.s);
            self.s = String::new();
            Some(res)
        } else {
            None
        }
    }
}
