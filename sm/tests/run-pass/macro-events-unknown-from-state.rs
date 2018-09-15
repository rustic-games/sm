extern crate sm_macro;
use sm_macro::sm;

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
