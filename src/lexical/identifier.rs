pub enum Identifier {
    Name(String),
}
impl Clone for Identifier {
    fn clone(&self) -> Identifier {
        match self {
            Identifier::Name(x) => Identifier::Name(x.clone()),
        }
    }
}
impl std::fmt::Debug for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Identifier::Name(x) => write!(f, "Idn{:?}", x).unwrap(),
        };
        Ok(())
    }
}
