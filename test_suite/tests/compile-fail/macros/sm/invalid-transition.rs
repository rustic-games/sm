extern crate sm;
use sm::sm;

sm!{
    Lock {
        InitialStates { Locked, Unlocked }

        TurnKey { Locked => Unlocked }
    }
}

fn main() {
    use Lock::*;

    let sm = Machine::new(Unlocked);
    sm = sm.transition(TurnKey);
    //~^ ERROR no method named `transition` found for struct `Lock::Machine<Lock::Unlocked, sm::NoneEvent>` in the current scope
}
