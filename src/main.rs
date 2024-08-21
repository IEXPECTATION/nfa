use std::{collections::HashMap, fmt::Display, vec};

enum Operand {
    Epsilon,
    Character(char),
}

struct State {
    operand: Operand,
    next: HashMap<char, State>,
    accpect: bool,
}

impl State {}

struct Automaton {
    startState: Option<State>,
}

impl Automaton {
    fn compile(&mut self, regex: &str) {
        // Traverse the `regex` to generate the nfa.
        let s = regex.to_string();
        let mut chars = s.chars();

        let operator: Vec<State> = Vec::new();
        let character: Vec<State> = Vec::new();

        while let Some(c) = chars.next() {
            match c {
                '|' => (),
                '+' => (),
                '*' => (),
                '?' => (),
                c => {
                    println!("{c}");
                }
            };
        }
    }

    fn accept(&self, text: &str) -> String {
        // Run the nfa to match the text.
        String::from("accept!")
    }
}

struct NFA {
    regex: String,
    machine: Automaton,
}

impl NFA {
    fn new(regex: &str) -> Self {
        NFA {
            regex: String::from(regex),
            machine: Automaton { startState: None },
        }
    }

    fn from(regex: &str) -> Self {
        let mut nfa = NFA::new(regex);
        nfa.compile();
        nfa
    }

    fn set_regex(&mut self, regex: &str) {
        self.regex = String::from(regex);
    }

    fn compile(&mut self) {
        self.machine.compile(&self.regex);
    }

    fn accept(&self, text: &str) -> String {
        self.machine.accept(text)
    }

    fn fmt(self) -> String {
        return "".to_string();
    }
}

fn main() {
    let mut nfa = NFA::new("abc|你好+");
    nfa.compile();
}
