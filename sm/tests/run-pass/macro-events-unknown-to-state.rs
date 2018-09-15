extern crate sm;
use sm::sm;

sm!{
    Lock {
        States { Unlocked }

        TurnKey {
            Unlocked => Locked
        }
    }
}

fn main() {}
