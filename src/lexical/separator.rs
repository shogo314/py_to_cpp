pub enum Separator {
    LeftParenthesis,
    RightParenthesis,
    LeftSquareBracket,
    RightSquareBracket,
    LeftCurlyBracket,
    RightCurlyBracket,
    Colon,
    Semicolon,
}
impl std::fmt::Debug for Separator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Sep{:?}",
            match self {
                Separator::LeftParenthesis => "(",
                Separator::RightParenthesis => ")",
                Separator::LeftSquareBracket => "[",
                Separator::RightSquareBracket => "]",
                Separator::LeftCurlyBracket => "{",
                Separator::RightCurlyBracket => "}",
                Separator::Colon => ":",
                Separator::Semicolon => ";",
            }
        )?;
        Ok(())
    }
}
impl Clone for Separator {
    fn clone(&self) -> Separator {
        match self {
            Separator::LeftParenthesis => Separator::LeftParenthesis,
            Separator::RightParenthesis => Separator::RightParenthesis,
            Separator::LeftSquareBracket => Separator::LeftSquareBracket,
            Separator::RightSquareBracket => Separator::RightSquareBracket,
            Separator::LeftCurlyBracket => Separator::LeftCurlyBracket,
            Separator::RightCurlyBracket => Separator::RightCurlyBracket,
            Separator::Colon => Separator::Colon,
            Separator::Semicolon => Separator::Semicolon,
        }
    }
}
pub fn to_sep(s: &str) -> Option<Separator> {
    match s {
        "(" => Some(Separator::LeftParenthesis),
        ")" => Some(Separator::RightParenthesis),
        "[" => Some(Separator::LeftSquareBracket),
        "]" => Some(Separator::RightSquareBracket),
        "{" => Some(Separator::LeftCurlyBracket),
        "}" => Some(Separator::RightCurlyBracket),
        ":" => Some(Separator::Colon),
        ";" => Some(Separator::Semicolon),
        _ => None,
    }
}
