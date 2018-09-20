extern crate sm;
use sm::sm;

sm!{
    TurnStile {
        InitialStates { Locked, Unlocked }

        Coin { Locked => Unlocked }
        Push { Unlocked => Locked }
    }

    Lock {
        InitialStates { Locked, Unlocked }

        TurnKey {
            Locked => Unlocked
            Unlocked => Locked
        }
    }
}

fn main() {}
