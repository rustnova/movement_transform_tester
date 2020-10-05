use std::fmt::Debug;
use std::io::prelude::*;

use bevy::prelude::*;

type Ticks = u32;
// pub type BoxClosey<S: Debug> = Box<dyn FnMut(Transform, Transform, Option<S>) -> Result>;

#[derive(Debug, Default, Clone)]
pub struct PrintResource {
    pub path: String,
    pub pass_buffer: String,
    pub fail_buffer: String,
    pub printed: bool,
}
impl PrintResource {
    pub fn print(&mut self) {
        let mut pass_file = std::fs::File::create(self.path.clone() + "passed.txt").expect("bad path!");
        let mut fail_file = std::fs::File::create(self.path.clone() + "fail.txt").expect("bad path!");
        pass_file
            .write_all(self.pass_buffer.as_bytes())
            .expect("failed to write!");
        fail_file
            .write_all(self.fail_buffer.as_bytes())
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

impl std::fmt::Display for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?} ", self)
    }
}

pub struct TestMarker<S>
where
    S: TestState + std::fmt::Display,
{
    pub original_state: Option<S>,
    pub new_state: Option<S>,
    pub result: Result,
}
impl<S: TestState + std::fmt::Display> TestMarker<S> {
    pub fn evaluate(&mut self, printer: &mut PrintResource) {
        if let Some(state) = &mut self.new_state {
            if let Some(original) = &mut self.original_state {
                self.result = state.eval(original);
                if self.result.pass {
                    printer.pass_buffer.push_str(&format!(
                        "Test Results: {} {} {}\n",
                        self.result,
                        self.original_state.as_ref().unwrap(),
                        self.new_state.as_ref().unwrap()
                    ));
                } else {
                    printer.fail_buffer.push_str(&format!(
                        "Test Results: {} {} {}\n",
                        self.result,
                        self.original_state.as_ref().unwrap(),
                        self.new_state.as_ref().unwrap()
                    ))
                }
            }
        }
    }
}
