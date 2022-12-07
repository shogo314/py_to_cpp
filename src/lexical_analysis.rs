mod lexical_line;
use crate::{lineunit::LineUnit, chartype::END_OF_LINE};
use lexical_line::LexicalLine;
pub fn lexical_analysis(s: &str) -> Vec<LineUnit> {
    let mut res: Vec<LineUnit> = Vec::new();
    let mut sc_: Vec<char> = s.chars().collect();
    sc_.push(END_OF_LINE);
    let sc = sc_;
    let mut ll: LexicalLine = LexicalLine::new();
    for i in 0..sc.len() - 1 {
        match ll.update(sc[i], sc[i + 1]) {
            Some(y) => res.push(y),
            None => (),
        }
    }
    res
}
