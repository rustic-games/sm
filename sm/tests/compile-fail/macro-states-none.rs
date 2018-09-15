extern crate sm_macro;
use sm_macro::sm;

sm!{
    Lock {}
    //~^ ERROR unexpected end of input, expected identifier
}
