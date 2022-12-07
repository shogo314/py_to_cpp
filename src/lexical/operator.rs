pub enum Operator {
    Eq,
    Ne,
    Le,
    Lt,
    Ge,
    Gt,
    LNot,
    LAnd,
    LOr,
    Add,
    Sub,
    Mul,
    Truediv,
    Floordiv,
    Mod,
    Pow,
    And,
    Or,
    Xor,
    Contains,
}
impl std::fmt::Debug for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Sep({})",
            match self {
                Operator::Eq => "Eq",
                Operator::Ne => "Ne",
                Operator::Le => "Le",
                Operator::Lt => "Lt",
                Operator::Ge => "Ge",
                Operator::Gt => "Gt",
                Operator::LNot => "LNot",
                Operator::LAnd => "LAnd",
                Operator::LOr => "LOr",
                Operator::Add => "Add",
                Operator::Sub => "Sub",
                Operator::Mul => "Mul",
                Operator::Truediv => "Truediv",
                Operator::Floordiv => "Floordiv",
                Operator::Mod => "Mod",
                Operator::Pow => "Pow",
                Operator::And => "And",
                Operator::Or => "Or",
                Operator::Xor => "Xor",
                Operator::Contains => "Contains",
            }
        )?;
        Ok(())
    }
}
impl Clone for Operator {
    fn clone(&self) -> Operator {
        match self {
            Operator::Eq => Operator::Eq,
            Operator::Ne => Operator::Ne,
            Operator::Le => Operator::Le,
            Operator::Lt => Operator::Lt,
            Operator::Ge => Operator::Ge,
            Operator::Gt => Operator::Gt,
            Operator::LNot => Operator::LNot,
            Operator::LAnd => Operator::LAnd,
            Operator::LOr => Operator::LOr,
            Operator::Add => Operator::Add,
            Operator::Sub => Operator::Sub,
            Operator::Mul => Operator::Mul,
            Operator::Truediv => Operator::Truediv,
            Operator::Floordiv => Operator::Floordiv,
            Operator::Mod => Operator::Mod,
            Operator::Pow => Operator::Pow,
            Operator::And => Operator::And,
            Operator::Or => Operator::Or,
            Operator::Xor => Operator::Xor,
            Operator::Contains => Operator::Contains,
        }
    }
}
pub fn is_ope(s: &str) -> bool {
    match s {
        "==" | "!=" | "<=" | "<" | ">=" | ">" | "not" | "and" | "or" | "+" | "-" | "*" | "/"
        | "//" | "%" | "**" | "&" | "|" | "^" | "in" => true,
        _ => false,
    }
}

pub fn to_ope(s: &str) -> Option<Operator> {
    match s {
        "==" => Some(Operator::Eq),
        "!=" => Some(Operator::Ne),
        "<=" => Some(Operator::Le),
        "<" => Some(Operator::Lt),
        ">=" => Some(Operator::Ge),
        ">" => Some(Operator::Gt),
        "not" => Some(Operator::LNot),
        "and" => Some(Operator::LAnd),
        "or" => Some(Operator::LOr),
        "+" => Some(Operator::Add),
        "-" => Some(Operator::Sub),
        "*" => Some(Operator::Mul),
        "/" => Some(Operator::Truediv),
        "//" => Some(Operator::Floordiv),
        "%" => Some(Operator::Mod),
        "**" => Some(Operator::Pow),
        "&" => Some(Operator::And),
        "|" => Some(Operator::Or),
        "^" => Some(Operator::Xor),
        "in" => Some(Operator::Contains),
        _ => None,
    }
}
