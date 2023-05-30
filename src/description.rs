use colored::*;
use serde::Deserialize;
use serde_json::Error;
use std::collections::HashMap;

#[derive(Deserialize)]
pub enum Action {
    RIGHT,
    LEFT,
}

#[derive(Deserialize)]
pub struct Transition {
    read: char,
    to_state: String,
    write: char,
    action: Action,
}

#[derive(Deserialize)]
pub struct Description {
    name: String,
    alphabet: Vec<char>,
    blank: char,
    states: Vec<String>,
    initial: String,
    finals: Vec<String>,
    transitions: HashMap<String, Vec<Transition>>,
}

fn print_tape(tape: &Vec<char>, head: &usize) {
    tape.iter().enumerate().for_each(|(i, c)| {
        if i == 0 {
            print!("[");
        }
        if i == *head {
            print!("{}", c.to_string().red().bold());
        } else {
            print!("{c}");
        }
        if i == tape.len() - 1 {
            print!("]");
        }
    });
}

fn print_transition(state: &String, transition: &Transition) {
    println!(
        " ({}, {}) -> ({}, {}, {})",
        state,
        transition.read,
        transition.to_state,
        transition.write,
        match transition.action {
            Action::RIGHT => "RIGHT",
            Action::LEFT => "LEFT",
        }
    )
}

impl Description {
    pub fn new(json: &String) -> Result<Self, Error> {
        let desc: Description = serde_json::from_str(&json)?;
        Ok(desc)
    }

    pub fn check(&self, input: &Vec<char>) -> Result<(), &str> {
        if !self.alphabet.contains(&self.blank) {
            return Err("blank character isn't part of the alphabet");
        }
        if !self.states.contains(&self.initial) {
            return Err("initial state isn´t part of the states list");
        }
        if !self.finals.iter().all(|f| self.states.contains(f)) {
            return Err("at least one final state is missing from the states list");
        }
        if !self.transitions.iter().all(|t| self.states.contains(t.0)) {
            return Err("at least one transition is missing from the states list");
        }
        if !self.transitions.iter().all(|el| {
            el.1.iter().all(|t| {
                self.states.contains(&t.to_state)
                    && self.alphabet.contains(&t.read)
                    && self.alphabet.contains(&t.write)
            })
        }) {
            return Err("at least one transition isn´t valid");
        }
        if !input.iter().all(|c| self.alphabet.contains(c)) {
            return Err("input does not coincide with the description alphabet");
        }
        Ok(())
    }

    pub fn start(&self, tape: Vec<char>) -> Result<(), &str> {
        println!("running : {}", self.name);
        self.run(&self.initial, 0, tape)
    }

    fn run(&self, state: &String, head: usize, mut tape: Vec<char>) -> Result<(), &str> {
        print_tape(&tape, &head);
        if self.finals.contains(&state) {
            return Ok(());
        }
        let transition = match self
            .transitions
            .get(state)
            .unwrap()
            .iter()
            .find(|&t| t.read == tape[head])
        {
            Some(t) => t,
            None => return Err("machine stopped : no transition found for current character"),
        };
        tape[head] = transition.write;
        print_transition(state, transition);
        let head = match &transition.action {
            Action::RIGHT => {
                if head == tape.len() {
                    tape.push(self.blank);
                }
                head + 1
            }
            Action::LEFT => {
                if head == 0 {
                    tape.insert(0, self.blank);
                    0
                } else {
                    head - 1
                }
            }
        };
        self.run(
            &transition.to_state,
            head,
            tape,
        )
    }
}
