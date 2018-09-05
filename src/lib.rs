//! # 💋 SM – a static State Machine library
//!
//! SM allows you to define a collection of states and events using Rust's type
//! system. You can query the current state, and execute transitions between
//! states. State machine usage is validated at compile-time. Undefined behavior
//! is not an option.
//!
//! The implementation ensures a zero-sized abstraction that uses Rust's
//! type-system and ownership model to guarantee valid transitions between
//! states using events, and makes sure previous states are no longer usable
//! after transitioning away to another state. Rust validates correct usage of
//! the state machine at compile-time, no runtime checking occurs when using the
//! library.
//!
//! The library exposes the `sm!` macro, which allows you to declaratively build
//! the state machine.
//!
//! ## Examples
//!
//! ### Quick Example
//!
//! ```rust
//! #[macro_use] extern crate sm;
//!
//! sm! {
//!     Lock { Locked, Unlocked, Broken }
//!
//!     TurnKey {
//!         Locked => Unlocked
//!         Unlocked => Locked
//!     }
//!
//!     Break {
//!         Locked => Broken
//!         Unlocked => Broken
//!     }
//! }
//!
//! fn main() {
//!     use Lock::*;
//!     let sm = Machine::new(Locked);
//!     let sm = sm.event(TurnKey);
//!
//!     assert_eq!(sm.state(), Unlocked);
//! }
//! ```
//!
//! ### Descriptive Example
//!
//! The below example explains step-by-step how to create a new state machine
//! using the provided macro, and then how to use the created machine in your
//! code by querying states, and transitioning between states by triggering
//! events.
//!
//! #### Declaring a new State Machine
//!
//! First, we import the macro from the crate:
//!
//! ```rust
//! #[macro_use] extern crate sm;
//! ```
//!
//! Next, we initiate the macro declaration:
//!
//! ```rust
//! # #[macro_use] extern crate sm;
//! sm! {
//! # Lock { Locked, Unlocked, Broken }
//! # }
//! ```
//!
//! Then, provide a name for the machine, and declare its states:
//!
//! ```rust
//! # #[macro_use] extern crate sm;
//! # sm! {
//!     Lock { Locked, Unlocked, Broken }
//! # }
//! ```
//!
//! Finally, we declare one or more events and the associated transitions:
//!
//! ```rust
//! # #[macro_use] extern crate sm;
//! # sm! {
//! #    Lock { Locked, Unlocked, Broken }
//!     TurnKey {
//!         Locked => Unlocked
//!         Unlocked => Locked
//!     }
//!
//!     Break {
//!         Locked => Broken
//!         Unlocked => Broken
//!     }
//! }
//! ```
//!
//! And we're done. We've defined our state machine structure, and the valid
//! transitions, and can now use this state machine in our code.
//!
//! #### Using your State Machine
//!
//! You can initialise the machine as follows:
//!
//! ```rust
//! # #[macro_use] extern crate sm;
//! # sm! {
//! #    Lock { Locked, Unlocked, Broken }
//! #    TurnKey {
//! #        Locked => Unlocked
//! #        Unlocked => Locked
//! #    }
//! #
//! #    Break {
//! #        Locked => Broken
//! #        Unlocked => Broken
//! #    }
//! # }
//! #
//! # fn main() {
//! let sm = Lock::Machine::new(Lock::Locked);
//! # }
//! ```
//!
//! We can make this a bit less verbose by bringing our machine into scope:
//!
//! ```rust
//! # #[macro_use] extern crate sm;
//! # sm! {
//! #    Lock { Locked, Unlocked, Broken }
//! #    TurnKey {
//! #        Locked => Unlocked
//! #        Unlocked => Locked
//! #    }
//! #
//! #    Break {
//! #        Locked => Broken
//! #        Unlocked => Broken
//! #    }
//! # }
//! #
//! # fn main() {
//! use Lock::*;
//! let sm = Machine::new(Locked);
//! # }
//! ```
//!
//! We've initialised our machine in the `Locked` state. You can get the current
//! state of the machine by sending the `state()` method to the machine:
//!
//! ```rust
//! # #[macro_use] extern crate sm;
//! # sm! {
//! #    Lock { Locked, Unlocked, Broken }
//! #    TurnKey {
//! #        Locked => Unlocked
//! #        Unlocked => Locked
//! #    }
//! #
//! #    Break {
//! #        Locked => Broken
//! #        Unlocked => Broken
//! #    }
//! # }
//! #
//! # fn main() {
//! # use Lock::*;
//! # let sm = Machine::new(Locked);
//! let state = sm.state();
//! assert_eq!(state, Locked);
//! # }
//! ```
//!
//! You can use the above method to model your own domain logic based on the
//! current state of the machine using any conditional expression.
//!
//! Finally, as per our declaration, we can transition this machine to the
//! `Unlocked` state by triggering the `TurnKey` event:
//!
//! ```rust
//! # #[macro_use] extern crate sm;
//! # sm! {
//! #    Lock { Locked, Unlocked, Broken }
//! #    TurnKey {
//! #        Locked => Unlocked
//! #        Unlocked => Locked
//! #    }
//! #
//! #    Break {
//! #        Locked => Broken
//! #        Unlocked => Broken
//! #    }
//! # }
//! #
//! # fn main() {
//! # use Lock::*;
//! # let sm = Machine::new(Locked);
//! let sm = sm.event(TurnKey);
//! assert_eq!(sm.state(), Unlocked);
//! # }
//! ```
//!
//! #### A word about Type-Safety and Ownership
//!
//! It's important to realise that we've _consumed_ the original machine in the
//! above example, and got a newly initialised machine back in the `Unlocked`
//! state.
//!
//! This allows us to safely use the machine without having to worry about
//! multiple readers using the machine in different states.
//!
//! All these checks are applied on compile-time, so the following example would
//! fail to compile:
//!
//! ```rust,compile_fail
//! # #[macro_use] extern crate sm;
//! # sm! {
//! #    Lock { Locked, Unlocked, Broken }
//! #    TurnKey {
//! #        Locked => Unlocked
//! #        Unlocked => Locked
//! #    }
//! #
//! #    Break {
//! #        Locked => Broken
//! #        Unlocked => Broken
//! #    }
//! # }
//! #
//! # fn main() {
//! # use Lock::*;
//! # let sm = Machine::new(Locked);
//! let sm2 = sm.event(TurnKey);
//! assert_eq!(sm.state(), Locked);
//! # }
//! ```
//!
//! This fails with the following compilation error:
//!
//! ```text
//! error[E0382]: use of moved value: `sm`
//!   --> src/lib.rs:140:12
//!    |
//! 14 | let sm2 = sm.event(TurnKey);
//!    |           -- value moved here
//! 15 | assert_eq!(sm.state(), Locked);
//!    |            ^^ value used here after move
//!    |
//!    = note: move occurs because `sm` has type `Lock::Machine<Lock::Locked>`, which does not implement the `Copy` trait
//! ```
//!
//! Similarly, we cannot execute undefined transitions, these are also caught by
//! the compiler:
//!
//! ```rust,compile_fail
//! # #[macro_use] extern crate sm;
//! # sm! {
//! #    Lock { Locked, Unlocked, Broken }
//! #    TurnKey {
//! #        Locked => Unlocked
//! #        Unlocked => Locked
//! #    }
//! #
//! #    Break {
//! #        Locked => Broken
//! #        Unlocked => Broken
//! #    }
//! # }
//! #
//! # fn main() {
//! # use Lock::*;
//! # let sm = Machine::new(Broken);
//! let sm = sm.event(TurnKey);
//! assert_eq!(sm.state(), Broken);
//! # }
//! ```
//!
//! This fails with the following compilation error:
//!
//! ```text
//! error[E0599]: no method named `event` found for type `Lock::Machine<Lock::Broken>` in the current scope
//!   --> src/lib.rs:246:13
//!    |
//! 3  | / sm! {
//! 4  | |    Lock { Locked, Unlocked, Broken }
//! 5  | |    TurnKey {
//! 6  | |        Locked => Unlocked
//! ...  |
//! 13 | |    }
//! 14 | | }
//!    | |_- method `event` not found for this
//! ...
//! 19 |   let sm = sm.event(TurnKey);
//!    |               ^^^^^
//!    |
//!    = help: items from traits can only be used if the trait is implemented and in scope
//!    = note: the following trait defines an item `event`, perhaps you need to implement it:
//!            candidate #1: `Lock::Transition`
//!    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
//! ```
//!
//! The error message is not great (and can potentially be improved in the
//! future), but any error telling you `event` is not implemented, or the passed
//! in event type is invalid is an indication that you are trying to execute an
//! illegal state transition.
//!
//! #### The End 💋
//!
//! And that's it! There's nothing else to it, except a declarative – and easy
//! to read – state machine construction macro, and a type-safe and
//! ownership-focused way of dealing with states and transitions, without any
//! runtime overhead.
//!
//! **Go forth and transition!**

