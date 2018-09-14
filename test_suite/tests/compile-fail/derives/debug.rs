extern crate sm;
use sm::{AsEnum, Event, Machine, State, Transition};

#[derive(Clone, Eq, PartialEq)]
struct HelloEvent;
impl Event for HelloEvent {}
//~^ ERROR `HelloEvent` doesn't implement `std::fmt::Debug`

#[derive(Clone, Eq, PartialEq)]
struct HelloState;
impl State for HelloState {}
//~^ ERROR `HelloState` doesn't implement `std::fmt::Debug`

#[derive(Eq, PartialEq)]
struct HelloMachine;
impl Machine for HelloMachine {}
//~^ ERROR `HelloMachine` doesn't implement `std::fmt::Debug`

struct HelloTransition;
impl<E: Event> Transition<E> for HelloTransition {}
//~^ ERROR `HelloTransition` doesn't implement `std::fmt::Debug`

struct HelloAsEnum;
impl AsEnum for HelloAsEnum {}
//~^ ERROR `HelloAsEnum` doesn't implement `std::fmt::Debug`

fn main() {}
