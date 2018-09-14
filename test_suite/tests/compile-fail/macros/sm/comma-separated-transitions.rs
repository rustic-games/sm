#[macro_use]
extern crate sm_macro;

sm!{
    Lock {
        States { Locked, Unlocked }

        TurnKey {
            // TODO: support both with and without comma separation?
            Locked => Unlocked,
            //~^ ERROR no rules expected the token `,`
            Unlocked => Locked,
        }
    }
}
