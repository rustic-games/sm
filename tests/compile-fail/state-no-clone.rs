#[macro_use]
extern crate sm;

#[derive(Debug)]
struct HelloWorld;
impl sm::State for HelloWorld {}
//~^ ERROR the trait bound `HelloWorld: std::clone::Clone` is not satisfied