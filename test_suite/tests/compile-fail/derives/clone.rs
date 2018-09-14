extern crate sm;
use sm::{AsEnum, Event, Machine, State, Transition};

#[derive(Debug, Eq, PartialEq)]
struct HelloEvent;
impl Event for HelloEvent {}
//~^ ERROR the trait bound `HelloEvent: std::clone::Clone` is not satisfied

#[derive(Debug, Eq, PartialEq)]
struct HelloState;
impl State for HelloState {}
//~^ ERROR the trait bound `HelloState: std::clone::Clone` is not satisfied

fn main() {}
