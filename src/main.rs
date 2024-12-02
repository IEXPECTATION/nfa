mod nfa;
mod parser;
mod scanner;

fn main() {
    let mut nfa = nfa::NFA::new();

    let postfix_regex = nfa.compile("ab|c");

    for token in postfix_regex {
        println!("{}", token)
    }

}
