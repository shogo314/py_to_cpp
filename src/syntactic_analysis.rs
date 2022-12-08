use crate::lexical::Lexical;
use crate::lineunit::LineUnit;
pub fn syntactic_analysis(lexicalxresult: Vec<LineUnit>) {
    for lu in lexicalxresult {
        println!("{:?}", lu);
        for i in 0..lu.value.len() {
            match &lu.value[i] {
                Lexical::Identifier(x) => eprint!("\x1b[94;1m{}\x1b[m", x.to_string()),
                Lexical::Keyword(x) => eprint!("\x1b[31;1m{}\x1b[m", x.to_string()),
                Lexical::Separator(x) => eprint!("\x1b[93;1m{}\x1b[m", x.to_string()),
                Lexical::Operator(x) => eprint!("\x1b[97;1m{}\x1b[m", x.to_string()),
                Lexical::Literal(x) => eprint!("\x1b[37;1m{}\x1b[m", x.to_string()),
                Lexical::Comment(x) => eprint!("\x1b[33;1m{}\x1b[m", x.to_string()),
            }
            if i < lu.value.len() - 1 {
                eprint!(" ");
            } else {
                eprintln!("");
            }
        }
    }
}
