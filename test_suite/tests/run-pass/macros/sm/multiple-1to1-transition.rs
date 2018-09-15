extern crate sm;
use sm::sm;

sm! {
    TurnStile {
        States { Locked, Unlocked }

        Coin { Locked => Unlocked }
        Push { Unlocked => Locked }
    }
}

fn main() {}
