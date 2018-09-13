#[macro_use]
extern crate sm;

#[derive(Debug)]
struct HelloWorld;
impl sm::Machine for HelloWorld {}
//~^ ERROR the trait bound `HelloWorld: std::cmp::Eq` is not satisfied

#[derive(Debug, Eq)] // add derived `PartialEq` to fix this error
//~^ ERROR can't compare `HelloUniverse` with `HelloUniverse`
struct HelloUniverse;
impl sm::Machine for HelloUniverse {}
//~^ ERROR can't compare `HelloUniverse` with `HelloUniverse`
