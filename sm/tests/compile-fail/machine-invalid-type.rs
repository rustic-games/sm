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

    let sm = Machine::new(Locked);
    while sm.state() == Locked {
        sm = sm.transition(TurnKey);
        //~^ ERROR mismatched types
    }
}
