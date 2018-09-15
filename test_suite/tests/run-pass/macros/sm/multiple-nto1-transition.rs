extern crate sm;
use sm::sm;

sm! {
    TurnStile {
        States { Locked, Unlocked }

        Coin {
            Locked, Unlocked => Unlocked
        }

        Push {
            Locked,
            Unlocked => Locked
        }
    }
}

fn main() {}
