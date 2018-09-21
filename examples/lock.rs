extern crate sm;
use sm::sm;

sm! {
    Lock {
        InitialStates { Locked, Unlocked }

        TurnKey {
            Locked => Unlocked
            Unlocked => Locked
        }
    }
}

// to test this example:
//
// * cargo install cargo-script
// * cargo script --dep sm examples/lock.rs
//
fn main() {
    use Lock::*;
    let sm = Machine::new(Locked);

    let sm = sm.transition(TurnKey);
    assert_eq!(sm.state(), Unlocked);
    assert_eq!(sm.trigger().unwrap(), TurnKey);

    let sm = sm.transition(TurnKey);
    assert_eq!(sm.state(), Locked); // change to Unlocked to fail the assertion.
}
