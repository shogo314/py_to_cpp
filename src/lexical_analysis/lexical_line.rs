mod lexical_temp;
use crate::{chartype::is_endofline, lexical::Lexical, lineunit::LineUnit};
use lexical_temp::LexicalTemp;
pub struct LexicalLine {
    vl: Vec<Lexical>,
    lt: LexicalTemp,
}
impl LexicalLine {
    pub fn new() -> LexicalLine {
        LexicalLine {
            vl: Vec::new(),
            lt: LexicalTemp::new(),
        }
    }
    pub fn update(&mut self, c_now: char, c_next: char) -> Option<LineUnit> {
        match self.lt.update(c_now, c_next) {
            Some(y) => self.vl.push(y),
            None => (),
        }
        if is_endofline(c_next) {
            let res = Some(LineUnit {
                indent: 0,
                value: self.vl.clone(),
            });
            self.vl.clear();
            res
        } else {
            None
        }
    }
}
