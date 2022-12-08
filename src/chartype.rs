pub fn is_letter(c: &char) -> bool {
    'a' <= *c && *c <= 'z' || 'A' <= *c && *c <= 'Z' || *c == '_'
}
pub fn is_numeral(c: &char) -> bool {
    '0' <= *c && *c <= '9'
}
pub fn is_separator(c: &char) -> bool {
    "(){{}}[]:;".find(c.clone()) != None
}
pub fn is_operator(c: &char) -> bool {
    "+-*/%=<>,.&|^".find(c.clone()) != None
}
pub fn is_backslash(c: &char) -> bool {
    *c == '\\'
}
pub fn is_hash(c: &char) -> bool {
    *c == '#'
}
pub const END_OF_LINE: char = '\n';
pub fn is_endofline(c: &char) -> bool {
    *c == '\n'
}
pub fn is_whitespace(c: &char) -> bool {
    *c == ' ' || *c == '\n'
}
pub fn _is_space(c: &char) -> bool {
    *c == ' '
}
