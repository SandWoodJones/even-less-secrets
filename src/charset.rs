pub fn get_random_char() -> char {
    let char_table: Vec<char> = ('!'..='~').collect(); // Includes almost whole range of ASCII printable characters (33 to 126)

    return char_table[fastrand::usize(..char_table.len())];
}