#![no_std]

/// Generate the declaratively described state machine diagram.
///
/// See the main crate documentation for more details.
#[macro_export]
macro_rules! sm {
    (
        $name:ident { $($state:ident),+ $(,)* }

        $($event:ident {
            $($from:ident => $to:ident)+
        })*
    ) => {
        #[allow(non_snake_case)]
        pub mod $name {
            pub trait State {}
            pub trait Event {}
            pub trait Transition<S: State, E: Event> {
                fn event(self, event: E) -> Machine<S>;
            }

            #[derive(PartialEq, Eq, Debug)]
            pub struct Machine<S: State>(pub S);

            impl<S> Machine<S>
            where
                S: State + Clone,
            {
                pub fn new(state: S) -> Self {
                    Machine(state)
                }

                #[allow(dead_code)]
                pub fn state(&self) -> S {
                    self.0.clone()
                }
            }

            $(
                #[derive(Copy, Eq, Debug)]
                pub struct $state;
                impl State for $state {}
                impl Clone for $state {
                    fn clone(&self) -> $state { *self }
                }

                impl PartialEq<$state> for $state {
                    fn eq(&self, _: & $state) -> bool {
                        true
                    }
                }
            )*

            sm!{@recurse ($($state),*), ()}

            $(
                #[derive(PartialEq, Eq, Debug)]
                pub struct $event;
                impl Event for $event {}

                $(
                    impl Transition<$to, $event> for Machine<$from> {
                        fn event(self, _: $event) -> Machine<$to> {
                            Machine::new($to)
                        }
                    }
                )*
            )*
        }
    };

    (@recurse ($state:ident, $($other:ident),+), ($($old:ident),*)) => {
        $(
            impl PartialEq<$other> for $state {
                fn eq(&self, _: & $other) -> bool {
                    false
                }
            }
        )*

        $(
            impl PartialEq<$old> for $state {
                fn eq(&self, _: & $old) -> bool {
                    false
                }
            }
        )*

        sm!{@recurse ($($other),*), ($($old,)* $state)}
    };

    (@recurse ($state:ident), ($($old:ident),*)) => {
        $(
            impl PartialEq<$old> for $state {
                fn eq(&self, _: & $old) -> bool {
                    false
                }
            }
        )*
    };
}

