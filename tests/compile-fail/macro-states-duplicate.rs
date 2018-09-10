#[macro_use]
extern crate sm;

sm!{
//~^ ERROR the name `Unlocked` is defined multiple times
//~| ERROR the name `Unlocked` is defined multiple times
//~| ERROR conflicting implementations of trait `sm::State` for type `Lock::Unlocked`
//~| ERROR conflicting implementations of trait `std::marker::Copy` for type `Lock::Unlocked`
//~| ERROR conflicting implementations of trait `std::cmp::PartialEq` for type `Lock::Unlocked`
//~| ERROR conflicting implementations of trait `std::cmp::PartialEq` for type `Lock::Unlocked`
//~| ERROR conflicting implementations of trait `std::cmp::PartialEq` for type `Lock::Unlocked`
//~| ERROR conflicting implementations of trait `std::cmp::Eq` for type `Lock::Unlocked`
//~| ERROR conflicting implementations of trait `std::clone::Clone` for type `Lock::Unlocked`
//~| ERROR conflicting implementations of trait `std::fmt::Debug` for type `Lock::Unlocked`
//~| ERROR conflicting implementations of trait `sm::AsEnum<Lock::Unlocked>` for type `Lock::Machine<Lock::Unlocked>`

    Lock {
        States { Unlocked, Unlocked }
    }
}
