pub fn word_type(word: char) -> &'static str {
    if word.is_alphabetic() {
        return "alpha";
    } else if word.is_numeric() {
        return "num";
    } else {
        return "other";
    }
}
