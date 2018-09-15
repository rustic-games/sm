extern crate sm_macro;
use sm_macro::sm;

sm!{
    Lock {
        States {
            Locked,
            Unlocked,
            Broken
        }
    }
}

fn main() {}
