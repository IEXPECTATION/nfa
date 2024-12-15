use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    mem,
};

use crate::parser::{self, Token};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub(crate) enum State {
    Epsilon,
    Input(char),
}

struct Fragment {
    from: u32,
    to: u32,
}

#[derive(Debug)]
struct Transition {
    transitions: HashMap<(/* from */ u32, State), /* to */ Vec<u32>>,
}

impl Transition {
    fn new() -> Self {
        Transition {
            transitions: HashMap::new(),
        }
    }

    fn add(&mut self, from: u32, s: State, to: u32) {
        self.transitions.entry((from, s)).or_default().push(to);
    }

    fn patch(&mut self, from: u32, s: State, to: u32) {}

    fn next_states(&self, from: u32, state: State) -> Option<&Vec<u32>> {
        return self.transitions.get(&(from, state));
    }

    fn epsilon_states(&self, from: u32) -> Option<&Vec<u32>> {
        return self.next_states(from, State::Epsilon);
    }

    fn _print_transition(&self) {
        for transtion in self.transitions.iter() {
            println!("({:?}) -> {:?}", transtion.0, transtion.1)
        }
    }
}

pub(crate) struct NFA {
    start_state: Option<u32>,
    accepting_state: Vec<u32>,
    transitions: Transition,
    id: u32,
}

impl Display for NFA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Debug for NFA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!();
    }
}

impl NFA {
    pub(crate) fn new() -> Self {
        NFA {
            start_state: None,
            accepting_state: Vec::new(),
            transitions: Transition::new(),
            id: 0,
        }
    }

    pub(crate) fn compile(&mut self, regex: &str) {
        let mut parser = parser::PostfixParser::new(regex);
        let postfixs = parser.parse();

        let mut stack = Vec::<Fragment>::new();

        for token in postfixs {
            match token {
                Token::Char(c) => {
                    let v1 = self.new_id();
                    let v2 = self.new_id();

                    self.transitions.add(v1, State::Input(c), v2);

                    stack.push(Fragment { from: v1, to: v2 });
                }
                Token::Or => {
                    let f2 = stack.pop().unwrap();
                    let f1 = stack.pop().unwrap();

                    let v1 = self.new_id();
                    let v2 = self.new_id();

                    /*
                           +- f1 -+
                       v1 -|      |- v2
                           +- f2 -+

                       Build a connection from v1 to f1 and f2.
                    */
                    self.transitions.add(v1, State::Epsilon, f1.from);
                    self.transitions.add(v1, State::Epsilon, f2.from);

                    // Build a connect from f1 and f2 to v2.

                    self.transitions.add(f1.to, State::Epsilon, v2);
                    self.transitions.add(f2.to, State::Epsilon, v2);

                    stack.push(Fragment { from: v1, to: v2 });
                }
                Token::Concatenate => {
                    let f2 = stack.pop().unwrap();
                    let f1 = stack.pop().unwrap();

                    self.transitions.add(f1.to, State::Epsilon, f2.from);

                    stack.push(Fragment {
                        from: f1.from,
                        to: f2.to,
                    })
                }
                Token::Quantifier(min, max) => match min {
                    0 => {
                        if max == 1 {
                            /*
                               ?
                                   +-    f1    -+
                               v1 -|            |- v2
                                   +------------+
                            */

                            let f1 = stack.pop().unwrap();

                            let v1 = self.new_id();
                            let v2 = self.new_id();

                            self.transitions.add(v1, State::Epsilon, f1.from);
                            self.transitions.add(f1.to, State::Epsilon, v2);
                            self.transitions.add(v1, State::Epsilon, v2);

                            stack.push(Fragment { from: v1, to: v2 });
                        } else {
                            /*
                                *
                                |-----------------+
                                    +- f1 --------|
                                v1 -|      |- v2
                                    +------|
                            */

                            let f1 = stack.pop().unwrap();

                            let v1 = self.new_id();
                            let v2 = self.new_id();

                            self.transitions.add(v1, State::Epsilon, f1.from);
                            self.transitions.add(v1, State::Epsilon, v2);
                            self.transitions.add(f1.to, State::Epsilon, v1);

                            stack.push(Fragment { from: v1, to: v2 });
                        }
                    }
                    1 => {
                        /*
                           +
                           |--------+
                           v1 - f1 -|- v2

                        */

                        let f1 = stack.pop().unwrap();

                        let v1 = self.new_id();
                        let v2 = self.new_id();

                        self.transitions.add(v1, State::Epsilon, f1.from);
                        self.transitions.add(f1.from, State::Epsilon, v1);
                        self.transitions.add(f1.to, State::Epsilon, v2);

                        stack.push(Fragment { from: v1, to: v2 });
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        let top = stack.pop().unwrap();

        self.start_state = Some(top.from);
        self.accepting_state.push(top.to);
    }

    pub(crate) fn accept(&self, input: &str) -> bool {
        if self.start_state == None {
            return false;
        }

        let mut current_states: Vec<u32> = Vec::new();
        let mut next_states: Vec<u32> = Vec::new();

        current_states.push(self.start_state.unwrap());

        for c in input.chars().fuse() {
            while let Some(from) = current_states.pop() {
                let nexts = self.transitions.epsilon_states(from);
                if let Some(states) = nexts {
                    current_states.extend(states);
                }

                let nexts = self.transitions.next_states(from, State::Input(c));
                if let Some(states) = nexts {
                    next_states.extend(states);
                }
            }

            mem::swap(&mut current_states, &mut next_states);
        }

        let mut accepting: Vec<u32> = Vec::new();
        while !current_states.is_empty() {
            let from = current_states.pop().unwrap();

            if self.accepting_state.contains(&from) {
                accepting.push(from);
            }

            let nexts = self.transitions.epsilon_states(from);
            if let Some(states) = nexts {
                current_states.extend(states);
            }
        }

        match accepting.len() {
            0 => false,
            _ => true,
        }
    }

    fn new_id(&mut self) -> u32 {
        let id = self.id;
        self.id += 1;
        id
    }

    pub(crate) fn _print_transitions(&self) {
        self.transitions._print_transition();
    }
}
