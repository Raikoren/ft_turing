use serde::Deserialize;
use std::{collections::HashMap};
use serde_json::Error;

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

impl Description {
    pub fn new(json: &String) -> Result<Self, Error> {
        let desc: Description = serde_json::from_str(&json)?;
        Ok(desc)
    }

    pub fn check(&self) -> Result<(), &str> {
        if !self.alphabet.contains(&self.blank) {
            return Err("blank character isn't part of the alphabet");
        }
        if !self.states.contains(&self.initial) {
            return Err("initial state isn´t part of the states list");
        }
        if !self.finals.iter().all(|f| self.states.contains(f)) {
            return Err(
                "at least one final state is missing from the states list",
            );
        }
        if !self.transitions.iter().all(|t| self.states.contains(t.0)) {
            return Err(
                "at least one transition is missing from the states list",
            );
        }
        if !self
            .transitions
            .iter()
            .all(|el| el.1.iter().all(|t|
                self.states.contains(&t.to_state) && self.alphabet.contains(&t.read) && self.alphabet.contains(&t.write)))
        {
            return Err(
                "at least one transition isn´t valid",
            );
        }
        Ok(())
    }

    pub fn run(&self, mut input: Vec<char>) {
        let mut state = self.initial.clone();
        let mut head = 0;
        println!("{} :", self.name);
        while !self.finals.contains(&state) {
            let transition = self
                .transitions
                .get(&state)
                .unwrap()
                .iter()
                .find(|&t| t.read == input[head])
                .unwrap();
            state = transition.to_state.clone();
            input[head] = transition.write;
            match &transition.action {
                Action::RIGHT => head += 1,
                Action::LEFT => head -= 1,
            }
            println!("[{}]", input.clone().into_iter().collect::<String>());
        }
    }
}
