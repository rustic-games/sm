#[macro_use]
extern crate sm_macro;

sm!{
    Lock {
        InvalidName { Locked, Unlocked }
        //~^ ERROR no rules expected the token `InvalidName`
    }
}