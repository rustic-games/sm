#[macro_use]
extern crate sm;

sm!{
    Lock { Locked }

    TurnKey {
        Locked => Locked
    }

    Knock {
        Locked => Locked
    }
}

fn main() {}
