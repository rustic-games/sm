extern crate sm;
use sm::sm;

sm!{
    Lock {
        InitialStates { Locked }

        TurnKey { Unlocked => Locked }
    }
}

fn main() {
    use Lock::*;
    let sm = Machine::new(Locked);

    sm.transition(Invalid);
    //~^ ERROR cannot find value `Invalid` in this scope
    //~| ERROR no method named `transition` found for struct `Lock::Machine<Lock::Locked, sm::NoneEvent>` in the current scope
}
