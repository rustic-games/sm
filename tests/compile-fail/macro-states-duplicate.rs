#[macro_use]
extern crate sm;

sm!{
    Lock { Unlocked, Unlocked }
    //~^^ ERROR the name `Unlocked` is defined multiple times
    //~| ERROR the name `Unlocked` is defined multiple times
    //~| ERROR conflicting implementations of trait `Lock::State` for type `Lock::Unlocked`
    //~| ERROR conflicting implementations of trait `std::marker::Copy` for type `Lock::Unlocked`
    //~| ERROR conflicting implementations of trait `std::cmp::PartialEq` for type `Lock::Unlocked`
    //~| ERROR conflicting implementations of trait `std::cmp::PartialEq` for type `Lock::Unlocked`
    //~| ERROR conflicting implementations of trait `std::cmp::PartialEq` for type `Lock::Unlocked`
    //~| ERROR conflicting implementations of trait `std::cmp::Eq` for type `Lock::Unlocked`
    //~| ERROR conflicting implementations of trait `std::clone::Clone` for type `Lock::Unlocked`
    //~| ERROR conflicting implementations of trait `std::fmt::Debug` for type `Lock::Unlocked`
}
