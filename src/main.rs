use std::{env, fs};
use description::Description;

mod description;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("echec de rime");
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