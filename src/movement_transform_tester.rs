use std::io::prelude::*;
use std::{fmt::Debug, fs::File};

use bevy::prelude::*;

type Ticks = u32;
pub type BoxClosey<S: Debug> = Box<dyn Fn(Transform, Transform, Option<S>) -> Result>;
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

pub trait Test<S> {
    fn state(&self) -> S
    where
        S: Debug;
    fn evaluate(&mut self) -> Result;
}

#[derive(Debug, Copy, Clone)]
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
    S: Debug,
{
    pub original_trans: Transform,
    pub new_trans: Transform,
    pub eval: BoxClosey<S>,
    pub result: Result,
    pub state: Option<S>,
}
impl<S: Debug> TestMarker<S> {
    pub fn evaluate(&self) {
        let x = (self.eval)(Transform::default(), Transform::default(), None);
    }
}
