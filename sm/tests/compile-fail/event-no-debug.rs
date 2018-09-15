extern crate sm;

#[derive(Clone, Copy, Eq, PartialEq)]
struct HelloWorld;
impl sm::Event for HelloWorld {}
//~^ ERROR `HelloWorld` doesn't implement `std::fmt::Debug`
