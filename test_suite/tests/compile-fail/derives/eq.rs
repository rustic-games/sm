extern crate sm;
use sm::{AsEnum, Event, Machine, State, Transition};

#[derive(Clone, Debug, PartialEq)]
struct HelloEvent;
impl Event for HelloEvent {}
//~^ ERROR the trait bound `HelloEvent: std::cmp::Eq` is not satisfied

#[derive(Clone, Debug, PartialEq)]
struct HelloState;
impl State for HelloState {}
//~^ ERROR the trait bound `HelloState: std::cmp::Eq` is not satisfied

#[derive(Debug, PartialEq)]
struct HelloMachine;
impl Machine for HelloMachine {}
//~^ ERROR the trait bound `HelloMachine: std::cmp::Eq` is not satisfied

fn main() {}
