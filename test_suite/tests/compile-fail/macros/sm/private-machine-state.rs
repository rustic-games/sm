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
    //~^ ERROR expected function, tuple struct or tuple variant, found struct `Machine`
}
