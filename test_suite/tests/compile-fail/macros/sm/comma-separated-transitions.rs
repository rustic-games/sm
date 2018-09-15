extern crate sm_macro;
use sm_macro::sm;

sm!{
    Lock {
        States { Locked, Unlocked }

        TurnKey {
        //~^ ERROR unexpected end of input, expected identifier
            Locked => Unlocked,
            // TODO: support both with and without comma separation?
            Unlocked => Locked,
        }
    }
}
