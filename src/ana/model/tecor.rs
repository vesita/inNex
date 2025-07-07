enum TecorMode {
    edge,
    letters,
    words,
}

struct Tecor {
    begin: [i64; 2],
    end: [i64; 2],

    at: [i64; 2],
    side: [i64; 2],

    ends: Vec<usize>,

    mode: TecorMode,
}

impl Tecor {
    fn new() -> Self {
        Self {
            begin: [0, 0],
            end: [0, 0],

            at: [0, 0],
            side: [0, 0],

            mode: TecorMode::edge,
        }
    }

    fn rec(&mut self, begin: [i64; 2], end: [i64; 2]) {
        self.begin = begin;
        self.end = end;
    }
}
