#[macro_use]
extern crate sm;

sm!{
    Lock { Locked, Unlocked }

    TurnKey {
        Locked => Unlocked
    }
}

fn main() {
    use Lock::*;
    let sm = Machine::new(Unlocked);

    sm.event(TurnKey);
    //~^ ERROR no method named `event` found for type `Lock::Machine<Lock::Unlocked>` in the current scope
}
