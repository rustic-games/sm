extern crate sm;
use sm::sm;

sm!{
    Lock {
        InitialStates { Locked }
    }
}

fn main() {
    use Lock::*;

    let _ = Machine::new(Invalid);
    //~^ ERROR cannot find value `Invalid` in this scope
}
