#[macro_use]
extern crate sm_macro;

sm!{
    Lock {
        States {
            Locked,
            Unlocked,
            Broken
        }
    }
}

fn main() {}
