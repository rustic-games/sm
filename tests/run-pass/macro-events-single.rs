#[macro_use]
extern crate sm;

sm!{
    Lock { Locked }

    TurnKey {
        Locked => Locked
    }
}

fn main() {}
