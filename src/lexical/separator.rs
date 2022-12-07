pub enum Separator {
    Colon,
}
impl std::fmt::Debug for Separator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Sep{:?}",
            match self {
                Separator::Colon => ":",
            }
        )?;
        Ok(())
    }
}
pub fn to_sep(s: &str) -> Option<Separator> {
    match s {
        ":" => Some(Separator::Colon),
        _ => None,
    }
}
