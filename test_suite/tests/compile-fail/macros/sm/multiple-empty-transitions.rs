#[macro_use]
extern crate sm_macro;

sm! {
    TurnStile {
        States { Locked }

        Coin {}
        //~^ ERROR no rules expected the token `}`
        Push {}
    }
}

fn main() {}
