extern crate sm;
use sm::sm;

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
