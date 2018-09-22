extern crate sm;
use sm::sm;

sm!{
    Lock {
        InitialStates { Unlocked, Locked }

        TurnKey {
            Locked => Unlocked
            Unlocked => Locked
        }
    }
}

fn main() {
    use Lock::Variant::*;
    use Lock::*;

    let mut sm = Machine::new(Locked).as_enum();

    loop {
        sm = match sm {
            InitialUnlocked(_) => unreachable!(),
            InitialLocked(m) => m.transition(TurnKey).as_enum(),
            UnlockedByTurnKey(m) => m.transition(TurnKey).as_enum(),
            LockedByTurnKey(_) => break,
        }
    }
}
