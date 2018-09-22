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

impl AsEnum for TurnStile<Unlocked, NoneEvent> {
    type Enum = Variant;

    fn as_enum(self) -> Self::Enum {
        Variant::InitialUnlocked(self)
    }
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
pub enum Variant {
    InitialUnlocked(TurnStile<Unlocked, NoneEvent>),
    InitialLocked(TurnStile<Locked, NoneEvent>),
    UnlockedByCoin(TurnStile<Unlocked, Coin>),
    LockedByPush(TurnStile<Locked, Push>),
}

#[test]
fn test_enum() {
    use Variant::*;

    let sm: TurnStile<Locked, NoneEvent> = TurnStile(Locked, None);
    match sm.as_enum() {
        InitialLocked(m) => {
            assert_eq!(m.state(), Locked);
            assert!(m.trigger().is_none());
        }
        InitialUnlocked(_) => unreachable!(),
        UnlockedByCoin(_) => unreachable!(),
        LockedByPush(_) => unreachable!(),
    }

    let sm: TurnStile<Unlocked, NoneEvent> = TurnStile(Unlocked, None);
    match sm.as_enum() {
        InitialLocked(_) => unreachable!(),
        InitialUnlocked(m) => {
            assert_eq!(m.state(), Unlocked);
            assert!(m.trigger().is_none());
        }
        UnlockedByCoin(_) => unreachable!(),
        LockedByPush(_) => unreachable!(),
    }

    let sm: TurnStile<Unlocked, Coin> = TurnStile(Unlocked, Some(Coin));
    match sm.as_enum() {
        InitialLocked(_) => unreachable!(),
        InitialUnlocked(_) => unreachable!(),
        UnlockedByCoin(m) => {
            assert_eq!(m.state(), Unlocked);
            assert_eq!(m.trigger().unwrap(), Coin);
        }
        LockedByPush(_) => unreachable!(),
    }

    let sm: TurnStile<Locked, Push> = TurnStile(Locked, Some(Push));
    match sm.as_enum() {
        InitialLocked(_) => unreachable!(),
        InitialUnlocked(_) => unreachable!(),
        UnlockedByCoin(_) => unreachable!(),
        LockedByPush(m) => {
            assert_eq!(m.state(), Locked);
            assert_eq!(m.trigger().unwrap(), Push);
        }
    }
}
