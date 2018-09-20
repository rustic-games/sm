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

    let _sm = Machine::new(Unlocked);
    //~^ ERROR the trait bound `Lock::Unlocked: sm::InitialState` is not satisfied
}
