use std::io::prelude::*;
use std::{fmt::Debug};

use bevy::prelude::*;

type Ticks = u32;
// pub type BoxClosey<S: Debug> = Box<dyn FnMut(Transform, Transform, Option<S>) -> Result>;

#[derive(Debug, Default, Clone)]
pub struct PrintResource {
    pub path: String,
    pub buffer: String,
    pub printed: bool,
}
impl PrintResource {
    pub fn print(&mut self) {
        let mut output_file = std::fs::File::create(&self.path).expect("bad path!");
        output_file
            .write_all(self.buffer.as_bytes())
            .expect("failed to write!");
        self.printed = true;
    }
}


pub trait TestState {
    fn eval(&mut self, original_state: &Self) -> Result;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Status {
    Pending,
    Complete,
}
impl Default for Status {
    fn default() -> Self {
        Self::Pending
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Result {
    pub pass: bool,
    pub status: Status,
    pub time: Ticks,
}

pub struct TestMarker<S>
where
    S: TestState + Debug,
{
    pub original_state: Option<S>,
    pub new_state: Option<S>,
    pub result: Result,
}
impl<S: TestState + Debug> TestMarker<S> {
    pub fn evaluate(&mut self, printer: &mut PrintResource) {
        if let Some(state) = &mut self.new_state {
            if let Some(original) = &mut self.original_state {
                self.result = state.eval(original);
                printer.buffer.push_str(&format!("Test Results:\n{:#?}\n{:#?}\n{:#?}\nEND\n", self.result, self.original_state, self.new_state))
            }
        }
    }
}
