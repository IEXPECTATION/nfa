use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    fmt::Display,
    rc::{self, Rc},
};

const EPSILON: char = 'ε';

#[derive(Default, Debug)]
struct State {
    accepting: bool,
    next: HashMap<char, Rc<RefCell<State>>>,
}

struct Fragment {
    start: Rc<RefCell<State>>,
    end: Rc<RefCell<State>>,
}

#[derive(Default)]
struct Automaton {
    start_state: Rc<RefCell<State>>,
    operator: Vec<char>,
    state: Vec<Rc<RefCell<Fragment>>>,
}

impl Automaton {
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
        if self.state.len() == 1 {
            // If there is only a start state, return immediately.
            // It looks lile "|bcd"
            return;
        }

        self.operator.push('|');
    }

    fn concat(&mut self, c: char, accepting: bool) {
        if (self.state.len() == 0) {
            let state: Rc<RefCell<State>> = Default::default();
            // self.state.push(state.clone());
            return;
        }

        if let Some(top) = self.state.last_mut() {
            let mut state = top.borrow_mut();

            let newState: Rc<RefCell<State>> = Default::default();
            // state.next.insert(c, newState.clone());
        }
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
    let mut nfa = NFA::new("abc");
    nfa.compile();

    // for item in nfa.machine.state {
    //     let state = item.borrow();
    //     println!("{state:?}");
    // }
}
