pub mod comment;
pub mod identifier;
pub mod keyword;
pub mod literal;
pub mod operator;
pub mod separator;
use comment::Comment;
use identifier::Identifier;
use keyword::Keyword;
use literal::Literal;
use operator::Operator;
use separator::Separator;
pub enum Lexical {
    Identifier(Identifier),
    Keyword(Keyword),
    Separator(Separator),
    Operator(Operator),
    Literal(Literal),
    Comment(Comment),
}
impl Clone for Lexical {
    fn clone(&self) -> Lexical {
        match self {
            Lexical::Identifier(x) => Lexical::Identifier(x.clone()),
            Lexical::Keyword(x) => Lexical::Keyword(x.clone           ()),
            Lexical::Separator(_) => todo!(),
            Lexical::Operator(_) => todo!(),
            Lexical::Literal(_) => todo!(),
            Lexical::Comment(_) => todo!(),
        }
    }
}
impl std::fmt::Debug for Lexical {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Lexical::Identifier(x) => write!(f, "Idn{:?}", x).unwrap(),
            Lexical::Keyword(_) => todo!(),
            Lexical::Separator(_) => todo!(),
            Lexical::Operator(_) => todo!(),
            Lexical::Literal(_) => todo!(),
            Lexical::Comment(_) => todo!(),
        };
        Ok(())
    }
}
