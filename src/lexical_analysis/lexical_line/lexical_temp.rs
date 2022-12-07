use crate::chartype::{is_endofline, is_whitespace};
use crate::lexical::identifier::Identifier;
use crate::lexical::Lexical;
pub struct LexicalTemp {
    s: String,
}
impl LexicalTemp {
    pub fn new() -> LexicalTemp {
        LexicalTemp { s: String::new() }
    }
    pub fn update(&mut self, c_now: char, c_next: char) -> Option<Lexical> {
        if is_endofline(c_now) {
            return None;
        }
        self.s.push(c_now);
        if is_whitespace(c_next) {
            let res = Lexical::Identifier(Identifier::Name(self.s.clone()));
            self.s = String::new();
            Some(res)
        } else {
            None
        }
    }
}
