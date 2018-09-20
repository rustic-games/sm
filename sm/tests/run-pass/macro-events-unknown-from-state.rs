extern crate sm;
use sm::sm;

sm!{
    Lock {
        InitialStates { Unlocked }

        TurnKey {
            Locked => Unlocked
        }
    }
}

fn main() {
    use Lock::*;
    assert_ne!(Locked, Unlocked);
}
