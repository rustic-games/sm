extern crate sm;
use sm::sm;

sm!{
    Lock {
        InitialStates { 1, 2 }
        //~^ ERROR expected identifier
    }
}
