#[macro_use]
extern crate sm_macro;

sm!{
    Lock {
        States { Unlocked }

        TurnKey {}
        //~^ ERROR no rules expected the token `}`
    }
}
