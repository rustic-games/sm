extern crate sm;
use sm::{Event, Machine, State, Transition};

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

impl Transition<Push> for TurnStile<Locked> {
    type Machine = TurnStile<Locked>;

    fn transition(self, _: Push) -> Self::Machine {
        TurnStile(Locked)
    }
}

impl Transition<Push> for TurnStile<Unlocked> {
    type Machine = TurnStile<Locked>;

    fn transition(self, _: Push) -> Self::Machine {
        TurnStile(Locked)
    }
}

impl Transition<Coin> for TurnStile<Locked> {
    type Machine = TurnStile<Unlocked>;

    fn transition(self, _: Coin) -> Self::Machine {
        TurnStile(Unlocked)
    }
}

impl Transition<Coin> for TurnStile<Unlocked> {
    type Machine = TurnStile<Unlocked>;

    fn transition(self, _: Coin) -> Self::Machine {
        TurnStile(Unlocked)
    }
}

#[test]
fn test_transitions_locked() {
    let sm = TurnStile(Locked);
    assert_eq!(sm.state(), Locked);

    let sm = sm.transition(Push);
    assert_eq!(sm.state(), Locked);

    let sm = sm.transition(Coin);
    assert_eq!(sm.state(), Unlocked);

    let sm = sm.transition(Coin);
    assert_eq!(sm.state(), Unlocked);

    let sm = sm.transition(Push);
    assert_eq!(sm.state(), Locked);
}

#[test]
fn test_transitions_unlocked() {
    let sm = TurnStile(Unlocked);
    assert_eq!(sm.state(), Unlocked);

    let sm = sm.transition(Push);
    assert_eq!(sm.state(), Locked);

    let sm = sm.transition(Coin);
    assert_eq!(sm.state(), Unlocked);

    let sm = sm.transition(Coin);
    assert_eq!(sm.state(), Unlocked);

    let sm = sm.transition(Push);
    assert_eq!(sm.state(), Locked);
}
