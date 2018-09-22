extern crate sm;
use sm::sm;

sm! {
    TurnStile {
        InitialStates { Locked, Unlocked }

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
    assert!(sm.trigger().is_none());

    let sm = sm.transition(Coin);
    assert_eq!(sm.state(), Unlocked);
    assert_eq!(sm.trigger().unwrap(), Coin);

    let sm = sm.transition(Coin);
    assert_eq!(sm.state(), Unlocked);

    let sm = sm.transition(Push);
    assert_eq!(sm.state(), Locked);
    assert_eq!(sm.trigger().unwrap(), Push);

    let sm = sm.transition(Push);
    assert_eq!(sm.state(), Locked); // change to Unlocked to fail the assertion.

    let mut sm = Machine::new(Unlocked).as_enum();
    use TurnStile::Variant::*;

    loop {
        sm = match sm {
            InitialLocked(_) => unreachable!(),
            InitialUnlocked(m) => {
                assert_eq!(m.state(), Unlocked);
                assert!(m.trigger().is_none());

                m.transition(Coin).as_enum()
            }
            UnlockedByCoin(m) => {
                assert_eq!(m.state(), Unlocked);
                assert_eq!(m.trigger().unwrap(), Coin);

                m.transition(Push).as_enum()
            }
            LockedByPush(m) => {
                assert_eq!(m.state(), Locked);
                assert_eq!(m.trigger().unwrap(), Push);

                break;
            }
        }
    }
}
