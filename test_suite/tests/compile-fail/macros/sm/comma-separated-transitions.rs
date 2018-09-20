extern crate sm;
use sm::sm;

sm!{
    Lock {
        InitialStates { Locked, Unlocked }

        TurnKey {
        //~^ ERROR unexpected end of input, expected identifier
            Locked => Unlocked,
            // TODO: support both with and without comma separation?
            Unlocked => Locked,
        }
    }
}
