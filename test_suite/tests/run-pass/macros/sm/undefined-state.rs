extern crate sm_macro;
use sm_macro::sm;

sm!{
    TurnStile {
        // Even though we haven't defined the `Unlocked` state below, it will
        // still work as expected, since the macro takes all (unique) states
        // from all defined transitions, and generates the code for those
        // states.
        States { Locked }

        Coin { Locked => Unlocked }
    }
}

fn main() {}
