struct Wopol {
    contents: Vec<Vec<String>>,
    tec: Tecor,
}

impl Wopol {
    fn new() -> Self {
        Self {
            contents: Vec::new(),
            ends: Vec::new(),
        }
    }

    fn read(&mut self, filename: &str) {
        use std::fs;
        let content = fs::read_to_string(filename).expect("Failed to read file");
        self.tec = Tecor::new();
        for (row, line) in content.lines().enumerate() {
            let mut chars = line.chars();
            while let Some(c) = chars.next() {
                self.contents.push(vec![c.to_string()]);
            }
            self.tec.ends.push(line.len());
        }
        self.tec.end = [row, tec.ends[row]];
    }
}
