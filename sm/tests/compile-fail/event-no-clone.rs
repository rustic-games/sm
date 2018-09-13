#[macro_use]
extern crate sm;

#[derive(Debug, Eq, PartialEq)]
struct HelloWorld;
impl sm::Event for HelloWorld {}
//~^ ERROR the trait bound `HelloWorld: std::clone::Clone` is not satisfied
