#[macro_use]
extern crate sm;

sm!{
    Lock {}
    //~^ ERROR no rules expected the token `}`
}
