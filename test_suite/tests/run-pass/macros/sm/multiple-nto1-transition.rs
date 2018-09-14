#[macro_use]
extern crate sm_macro;

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
