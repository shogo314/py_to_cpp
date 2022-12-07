use crate::lexical::Lexical;
pub struct LineUnit {
    pub indent: i32,
    pub value: Vec<Lexical>,
}
impl std::fmt::Debug for LineUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:<4}{:?}", self.indent, self.value)?;
        Ok(())
    }
}
