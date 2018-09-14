#[macro_use]
extern crate sm_macro;

sm! {
    TurnStile {
        States { Locked, Unlocked }

        Coin { Locked, Unlocked => Unlocked }
    }
}

fn main() {}
