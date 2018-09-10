#[macro_use]
extern crate sm;

sm! {
    TurnStile {
        States { Locked, Unlocked }

        Push {
            Unlocked, Locked => Locked
        }

        Coin {
            Locked, Unlocked => Unlocked
        }
    }
}

// to test this example:
//
// * cargo install cargo-script
// * cargo script --dep sm examples/turnstile.rs
//
// see: https://en.wikipedia.org/wiki/Finite-state_machine#Example:_coin-operated_turnstile
//
fn main() {
    use TurnStile::*;
    let sm = Machine::new(Locked);
    assert_eq!(sm.state(), Locked);

    let sm = sm.transition(Coin);
    assert_eq!(sm.state(), Unlocked);

    let sm = sm.transition(Coin);
    assert_eq!(sm.state(), Unlocked);

    let sm = sm.transition(Push);
    assert_eq!(sm.state(), Locked);

    let sm = sm.transition(Push);
    assert_eq!(sm.state(), Locked); // change to Unlocked to fail the assertion.
}