#[cfg(test)]
mod tests {
    sm!{
        GameLoop { Idle, Simulating, Rendering }

        None {
            Simulating => Idle
            Rendering => Idle
            Idle => Idle
        }

        Simulate {
            Idle => Simulating
        }

        Render {
            Idle => Rendering
        }
    }

    #[test]
    fn it_works() {
        use self::GameLoop::*;

        let sm1 = Machine::new(Idle);
        assert_eq!(sm1, Machine(Idle));
        assert_eq!(sm1.state(), Idle);

        let sm2 = sm1.event(Simulate);
        assert_eq!(sm2, Machine(Simulating));
        assert_eq!(sm2.state(), Simulating);

        let sm3 = sm2.event(None);
        assert_eq!(sm3, Machine(Idle));
        assert_eq!(sm3.state(), Idle);

        let sm4 = sm3.event(Render);
        assert_eq!(sm4, Machine(Rendering));
        assert_eq!(sm4.state(), Rendering);

        let sm5 = sm4.event(None);
        assert_eq!(sm5, Machine(Idle));

        let sm6 = sm5.event(None);
        assert_eq!(sm6, Machine(Idle));

        let state = sm6.state();
        assert_eq!(state, Idle);
        assert_ne!(state, Rendering);
        assert_ne!(state, Simulating);
    }
}
