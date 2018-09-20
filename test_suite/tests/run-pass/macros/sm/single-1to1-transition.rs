extern crate sm;
use sm::sm;

sm! {
    TurnStile {
        InitialStates { Locked, Unlocked }

        Coin { Locked => Unlocked }
    }
}

fn main() {}
