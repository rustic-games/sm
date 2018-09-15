extern crate sm;

#[derive(Eq, PartialEq)]
struct HelloWorld;
impl sm::Machine for HelloWorld {}
//~^ ERROR `HelloWorld` doesn't implement `std::fmt::Debug`
