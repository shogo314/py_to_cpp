mod lexical;
mod lineunit;
mod lexical_analysis;
mod syntactic_analysis;
use lexical_analysis::lexical_analysis;
use syntactic_analysis::syntactic_analysis;
fn main() {
    let mut buffer = String::new();
    while std::io::stdin().read_line(&mut buffer).unwrap() != 0 {}
    let lexical_result = lexical_analysis(&buffer);
    syntactic_analysis(lexical_result);
}
