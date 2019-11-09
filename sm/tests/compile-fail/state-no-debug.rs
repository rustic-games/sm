extern crate sm;

#[derive(Copy, Clone, Eq, PartialEq)]
struct HelloWorld;
impl sm::State for HelloWorld {}
//~^ ERROR `HelloWorld` doesn't implement `std::fmt::Debug`

fn main() {}
