#[macro_use]
extern crate sm_macro;

sm!{
    Lock {
        States { Locked }

        TurnKey {
            Locked => Locked
        }

        Knock {
            Locked => Locked
        }
    }
}

fn main() {}
