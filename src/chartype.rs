pub fn _is_letter(c: char) -> bool {
    'a' <= c && c <= 'z' || 'A' <= c && c <= 'Z' || c == '_'
}
pub fn _is_numeral(c: char) -> bool {
    '0' <= c && c <= '9'
}
pub fn _is_separator(c: char) -> bool {
    "(){{}}[]:;".find(c) != None
}
pub fn _is_operator(c: char) -> bool {
    "+-*/%=<>,.".find(c) != None
}
pub fn _is_backslash(c: char) -> bool {
    c == '\\'
}
pub fn _is_hash(c: char) -> bool {
    c == '#'
}
pub const END_OF_LINE: char = '\n';
pub fn is_endofline(c: char) -> bool {
    c == '\n'
}
pub fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\n'
}
pub fn _is_space(c: char) -> bool {
    c == ' '
}
