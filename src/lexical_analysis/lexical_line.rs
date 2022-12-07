mod lexical_temp;
use crate::{lexical::Lexical, lineunit::LineUnit};
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
        let x = self.lt.update(c_now, c_next);
        match x {
            Some(y) => self.vl.push(y),
            None => (),
        }
        if c_next == '\n' {
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
