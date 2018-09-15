extern crate sm_macro;
use sm_macro::sm;

sm! {
    TurnStile {
        States { Locked }

        Coin {}
        Push {}
    }
}

fn main() {}
