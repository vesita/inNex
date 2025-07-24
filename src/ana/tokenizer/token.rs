struct inNex_token {
    token: str,
}

struct token_stream {
    pn: Pano,
    at: usize,
    tokens: queue<inNex_token>,
}

impl token_stream {
    fn new() -> Self {
        token_stream {
            pn: Pano::On,
            at: 0,
            tokens: queue::new(),
        }
    }

    fn load_file(&mut self, file_path: &str) {
        let mut travel = Travel::new();
        travel.read_file(file_path);

        while let Some(token) = travel.get_next() {
            self.tokens.push(inNex_token {
                token: token.to_string(),
            });
        }
    }

    pub fn topano(&mut self, tar: Pano) {
        self.pn = tar;
    }

    pub fn move_to(&mut self, offset: usize) {
        match self.pn {
            Pano::On => {
                self.at += offset;
                if self.at >= self.tokens.len() {
                    self.at = self.tokens.len() - 1;
                }
            }
            Pano::Off => {
                self.at -= offset;
                if self.at >= self.tokens.len() {
                    self.at = 0;
                }
            }
        }
    }

    pub fn get_token(&self) -> Option<String> {
        self.tokens.get(self.at).map(|token| token.to_string())
    }
}
