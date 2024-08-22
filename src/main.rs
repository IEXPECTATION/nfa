use std::{
    collections::HashMap,
    rc::{self, Rc},
};

const EPSILON: char = 'ε';

#[derive(Default)]
struct State {
    next: HashMap<char, State>,
    accept: bool,
}

impl State {
    fn new(accept: bool) -> Self {
        Self {
            next: HashMap::new(),
            accept: accept,
        }
    }
}

#[derive(Default)]
struct Automaton<'a> {
    start_state: &'a State,
    operator: Vec<&'a State>,
    character: Vec<&'a State>,
}

impl Automaton<'_> {
    fn compile(&mut self, regex: &str) {
        // Traverse the `regex` to generate the nfa.
        let s = regex.to_string();
        let mut chars = s.chars();

        while let Some(c) = chars.next() {
            match c {
                '|' => self.or(),
                '+' => (),
                '*' => (),
                '?' => (),
                c => {
                    self.concat(c, false);
                }
            };
        }
    }

    fn accept(&self, text: &str) -> String {
        // Run the nfa to match the text.
        String::from("accept!")
    }

    fn or(&mut self) {
        ()
    }

    fn concat(&mut self, c: char, accpect: bool) {
        if self.character.len() == 0 {
            self.character.push(self.start_state);
            self.start_state.next.insert('c', Default::default());
        }

        if let Some(top) = self.character.last_mut() {
            // top.next.insert(c, State::new(false));
        }
    }
}

struct NFA<'a> {
    regex: String,
    machine: Automaton<'a>,
}

impl NFA<'_> {
    fn new(regex: &str) -> Self {
        NFA {
            regex: String::from(regex),
            machine: Default::default(),
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

    pub fn compile(&mut self) {
        self.machine.compile(&self.regex);
    }

    fn accept(&self, text: &str) -> String {
        self.machine.accept(text)
    }

    fn test(&self, regex: &str, text: &str) -> String {
        String::new()
    }
}

fn main() {
    // let mut nfa = NFA::new("abc|你好+");
    // nfa.compile();
}
