#[macro_use]
extern crate sm_macro;

sm! {
    TurnStile {
        States { Locked, Unlocked }

        Coin { Locked => Unlocked }
        Push { Unlocked => Locked }
    }
}

fn main() {}
