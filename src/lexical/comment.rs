pub enum Comment {
    Name(String),
}
impl Clone for Comment {
    fn clone(&self) -> Comment {
        match self {
            Comment::Name(x) => Comment::Name(x.clone()),
        }
    }
}
impl std::fmt::Debug for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Comment::Name(x) => write!(f, "Idn{:?}", x).unwrap(),
        };
        Ok(())
    }
}
