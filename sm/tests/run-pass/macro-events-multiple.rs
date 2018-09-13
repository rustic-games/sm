#[macro_use]
extern crate sm;

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
