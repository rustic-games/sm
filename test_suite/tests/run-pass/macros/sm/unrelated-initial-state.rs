extern crate sm;
use sm::sm;

sm!{
    TurnStile {
        InitialStates { Unlocked }

        Push { Unlocked => Locked }
    }

    Lock {
        InitialStates { Locked }

        TurnKey { Locked => Unlocked }
    }
}

fn main() {
    // TODO: this should not be allowed, since you are mixing states from
    // different machines, and the `Unlocked` state is not defined as an
    // aceptable "initial state" in the Lock machine.
    //
    // This currently works because we only validate if the struct has the
    // `InitialState` trait implemented, not if it's an actual state that was
    // defined on the current machine.
    //
    // This is UB, and not promoted. It will be fixed in the future and will
    // cause breaking changes.
    let _sm = Lock::Machine::new(TurnStile::Unlocked);
}
