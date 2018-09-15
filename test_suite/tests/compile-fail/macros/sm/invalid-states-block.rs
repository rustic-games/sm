extern crate sm_macro;
use sm_macro::sm;

sm!{
    Lock {
        InvalidName { Locked, Unlocked }
        //~^ ERROR expected `States { ... }` block
    }
}
