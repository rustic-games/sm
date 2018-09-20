extern crate sm;
use sm::sm;

sm! {
    TurnStile {
        InitialStates { Locked, Unlocked }

        Coin { Locked => Unlocked }
        Push { Unlocked => Locked }
    }
}

fn main() {}
