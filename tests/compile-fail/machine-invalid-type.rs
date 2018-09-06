#[macro_use]
extern crate sm;

sm!{
    Lock { Locked, Unlocked }

    TurnKey { Locked => Unlocked }
}

fn main() {
    use Lock::*;

    let sm = Machine::new(Locked);
    while sm.state() == Locked {
        sm = sm.event(TurnKey);
        //~^ ERROR mismatched types
    }
}
