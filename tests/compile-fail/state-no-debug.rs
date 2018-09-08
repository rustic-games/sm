#[macro_use]
extern crate sm;

struct HelloWorld;
impl sm::State for HelloWorld {}
//~^ ERROR `HelloWorld` doesn't implement `std::fmt::Debug`
