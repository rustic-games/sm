extern crate sm;
use sm::sm;

sm!{
    Lock {
        InitialStates { Unlocked }

        TurnKey {}
    }
}

fn main() {}
