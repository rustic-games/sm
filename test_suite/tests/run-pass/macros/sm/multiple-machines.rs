#[macro_use]
extern crate sm_macro;

sm!{
    TurnStile {
        States { Locked, Unlocked }

        Coin { Locked => Unlocked }
        Push { Unlocked => Locked }
    }

    Lock {
        States { Locked, Unlocked }

        TurnKey {
            Locked => Unlocked
            Unlocked => Locked
        }
    }
}

fn main() {}
