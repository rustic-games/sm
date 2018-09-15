extern crate sm;
use sm::sm;

sm!{
    Lock {
        States { Unlocked }

        TurnKey {
            Locked => Unlocked
        }
    }
}

fn main() {
    use Lock::*;
    assert_ne!(Locked, Unlocked);
}
