mod nfa;
mod scanner;
mod tokenizer;

use crate::tokenizer::Token;

use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
    u32, vec,
};

fn main() {
    let mut nfa = nfa::NFA::new();

    // let ret = nfa.accept("a").unwrap();
    // print!("{}", ret);
}
