extern crate sm;
use sm::sm;

sm!{
    Lock {
        InvalidName { Locked, Unlocked }
        //~^ ERROR expected `InitialStates { ... }` block
    }
}

fn main() {}
