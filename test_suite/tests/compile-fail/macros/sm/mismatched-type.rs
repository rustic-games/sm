extern crate sm_macro;
use sm_macro::sm;

sm!{
    Lock {
        States { Locked, Unlocked }

        TurnKey { Locked => Unlocked }
    }
}

fn main() {
    use Lock::*;

    let sm = Machine::new(Locked);
    sm = sm.transition(TurnKey)
    //~^ ERROR mismatched types
}
