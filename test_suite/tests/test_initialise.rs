extern crate sm;
use sm::{Event, InitialState, Initializer, Machine, NoneEvent, State};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Unlocked;
impl State for Unlocked {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Locked;
impl State for Locked {}
impl InitialState for Locked {}

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

impl<S: InitialState> Initializer<S> for TurnStile<S, NoneEvent> {
    type Machine = TurnStile<S, NoneEvent>;

    fn new(state: S) -> Self::Machine {
        TurnStile(state, None)
    }
}

#[test]
fn test_initialise_locked() {
    let sm = TurnStile::new(Locked);
    assert_eq!(sm.state(), Locked);
}
