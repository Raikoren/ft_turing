use core::fmt;
use std::{collections::HashMap, error::Error};
use serde::{Deserialize};

#[derive(Debug)]
pub struct MyError
{
    details: String,
}

impl MyError
{
    fn new(msg: &str) -> MyError
     {
        MyError
         { details: msg.to_string() }
    }
}

impl fmt::Display for MyError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for MyError
{
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Deserialize)]
pub enum Action {
    RIGHT,
    LEFT
}

#[derive(Deserialize)]
pub struct Transition {
    pub read: char,
    pub to_state: String,
    pub write: char,
    pub action: Action
}

#[derive(Deserialize)]
pub struct Description {
    pub name: String,
    pub alphabet: Vec<char>,
    pub blank: char,
    pub states: Vec<String>,
    pub initial: String,
    pub finals: Vec<String>,
    pub transitions: HashMap<String, Vec<Transition>>
}

impl Description {
    pub fn check_description(self) -> Result<Self, MyError> {
        if !(self.alphabet.contains(&self.blank)) {
            return Err(MyError
                ::new("blank character isn't part of the alphabet"))
        }
        if !(self.states.contains(&self.initial)) {
            return Err(MyError
                ::new("initial state isn´t part of the states list"))
        }
        if !(self.finals.iter().all(|f| self.states.contains(f))) {
            return Err(MyError
                ::new("at least one final state is missing from the states list"))
        }
        if !(self.transitions.iter().all(|t| self.states.contains(t.0))) {
            return Err(MyError
                ::new("at least one transition is missing from the states list"))
        }
        if !(self.transitions.iter().all(|t| t.1.iter().all(|q| self.states.contains(&q.to_state)))) {
            return Err(MyError
                ::new("at least one to_state value is missing from the states list"))
        }
        Ok(self)
    }
}