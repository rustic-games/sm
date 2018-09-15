extern crate sm;
use sm::sm;

sm! {
    TurnStile {
        States { Locked, Unlocked }

        Coin { Locked, Unlocked => Unlocked }
    }
}

fn main() {}
