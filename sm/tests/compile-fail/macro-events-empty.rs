#[macro_use]
extern crate sm;

sm!{
    Lock {
        States { Unlocked }

        TurnKey {}
        //~^ ERROR no rules expected the token `}`
    }
}
