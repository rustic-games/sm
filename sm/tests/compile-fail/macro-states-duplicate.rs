extern crate sm;
use sm::sm;

sm!{
//~^ ERROR conflicting implementations of trait `sm::InitialState` for type `Lock::Unlocked`
    Lock {
        InitialStates { Unlocked, Unlocked }
    }
}

fn main() {}
