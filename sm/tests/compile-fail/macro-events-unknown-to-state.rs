#[macro_use]
extern crate sm;

sm!{
    Lock {
        States { Unlocked }

        TurnKey {
            Unlocked => Locked
            //~^ ERROR cannot find value `Locked` in this scope
            //~| ERROR cannot find type `Locked` in this scope
        }
    }
}
