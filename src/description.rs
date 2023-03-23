use core::fmt;
use std::{collections::HashMap, error::Error};
use serde::{Deserialize};

#[derive(Debug)]
struct DescError {
    details: String,
}

impl DescError {
    fn new(msg: &str) -> DescError {
        DescError { details: msg.to_string() }
    }
}

impl fmt::Display for DescError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for DescError {
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
    name: String,
    alphabet: Vec<char>,
    blank: char,
    pub states: Vec<String>,
    pub initial: String,
    pub finals: Vec<String>,
    pub transitions: HashMap<String, Vec<Transition>>
}

impl Description {
    pub fn check_description(self) -> Result<Self, DescError> {
        if !(self.alphabet.contains(&self.blank)) {
            return Err(DescError::new("blank character isn't part of the alphabet"))
        }
        if !(self.states.contains(&self.initial)) {
            return Err(DescError::new("initial state isn´t part of the state array"))
        }
        Ok(self)
    }
}