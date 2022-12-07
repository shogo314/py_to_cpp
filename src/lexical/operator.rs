pub enum Operator {
    Eq,
}
impl std::fmt::Debug for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Sep{:?}",
            match self {
                Operator::Eq => "Eq",
            }
        )?;
        Ok(())
    }
}
pub fn to_ope(s: &str) -> Option<Operator> {
    match s {
        "==" => Some(Operator::Eq),
        _ => None,
    }
}
