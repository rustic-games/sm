#[macro_use]
extern crate sm;

sm!{
    Lock {
        States { Locked }

        TurnKey {
            Locked => Locked
        }
    }
}

fn main() {}
