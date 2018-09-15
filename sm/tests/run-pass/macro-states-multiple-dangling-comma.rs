extern crate sm;
use sm::sm;

sm!{
    Lock {
        States { Locked, Unlocked, Broken, }
    }
}

fn main() {}
