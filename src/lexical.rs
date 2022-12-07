pub mod identifier;
use identifier::Identifier;
pub enum Lexical {
    Identifier(Identifier),
}
impl Clone for Lexical {
    fn clone(&self) -> Lexical {
        match self {
            Lexical::Identifier(x) => Lexical::Identifier(x.clone()),
        }
    }
}
impl std::fmt::Debug for Lexical {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Lexical::Identifier(x) => write!(f, "Idn{:?}", x).unwrap(),
        };
        Ok(())
    }
}
