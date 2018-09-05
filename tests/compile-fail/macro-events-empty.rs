#[macro_use]
extern crate sm;

sm!{
    Lock { Unlocked }

    TurnKey {}
    //~^ ERROR no rules expected the token `}`
}
