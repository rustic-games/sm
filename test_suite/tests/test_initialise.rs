extern crate sm;
use sm::{InitialState, Machine, NewMachine, State};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Unlocked;
impl State for Unlocked {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Locked;
impl State for Locked {}
impl InitialState for Locked {}

#[derive(Debug, Eq, PartialEq)]
pub struct TurnStile<S: State>(S);
impl<S: State> Machine for TurnStile<S> {
    type State = S;

    fn state(&self) -> S {
        self.0.clone()
    }
}

impl<S: InitialState> NewMachine<S> for TurnStile<S> {
    type Machine = TurnStile<S>;

    fn new(state: S) -> Self::Machine {
        TurnStile(state)
    }
}

#[test]
fn test_initialise_locked() {
    let sm = TurnStile::new(Locked);
    assert_eq!(sm.state(), Locked);
}
