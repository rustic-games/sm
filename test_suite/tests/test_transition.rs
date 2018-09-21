extern crate sm;
use sm::{Event, Machine, NoneEvent, State, Transition};

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

impl<E: Event> Transition<Push> for TurnStile<Locked, E> {
    type Machine = TurnStile<Locked, Push>;

    fn transition(self, event: Push) -> Self::Machine {
        TurnStile(Locked, Some(event))
    }
}

impl<E: Event> Transition<Push> for TurnStile<Unlocked, E> {
    type Machine = TurnStile<Locked, Push>;

    fn transition(self, event: Push) -> Self::Machine {
        TurnStile(Locked, Some(event))
    }
}

impl<E: Event> Transition<Coin> for TurnStile<Locked, E> {
    type Machine = TurnStile<Unlocked, Coin>;

    fn transition(self, event: Coin) -> Self::Machine {
        TurnStile(Unlocked, Some(event))
    }
}

impl<E: Event> Transition<Coin> for TurnStile<Unlocked, E> {
    type Machine = TurnStile<Unlocked, Coin>;

    fn transition(self, event: Coin) -> Self::Machine {
        TurnStile(Unlocked, Some(event))
    }
}

#[test]
fn test_transitions_locked() {
    let sm: TurnStile<Locked, NoneEvent> = TurnStile(Locked, None);
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
    let sm: TurnStile<Unlocked, NoneEvent> = TurnStile(Unlocked, None);
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
