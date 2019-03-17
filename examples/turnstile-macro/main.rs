extern crate sm;
use sm::sm;

sm! {
    TurnStile {
        InitialStates { Locked }

        Push { Unlocked, Locked => Locked }
        Coin { Unlocked, Locked => Unlocked }
    }
}

use crate::TurnStile::*;

fn main() {
    // non-enum usage
    let sm = Machine::new(Locked);
    assert_eq!(sm.state(), Locked);
    assert!(sm.trigger().is_none());

    let sm = sm.transition(Coin);
    assert_eq!(sm.state(), Unlocked);
    assert_eq!(sm.trigger().unwrap(), Coin);

    // enum usage
    let mut sm = Machine::new(Locked).as_enum();
    let mut coins = 0;

    loop {
        sm = match sm {
            Variant::InitialLocked(m) => m.transition(Push).as_enum(),
            Variant::UnlockedByCoin(m) => {
                coins += 1;

                m.transition(Push).as_enum()
            }
            Variant::LockedByPush(m) => {
                if coins == 100 {
                    break;
                }

                m.transition(Coin).as_enum()
            }
        }
    }

    println!("coins: {}", coins);
}
