#[macro_use]
extern crate sm;

sm!{
    Lock {
        States { Unlocked }

        TurnKey {
            // FIXME: investigate why this compiles without main()
            Locked => Unlocked
            //~^ ERROR cannot find type `Locked` in this scope
        }
    }
}

fn main() {
    use Lock::*;
    assert_eq!(Locked, Unlocked);
    //~^ ERROR cannot find value `Locked` in this scope
}
