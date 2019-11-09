extern crate sm;
use sm::sm;

sm!{
    Lock {}
    //~^ ERROR unexpected end of input, expected identifier
}

fn main() {}
