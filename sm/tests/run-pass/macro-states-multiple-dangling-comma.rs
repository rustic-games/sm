#[macro_use]
extern crate sm;

sm!{
    Lock {
        States { Locked, Unlocked, Broken, }
    }
}

fn main() {}
