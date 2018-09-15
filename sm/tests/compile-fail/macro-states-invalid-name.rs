extern crate sm_macro;
use sm_macro::sm;

sm!{
    Lock {
        States { 1, 2 }
        //~^ ERROR expected identifier
    }
}
