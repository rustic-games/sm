#[macro_use]
extern crate sm_macro;

sm!{
    TurnStile {}
    //~^ ERROR no rules expected the token `}`
}
