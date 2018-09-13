#[macro_use]
extern crate sm_macro;

sm!{
    Lock {
        States { 1, 2 }
        //~^ ERROR no rules expected the token `1`
    }
}
