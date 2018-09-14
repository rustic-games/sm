#[macro_use]
extern crate sm_macro;

sm!{
    TurnStile {
        States { Locked }

        Coin { Locked => Unlocked }
        //~^ ERROR cannot find type `Unlocked` in this scope
        //~| ERROR cannot find value `Unlocked` in this scope
    }
}

fn main() {}
