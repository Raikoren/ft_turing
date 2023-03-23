use std::{env, fs};
use description::Description;

mod description;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] === "--help" || args.len() != 3 {
        panic!("usage: ft_turing [-h] jsonfile input\n\n" +
        "positional arguments:\n" +
        "\tjsonfile\t\t\tjson description of the machine\n\n" +
        "\iinput\t\t\tinput of the machine\n\n" +
        "optional arguments:\n" +
        "\t-h, --help\t\t\tshow this help message and exi\n\n");
    }
    let src = &args[1];
    let contents = fs::read_to_string(src).expect("Should have been able to read the file");
    let desc: Description = serde_json::from_str(&contents).expect("couldn't parse the description");
    let input: Vec<char> = args[2].chars().collect();
    run_machine(desc, input);
}

fn run_machine(desc: Description, mut input: Vec<char>) {
    let mut state = String::from(desc.initial);
    let mut head = 0;
    while state != desc.finals[0] {
        let transition = desc.transitions.get(&state).unwrap().iter().find(|&t| t.read == input[head]).unwrap();
        state = transition.to_state.clone();
        input[head] = transition.write;
        match &transition.action {
            Action::RIGHT => head += 1,
            Action::LEFT => head -= 1
        }
        println!("[{}]", input.clone().into_iter().collect::<String>());
    }
}