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

    let mut sm = Machine::new(Locked).as_enum();

    loop {
        sm = match sm {
            States::Locked(m) => m.transition(TurnKey).as_enum(),
            States::Unlocked(_) => {
                break;
            }
        }
    }
}
