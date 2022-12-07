use crate::lexical::Lexical;
pub struct LexicalTemp {
    s: String,
}
impl LexicalTemp {
    pub fn new() -> LexicalTemp {
        LexicalTemp { s: String::new() }
    }
    pub fn update(&mut self, c_now: char, c_next: char) -> Option<Lexical> {
        if c_now == '\n' {
            return None;
        }
        self.s.push(c_now);
        if c_next == ' ' || c_next == '\n' {
            let res = Lexical::Identifier(self.s.clone());
            self.s = String::new();
            Some(res)
        } else {
            None
        }
    }
}
