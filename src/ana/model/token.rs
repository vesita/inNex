enum token_state {
    init,
    ana,
}

struct Token {
    the: String,
    state: token_state,
    tag: String,
}
