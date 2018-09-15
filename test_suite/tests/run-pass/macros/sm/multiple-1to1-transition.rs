extern crate sm_macro;
use sm_macro::sm;

sm! {
    TurnStile {
        States { Locked, Unlocked }

        Coin { Locked => Unlocked }
        Push { Unlocked => Locked }
    }
}

fn main() {}
