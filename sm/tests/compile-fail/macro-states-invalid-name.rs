extern crate sm;
use sm::sm;

sm!{
    Lock {
        States { 1, 2 }
        //~^ ERROR expected identifier
    }
}
