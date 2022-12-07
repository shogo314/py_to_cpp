pub enum Keyword {
    Break,
    Continue,
    If,
    Elif,
    Else,
    For,
    In,
    While,
    Return,
}
impl std::fmt::Debug for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Key{:?}",
            match self {
                Keyword::Break => "Break",
                Keyword::Continue => "Continue",
                Keyword::If => "If",
                Keyword::Elif => "Elif",
                Keyword::Else => "Else",
                Keyword::For => "For",
                Keyword::In => "In",
                Keyword::While => "While",
                Keyword::Return => "Return",
            }
        )?;
        Ok(())
    }
}
impl Clone for Keyword {
    fn clone(&self) -> Keyword {
        match self {
            Keyword::Break => Keyword::Break,
            Keyword::Continue => Keyword::Continue,
            Keyword::If => Keyword::If,
            Keyword::Elif => Keyword::Elif,
            Keyword::Else => Keyword::Else,
            Keyword::For => Keyword::For,
            Keyword::In => Keyword::In,
            Keyword::While => Keyword::While,
            Keyword::Return => Keyword::Return,
        }
    }
}
pub fn is_key(s: &str) -> bool {
    match s {
        "break" | "continue" | "if" | "elif" | "else" | "for" | "in" | "while" | "return" => true,
        _ => false,
    }
}

pub fn to_key(s: &str) -> Option<Keyword> {
    match s {
        "break" => Some(Keyword::Break),
        "continue" => Some(Keyword::Continue),
        "if" => Some(Keyword::If),
        "elif" => Some(Keyword::Elif),
        "else" => Some(Keyword::Else),
        "for" => Some(Keyword::For),
        "in" => Some(Keyword::In),
        "while" => Some(Keyword::While),
        "return" => Some(Keyword::Return),
        _ => None,
    }
}
