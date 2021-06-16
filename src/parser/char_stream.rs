pub struct CharStream {
    pub code: Vec<char>,
    pub index: usize,
    pub eof: bool
}

impl CharStream {
    pub fn peek (&self) -> char {
        self.code[self.index]
    }

    pub fn read (&mut self) -> char {
        let c = self.code[self.index];

        if self.index >= self.code.len() - 1 {
            self.eof = true;
            // Continually reads the final char once eof
            self.index -= 1;
        } else {
            self.index += 1;
        }

        c
    }

    pub fn new (code: String) -> CharStream {
        CharStream {
            code: code.chars().collect(),
            index: 0,
            eof: false
        }
    }
}
