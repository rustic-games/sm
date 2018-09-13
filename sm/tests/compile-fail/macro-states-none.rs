#[macro_use]
extern crate sm_macro;

sm!{
    Lock {}
    //~^ ERROR no rules expected the token `}`
}
