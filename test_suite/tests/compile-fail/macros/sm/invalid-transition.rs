#[macro_use]
extern crate sm_macro;

sm!{
    Lock {
        States { Locked, Unlocked }

        TurnKey { Locked => Unlocked }
    }
}

fn main() {
    use Lock::*;

    let sm = Machine::new(Unlocked);
    sm = sm.transition(TurnKey);
    //~^ ERROR no method named `transition` found for type `Lock::Machine<Lock::Unlocked>` in the current scope
}
