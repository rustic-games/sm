// to run this example:
//
//     $ cargo run --example turnstile-manual
//
// see: https://en.wikipedia.org/wiki/Finite-state_machine#Example:_coin-operated_turnstile

extern crate sm;
use sm::{AsEnum, Event, InitialState, Initializer, Machine, NoneEvent, State, Transition};

// Create a new `TurnStile` tuple-struct, which will be our state machine
// object. It keeps track of both a `State`, and optional `Event`.
//
// The state of the machine tells us exactly that; the current state of the
// state machine.
//
// The optional event tells us which (if any) event caused the state machine to
// end up in the current state. This will return `None` if the machine is still
// in its initial state, or returns an event if the machine was transitioned
// from one state to another using an event.
#[derive(Debug, Eq, PartialEq)]
pub struct TurnStile<S: State, E: Event>(S, Option<E>);

// Implement the `Machine` trait, allowing us to query the current `state() and
// the `trigger()` event that caused the current state of the machine.
impl<S: State, E: Event> Machine for TurnStile<S, E> {
    type Event = E;
    type State = S;

    fn state(&self) -> Self::State {
        self.0.clone()
    }

    fn trigger(&self) -> Option<Self::Event> {
        self.1.clone()
    }
}

// Implement the `Initializer` trait, to allow a new machine to be initialised
// using the `new` associated function, given a valid state marked with the
// `InitialState` marker trait.
impl<S: InitialState> Initializer<S> for TurnStile<S, NoneEvent> {
    type Machine = TurnStile<S, NoneEvent>;

    fn new(state: S) -> Self::Machine {
        TurnStile(state, None)
    }
}

// Define the `Locked` unit-like struct. We also derive several standard traits,
// since those are required on any struct we want to use as a machine state.
#[derive(Clone, Debug, Eq)]
pub struct Locked;

// Implement the `State` marker trait for our new struct, which allows us to use
// the struct as a valid state in our state machine.
impl State for Locked {}

// We also implement the `InitialState` marker trait for the `Locked` state.
// This allows us to use this state when initialising a new machine.
impl InitialState for Locked {}

// Similar to the `Locked` state, we also implement the `Unlocked` state. Since
// we don't implement the `InitialState` trait for this state, we cannot
// initialise the machine with this state, we have to get to this state by using
// the appropriate event (see below).
#[derive(Clone, Debug, Eq)]
pub struct Unlocked;
impl State for Unlocked {}

// Define the `Coin` unit-like struct. We also derive several standard traits,
// since those are required on any struct we want to use as an event.
#[derive(Clone, Debug, Eq)]
pub struct Coin;

// Implement the `Event` marker trait for our new struct, which allows us to use
// the struct as a valid event to transition our machine between states.
impl Event for Coin {}

// Similar to the `Coin` event, we also implement the `Push` event.
#[derive(Clone, Debug, Eq)]
pub struct Push;
impl Event for Push {}

// To be able to compare states or events against each other, we need to
// implement the `PartialEq` traits for all valid combinations:

// Unlocked != Locked
impl PartialEq<Unlocked> for Locked {
    fn eq(&self, _: &Unlocked) -> bool {
        false
    }
}

// Locked == Locked
impl PartialEq<Locked> for Locked {
    fn eq(&self, _: &Locked) -> bool {
        true
    }
}

// Unlocked == Unlocked
impl PartialEq<Unlocked> for Unlocked {
    fn eq(&self, _: &Unlocked) -> bool {
        true
    }
}

// Locked != Unlocked
impl PartialEq<Locked> for Unlocked {
    fn eq(&self, _: &Locked) -> bool {
        false
    }
}

// Push != Coin
impl PartialEq<Push> for Coin {
    fn eq(&self, _: &Push) -> bool {
        false
    }
}

// Coin == Coin
impl PartialEq<Coin> for Coin {
    fn eq(&self, _: &Coin) -> bool {
        true
    }
}

// Push == Push
impl PartialEq<Push> for Push {
    fn eq(&self, _: &Push) -> bool {
        true
    }
}

// Coin != Push
impl PartialEq<Coin> for Push {
    fn eq(&self, _: &Coin) -> bool {
        false
    }
}

// Next, we start implementing all possible state transitions for our machine.
// We want these transitions to be validated at compile time, so we'll implement
// one transition for each possible state change, using concrete types, instead
// of the generic state trait.

// We start with the `Locked -> Push -> Locked` transition
impl<E: Event> Transition<Push> for TurnStile<Locked, E> {
    type Machine = TurnStile<Locked, Push>;

    fn transition(self, event: Push) -> Self::Machine {
        TurnStile(Locked, Some(event))
    }
}

// `Unlocked -> Push -> Locked`
impl<E: Event> Transition<Push> for TurnStile<Unlocked, E> {
    type Machine = TurnStile<Locked, Push>;

    fn transition(self, event: Push) -> Self::Machine {
        TurnStile(Locked, Some(event))
    }
}

// `Locked -> Coin -> Unlocked`
impl<E: Event> Transition<Coin> for TurnStile<Locked, E> {
    type Machine = TurnStile<Unlocked, Coin>;

    fn transition(self, event: Coin) -> Self::Machine {
        TurnStile(Unlocked, Some(event))
    }
}

// `Unlocked -> Coin -> Unlocked`
impl<E: Event> Transition<Coin> for TurnStile<Unlocked, E> {
    type Machine = TurnStile<Unlocked, Coin>;

    fn transition(self, event: Coin) -> Self::Machine {
        TurnStile(Unlocked, Some(event))
    }
}

// Finally, we add an enum implementation to allow pattern matching against
// possible machine states and events.

pub enum Variant {
    InitialLocked(TurnStile<Locked, NoneEvent>),
    UnlockedByCoin(TurnStile<Unlocked, Coin>),
    LockedByPush(TurnStile<Locked, Push>),
}

impl AsEnum for TurnStile<Locked, NoneEvent> {
    type Enum = Variant;

    fn as_enum(self) -> Self::Enum {
        Variant::InitialLocked(self)
    }
}

impl AsEnum for TurnStile<Unlocked, Coin> {
    type Enum = Variant;

    fn as_enum(self) -> Self::Enum {
        Variant::UnlockedByCoin(self)
    }
}

impl AsEnum for TurnStile<Locked, Push> {
    type Enum = Variant;

    fn as_enum(self) -> Self::Enum {
        Variant::LockedByPush(self)
    }
}

// And we're done! We've now manually implemented our static state machine using
// a combination of traits and zero-sized structs.
//
// To make it more convenient to use, this crate also exposes a macro that
// generates the above code, based on a declarative state machine declaration.
//
// See the `turnstile-macro` example for more details.

fn main() {
    // non-enum usage
    let sm = TurnStile::new(Locked);
    assert_eq!(sm.state(), Locked);
    assert!(sm.trigger().is_none());

    let sm = sm.transition(Coin);
    assert_eq!(sm.state(), Unlocked);
    assert_eq!(sm.trigger().unwrap(), Coin);

    // enum usage
    let mut sm = TurnStile::new(Locked).as_enum();
    let mut coins = 0;

    loop {
        sm = match sm {
            Variant::InitialLocked(m) => m.transition(Push).as_enum(),
            Variant::UnlockedByCoin(m) => {
                coins += 1;

                m.transition(Push).as_enum()
            },
            Variant::LockedByPush(m) => {
                if coins == 100 {
                    break;
                }

                m.transition(Coin).as_enum()
            },
        }
    }

    println!("coins: {}", coins);
}
