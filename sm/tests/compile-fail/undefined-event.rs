extern crate sm_macro;
use sm_macro::sm;

sm!{
    Lock {
        States { Locked }

        TurnKey { Unlocked => Locked }
    }
}

fn main() {
    use Lock::*;
    let sm = Machine::new(Locked);

    sm.transition(Invalid);
    //~^ ERROR cannot find value `Invalid` in this scope
    //~| ERROR no method named `transition` found for type `Lock::Machine<Lock::Locked>` in the current scope
}
