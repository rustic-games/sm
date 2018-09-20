extern crate sm;
use sm::sm;

sm!{
    Lock {
        InitialStates { Locked, Unlocked, Broken }
    }
}

fn main() {}
