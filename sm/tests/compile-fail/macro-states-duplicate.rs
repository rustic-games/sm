extern crate sm;
use sm::sm;

sm! {
//~^ ERROR conflicting implementations of trait `sm::InitialState` for type `Lock::Unlocked`
//~| ERROR the name `InitialUnlocked` is defined multiple times
//~| ERROR conflicting implementations of trait `sm::AsEnum` for type `Lock::Machine<Lock::Unlocked, sm::NoneEvent>`
    Lock {
        InitialStates { Unlocked, Unlocked }
    }
}

fn main() {}
