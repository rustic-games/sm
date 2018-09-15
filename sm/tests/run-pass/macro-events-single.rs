extern crate sm;
use sm::sm;

sm!{
    Lock {
        States { Locked }

        TurnKey {
            Locked => Locked
        }
    }
}

fn main() {}
