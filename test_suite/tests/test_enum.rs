extern crate sm;
use sm::{AsEnum, Event, Machine, State};

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
pub struct TurnStile<S: State>(pub S);
impl<S: State> Machine for TurnStile<S> {
    type State = S;

    fn state(&self) -> S {
        self.0.clone()
    }
}

impl AsEnum for TurnStile<Unlocked> {
    type Enum = States;

    fn as_enum(self) -> Self::Enum {
        States::Unlocked(self)
    }
}

impl AsEnum for TurnStile<Locked> {
    type Enum = States;

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
pub enum States {
    Locked(TurnStile<Locked>),
    Unlocked(TurnStile<Unlocked>),
}

#[test]
fn test_enum() {
    let sm = TurnStile(Locked);
    match sm.as_enum() {
        States::Locked(m) => assert_eq!(m.state(), Locked),
        States::Unlocked(_) => unreachable!(),
    }

    let sm = TurnStile(Unlocked);
    match sm.as_enum() {
        States::Locked(_) => unreachable!(),
        States::Unlocked(m) => assert_eq!(m.state(), Unlocked),
    }
}
