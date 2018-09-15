extern crate sm_macro;
use sm_macro::sm;

sm!{
    Lock {
        States { Unlocked }

        TurnKey {
            Unlocked => Locked
        }
    }
}

fn main() {}
