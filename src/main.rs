use std::env::{self};

mod nfa;
mod parser;
mod scanner;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: ./nfa regex input");
        return;
    }

    let regex = &args[1];
    let input = &args[2];
    let mut nfa = nfa::NFA::new();
    nfa.compile(&regex);

    println!("{}", nfa.accept(&input));

    // nfa.compile("(0|1|2|3|4|5|6|7|8|9)*@qq.com");
    // nfa.compile("(a|b)*c");
    // nfa._print_transitions();
    // println!("{}", nfa.accept("wuchx19@gmail.com"));
    // println!("{}", nfa.accept("1120644453@qq.com"));
    // println!("{}", nfa.accept("@qq.com"));
    // println!("{}", nfa.accept("w@qq.com"));

    // let mut parser = parser::PostfixParser::new("(0|1|2|3|4|5|6|7|8|9)@qq.com");
    // let positions = parser.parse();

    // for position in positions {
    //     println!("{}", position);
    // }
}
