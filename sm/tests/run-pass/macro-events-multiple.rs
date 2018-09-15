extern crate sm_macro;
use sm_macro::sm;

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
