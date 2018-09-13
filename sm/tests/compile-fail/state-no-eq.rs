#[macro_use]
extern crate sm;

#[derive(Clone, Copy, Debug)]
struct HelloWorld;
impl sm::State for HelloWorld {}
//~^ ERROR the trait bound `HelloWorld: std::cmp::Eq` is not satisfied
