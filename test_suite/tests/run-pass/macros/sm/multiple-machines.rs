extern crate sm;
use sm::sm;

sm!{
    TurnStile {
        States { Locked, Unlocked }

        Coin { Locked => Unlocked }
        Push { Unlocked => Locked }
    }

    Lock {
        States { Locked, Unlocked }

        TurnKey {
            Locked => Unlocked
            Unlocked => Locked
        }
    }
}

fn main() {}
