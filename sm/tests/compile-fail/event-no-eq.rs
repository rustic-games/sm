#[macro_use]
extern crate sm;

#[derive(Debug)]
struct HelloWorld;
impl sm::Event for HelloWorld {}
//~^ ERROR the trait bound `HelloWorld: std::cmp::Eq` is not satisfied
