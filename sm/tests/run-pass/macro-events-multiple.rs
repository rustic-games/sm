extern crate sm;
use sm::sm;

sm!{
    Lock {
        InitialStates { Locked }

        TurnKey {
            Locked => Locked
        }

        Knock {
            Locked => Locked
        }
    }
}

fn main() {}
