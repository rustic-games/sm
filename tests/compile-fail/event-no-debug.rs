#[macro_use]
extern crate sm;

#[derive(Eq, PartialEq)]
struct HelloWorld;
impl sm::Event for HelloWorld {}
//~^ ERROR `HelloWorld` doesn't implement `std::fmt::Debug`
