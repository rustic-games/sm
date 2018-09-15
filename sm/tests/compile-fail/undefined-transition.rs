extern crate sm;
use sm::sm;

sm!{
    Lock {
        States { Locked, Unlocked }

        TurnKey {
            Locked => Unlocked
        }
    }
}

fn main() {
    use Lock::*;
    let sm = Machine::new(Unlocked);

    sm.transition(TurnKey);
    //~^ ERROR no method named `transition` found for type `Lock::Machine<Lock::Unlocked>` in the current scope
}
