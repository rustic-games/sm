#[macro_use]
extern crate sm;

struct HelloWorld;
impl sm::Event for HelloWorld {}
//~^ ERROR `HelloWorld` doesn't implement `std::fmt::Debug`
