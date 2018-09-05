#[macro_use]
extern crate sm;

sm!{
    Lock { 1, 2 }
    //~^ ERROR no rules expected the token `1`
}
