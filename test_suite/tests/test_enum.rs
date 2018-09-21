extern crate sm;
use sm::{AsEnum, Event, Machine, NoneEvent, State};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Unlocked;
impl State for Unlocked {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Locked;
impl State for Locked {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Coin;
impl Event for Coin {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Push;
impl Event for Push {}

#[derive(Debug, Eq, PartialEq)]
pub struct TurnStile<S: State, E: Event>(S, Option<E>);
impl<S: State, E: Event> Machine for TurnStile<S, E> {
    type State = S;
    type Event = E;

    fn state(&self) -> Self::State {
        self.0.clone()
    }

    fn trigger(&self) -> Option<Self::Event> {
        self.1.clone()
    }
}

impl<E: Event> AsEnum for TurnStile<Unlocked, E> {
    type Enum = States<E>;

    fn as_enum(self) -> Self::Enum {
        States::Unlocked(self)
    }
}

impl<E: Event> AsEnum for TurnStile<Locked, E> {
    type Enum = States<E>;

    fn as_enum(self) -> Self::Enum {
        States::Locked(self)
    }
}

impl PartialEq<Locked> for Unlocked {
    fn eq(&self, _: &Locked) -> bool {
        false
    }
}

impl PartialEq<Unlocked> for Locked {
    fn eq(&self, _: &Unlocked) -> bool {
        false
    }
}

#[derive(Debug)]
pub enum States<E: Event> {
    Locked(TurnStile<Locked, E>),
    Unlocked(TurnStile<Unlocked, E>),
}

#[test]
fn test_enum() {
    let sm: TurnStile<Locked, NoneEvent> = TurnStile(Locked, None);
    match sm.as_enum() {
        States::Locked(m) => assert_eq!(m.state(), Locked),
        States::Unlocked(_) => unreachable!(),
    }

    let sm: TurnStile<Unlocked, NoneEvent> = TurnStile(Unlocked, None);
    match sm.as_enum() {
        States::Locked(_) => unreachable!(),
        States::Unlocked(m) => assert_eq!(m.state(), Unlocked),
    }
}
