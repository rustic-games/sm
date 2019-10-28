extern crate sm;
use sm::sm;

sm! {
    TurnStile {
        InitialStates { Unlocked }

        Coin { Locked => Unlocked }
        Push { Unlocked => Locked }
    }
}

fn main() {
    use TurnStile::*;

    let sm1 = Machine::new(Unlocked);
    let sm1 = sm1.transition(Push);
    assert_eq!(sm1.state(), Locked);

    let sm2 = sm1.clone();
    assert_eq!(sm1.transition(Coin).state(), Unlocked);
    assert_eq!(sm2.state(), Locked);
}
