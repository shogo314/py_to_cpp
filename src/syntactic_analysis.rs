use crate::lineunit::LineUnit;
pub fn syntactic_analysis(lexical_result: Vec<LineUnit>) {
    for lu in lexical_result {
        println!("{:?}", lu);
    }
}
