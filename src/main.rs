use std::{env, fs, error::Error};
use description::Description;

use crate::description::Action;

mod description;

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 || args[1] == "help" {
        print!("usage: ft_turing jsonfile input\n
positional arguments:
\tjsonfile\t\tjson description of the machine
\tinput\t\t\tinput of the machine\n
optional arguments:
\thelp\t\t\tshow this help message and exit");
        return Ok(())
    }
    let src = &args[1];
    let contents = fs::read_to_string(src).expect("Should have been able to read the file");
    let desc: Description = serde_json::from_str(&contents).expect("couldn't parse the description");
    let input: Vec<char> = args[2].chars().collect();
    let desc = desc.check_description()?;
    run_machine(desc, input);
    args.iter().for_each(|a| println!("{a}"));
    Ok(())
}

fn run_machine(desc: Description, mut input: Vec<char>) {
    let mut state = String::from(desc.initial);
    let mut head = 0;
    while !desc.finals.contains(&state) {
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