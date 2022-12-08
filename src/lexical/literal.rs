pub enum Literal {
    Integer(i32),
    String(String),
    Boolean(bool),
}
impl Clone for Literal {
    fn clone(&self) -> Literal {
        match self {
            Literal::Integer(x) => Literal::Integer(x.clone()),
            Literal::String(x) => Literal::String(x.clone()),
            Literal::Boolean(x) => Literal::Boolean(x.clone()),
        }
    }
}
impl std::fmt::Debug for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Literal::Integer(x) => "Int(".to_string() + &x.to_string() + ")",
                Literal::String(x) => "Str(".to_string() + &x + ")",
                Literal::Boolean(x) => "Bol(".to_string() + &x.to_string() + ")",
            }
        )?;
        Ok(())
    }
}
impl ToString for Literal {
    fn to_string(&self) -> String {
        match self {
            Literal::Integer(x) => x.to_string(),
            Literal::String(x) => x.to_string(),
            Literal::Boolean(x) => {
                if *x {
                    "True".to_string()
                } else {
                    "False".to_string()
                }
            }
        }
    }
}
