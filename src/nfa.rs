use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    fmt::Display,
};

use crate::parser::{self, Token};

#[derive(Debug)]
enum Regex {}

enum State {
    EPSILON,
    INPUT(Regex),
}

struct Fragment {
    start: u32,
    end: u32,
}

pub(crate) struct NFA {
    start_state: Option<u32>,
    states: Vec<State>,
    accepting_state: Vec<u32>,
    transitions: HashMap<(u32, State), HashSet<u32>>,
}

impl NFA {}

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
            states: vec![],
            accepting_state: vec![],
            transitions: HashMap::<(u32, State), HashSet<u32>>::new(),
        }
    }

    pub(crate) fn compile(&mut self, regex: &str) -> Vec<Token> {
        let mut parser = parser::PostfixParser::new(regex);
        return parser.parse();
    }

    fn or(&mut self) {}

    fn token(&mut self, c: char) {}

    fn accept(&self, input: &str) -> Option<bool> {
        None
    }
}
