enum State {}

struct NFA {
    pattern: String,
}

impl NFA {
    fn new() -> Self {
        NFA {
            pattern: String::new(),
        }
    }

    fn push_str(&mut self, pattern: &str) {
        self.pattern.push_str(pattern);
    }

    fn compile(&mut self) {}

    fn match_str(&self, text: &'static str) {}

    fn match_string(&self, text: &String) {}
}

impl From<&str> for NFA {
    fn from(value: &str) -> Self {
        NFA {
            pattern: String::from(value),
        }
    }
}

impl From<String> for NFA {
    fn from(value: String) -> Self {
        NFA { pattern: value }
    }
}

fn main() {
    let a = NFA::new();
}
