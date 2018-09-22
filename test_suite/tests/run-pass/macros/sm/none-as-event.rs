extern crate sm;
use sm::sm;

sm! {
    TurnStile {
        InitialStates { Locked }

        None { Unlocked, Locked => Locked }
        Coin { Locked => Unlocked }
    }
}

fn main() {}
