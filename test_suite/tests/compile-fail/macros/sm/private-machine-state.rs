extern crate sm;
use sm::sm;

sm!{
    Lock {
        InitialStates { Locked }

        TurnKey { Locked => Unlocked }
    }
}

fn main() {
    use Lock::*;

    let _sm = Machine(Locked);
    //~^ ERROR expected function, found struct `Machine`
}
