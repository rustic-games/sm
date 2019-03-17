//! Using this library, you declaratively define your state machines as as set
//! of _states_, connected via _transitions_, triggered by _events_. You can
//! query the current state of the machine, or pattern match against all
//! possible machine variants.
//!
//! The implementation ensures a zero-sized abstraction that uses Rust's
//! type-system and ownership model to guarantee valid transitions between
//! states using events, and makes sure previous states are no longer accessible
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
//! extern crate sm;
//! use sm::sm;
//!
//! sm! {
//!     Lock {
//!         InitialStates { Locked, Unlocked }
//!
//!         TurnKey {
//!             Locked => Unlocked
//!             Unlocked => Locked
//!         }
//!
//!         Break {
//!             Locked, Unlocked => Broken
//!         }
//!     }
//! }
//!
//! fn main() {
//!     use Lock::*;
//!     let lock = Machine::new(Locked);
//!     let lock = lock.transition(TurnKey);
//!
//!     assert_eq!(lock.state(), Unlocked);
//!     assert_eq!(lock.trigger().unwrap(), TurnKey);
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
//! extern crate sm;
//! use sm::sm;
//! ```
//!
//! Next, we initiate the macro declaration:
//!
//! ```rust
//! # extern crate sm;
//! # use sm::sm;
//! sm! {
//! #   Lock {
//! #       InitialStates { Locked, Unlocked }
//! #   }
//! # }
//! #
//! # fn main() {}
//! ```
//!
//! Then, provide a name for the machine, and declare a list of allowed initial
//! states:
//!
//! ```rust
//! # extern crate sm;
//! # use sm::sm;
//! # sm! {
//!     Lock {
//!         InitialStates { Locked, Unlocked }
//! #   }
//! # }
//! #
//! # fn main() {}
//! ```
//!
//! Finally, we declare one or more events and the associated transitions:
//!
//! ```rust
//! # extern crate sm;
//! # use sm::sm;
//! # sm! {
//! #   Lock {
//! #       InitialStates { Locked, Unlocked }
//! #
//!         TurnKey {
//!             Locked => Unlocked
//!             Unlocked => Locked
//!         }
//!
//!         Break {
//!             Locked, Unlocked => Broken
//!         }
//!     }
//! }
//! #
//! # fn main() {}
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
//! # extern crate sm;
//! # use sm::sm;
//! # sm! {
//! #   Lock {
//! #       InitialStates { Locked, Unlocked }
//! #
//! #       TurnKey {
//! #           Locked => Unlocked
//! #           Unlocked => Locked
//! #       }
//! #
//! #       Break {
//! #           Locked, Unlocked => Broken
//! #       }
//! #   }
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
//! # extern crate sm;
//! # use sm::sm;
//! # sm! {
//! #   Lock {
//! #       InitialStates { Locked, Unlocked }
//! #
//! #       TurnKey {
//! #           Locked => Unlocked
//! #           Unlocked => Locked
//! #       }
//! #
//! #       Break {
//! #           Locked, Unlocked => Broken
//! #       }
//! #   }
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
//! # extern crate sm;
//! # use sm::sm;
//! # sm! {
//! #   Lock {
//! #       InitialStates { Locked, Unlocked }
//! #
//! #       TurnKey {
//! #           Locked => Unlocked
//! #           Unlocked => Locked
//! #       }
//! #
//! #       Break {
//! #           Locked, Unlocked => Broken
//! #       }
//! #   }
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
//! While you _can_ use `sm.state()` with conditional branching to execute your
//! code based on the current state, this can be a bit tedious, it's less
//! idiomatic, and it prevents you from using one extra compile-time validation
//! tool in our toolbox: using Rust's exhaustive pattern matching requirement to
//! ensure you've covered all possible state variants in your business logic.
//!
//! While `sm.state()` returns the state as a unit-like struct (which itself is
//! a [ZST], or Zero Sized Type), you can use the `sm.as_enum()` method to get
//! the state machine back as an enum variant.
//!
//! [ZST]:
//! https://doc.rust-lang.org/nomicon/exotic-sizes.html#zero-sized-types-zsts
//!
//! Using the enum variant and pattern matching, you are able to do the
//! following:
//!
//! ```rust
//! # extern crate sm;
//! # use sm::sm;
//! # sm! {
//! #   Lock {
//! #       InitialStates { Locked, Unlocked }
//! #
//! #       TurnKey {
//! #           Locked => Unlocked
//! #           Unlocked => Locked
//! #       }
//! #
//! #       Break {
//! #           Locked, Unlocked => Broken
//! #       }
//! #   }
//! # }
//! #
//! # fn main() {
//! # use Lock::*;
//! # let sm = Machine::new(Locked);
//! use Lock::Variant::*;
//!
//! match sm.as_enum() {
//!     InitialLocked(m) => {
//!         assert_eq!(m.state(), Locked);
//!         assert!(m.trigger().is_none());
//!     }
//!     InitialUnlocked(m) => {
//!         assert_eq!(m.state(), Unlocked);
//!         assert!(m.trigger().is_none());
//!     }
//!     LockedByTurnKey(m) => {
//!         assert_eq!(m.state(), Locked);
//!         assert_eq!(m.trigger().unwrap(), TurnKey);
//!     }
//!     UnlockedByTurnKey(m) => {
//!         assert_eq!(m.state(), Unlocked);
//!         assert_eq!(m.trigger().unwrap(), TurnKey);
//!     }
//!     BrokenByBreak(m) => {
//!         assert_eq!(m.state(), Broken);
//!         assert_eq!(m.trigger().unwrap(), Break);
//!     }
//! }
//! # }
//! ```
//!
//! Each state configured with `InitialStates` has its own variant named
//! `Initial<State>`. Next to those, each valid state + event combination also
//! has its own variant, named `<state>By<event>`.
//!
//! The compiler won't be satisfied until you've either exhausted all possible
//! enum variants, or you explicitly opt-out of matching all variants, either
//! way, you can be much more confident that your code won't break if you add a
//! new state down the road, but forget to add it to a pattern match somewhere
//! deep inside your code-base.
//!
//! To transition this machine to the `Unlocked` state, we send the `transition`
//! method, using the `TurnKey` event:
//!
//! ```rust
//! # extern crate sm;
//! # use sm::sm;
//! # sm! {
//! #   Lock {
//! #       InitialStates { Locked, Unlocked }
//! #
//! #       TurnKey {
//! #           Locked => Unlocked
//! #           Unlocked => Locked
//! #       }
//! #
//! #       Break {
//! #           Locked, Unlocked => Broken
//! #       }
//! #   }
//! # }
//! #
//! # fn main() {
//! # use Lock::*;
//! # let sm = Machine::new(Locked);
//! let sm = sm.transition(TurnKey);
//! assert_eq!(sm.state(), Unlocked);
//! # }
//! ```
//!
//! Because multiple events can lead to a single state, it's also important to
//! be able to determine what event caused the machine to transition to the
//! current state. We can ask this information using the `trigger()` method:
//!
//! ```rust
//! # extern crate sm;
//! # use sm::sm;
//! # sm! {
//! #   Lock {
//! #       InitialStates { Locked, Unlocked }
//! #
//! #       TurnKey {
//! #           Locked => Unlocked
//! #           Unlocked => Locked
//! #       }
//! #
//! #       Break {
//! #           Locked, Unlocked => Broken
//! #       }
//! #   }
//! # }
//! #
//! # fn main() {
//! # use Lock::*;
//! # let sm = Machine::new(Locked);
//! # let sm = sm.transition(TurnKey);
//! # assert_eq!(sm.state(), Unlocked);
//! assert_eq!(sm.trigger().unwrap(), TurnKey);
//! # }
//! ```
//!
//! The `trigger()` method returns `None` if no state transition has taken place
//! yet (ie. the machine is still in its initial state), and `Some(Event)` if
//! one or more transitions have taken place.
//!
//! #### A word about Type-Safety and Ownership
//!
//! It's important to realise that we've _consumed_ the original machine in the
//! above example when we transitioned the machine to a different state, and got
//! a newly initialised machine back in the `Unlocked` state.
//!
//! This allows us to safely use the machine without having to worry about
//! multiple readers using the machine in different states.
//!
//! All these checks are applied on compile-time, so the following example would
//! fail to compile:
//!
//! ```rust,compile_fail
//! # extern crate sm;
//! # use sm::sm;
//! # sm! {
//! #   Lock {
//! #       InitialStates { Locked, Unlocked }
//! #
//! #       TurnKey {
//! #           Locked => Unlocked
//! #           Unlocked => Locked
//! #       }
//! #
//! #       Break {
//! #           Locked, Unlocked => Broken
//! #       }
//! #   }
//! # }
//! #
//! # fn main() {
//! # use Lock::*;
//! # let sm = Machine::new(Locked);
//! let sm2 = sm.transition(TurnKey);
//! assert_eq!(sm.state(), Locked);
//! # }
//! ```
//!
//! This fails with the following compilation error:
//!
//! ```text
//! error[E0382]: use of moved value: `sm`
//!   --> src/lib.rs:315:12
//!    |
//! 22 | let sm2 = sm.transition(TurnKey);
//!    |           -- value moved here
//! 23 | assert_eq!(sm.state(), Locked);
//!    |            ^^ value used here after move
//!    |
//!    = note: move occurs because `sm` has type `Lock::Machine<Lock::Locked>`, which does not implement the `Copy` trait
//! ```
//!
//! Similarly, we cannot execute undefined transitions, these are also caught by
//! the compiler:
//!
//! ```rust,compile_fail
//! # extern crate sm;
//! # use sm::sm;
//! # sm! {
//! #   Lock {
//! #       InitialStates { Locked, Unlocked }
//! #
//! #       TurnKey {
//! #           Locked => Unlocked
//! #           Unlocked => Locked
//! #       }
//! #
//! #       Break {
//! #           Locked, Unlocked => Broken
//! #       }
//! #   }
//! # }
//! #
//! # fn main() {
//! # use Lock::*;
//! # let sm = Machine::new(Locked);
//! # let sm = sm.transition(Break);
//! assert_eq!(sm.state(), Broken);
//!
//! let sm = sm.transition(TurnKey);
//! # }
//! ```
//!
//! This fails with the following compilation error:
//!
//! ```text
//! error[E0599]: no method named `transition` found for type `Lock::Machine<Lock::Broken>` in the current scope
//!   --> src/lib.rs:360:13
//!    |
//! 4  | sm! {
//!    | --- method `transition` not found for this
//! ...
//! 25 | let sm = sm.transition(TurnKey);
//!    |             ^^^^^^^^^^
//!    |
//!    = help: items from traits can only be used if the trait is implemented and in scope
//!    = note: the following trait defines an item `transition`, perhaps you need to implement it:
//!            candidate #1: `sm::Transition`
//! ```
//!
//! The error message is not great (and can potentially be improved in the
//! future), but any error telling you `transition` is not implemented, or the
//! passed in event type is invalid is an indication that you are trying to
//! execute an illegal state transition.
//!
//! Finally, we are confined to initialising a new machine in only the states
//! that we defined in `InitialStates`:
//!
//! ```rust,compile_fail
//! # extern crate sm;
//! # use sm::sm;
//! # sm! {
//! #   Lock {
//! #       InitialStates { Locked, Unlocked }
//! #
//! #       TurnKey {
//! #           Locked => Unlocked
//! #           Unlocked => Locked
//! #       }
//! #
//! #       Break {
//! #           Locked, Unlocked => Broken
//! #       }
//! #   }
//! # }
//! #
//! # fn main() {
//! # use Lock::*;
//! let sm = Machine::new(Broken);
//! # assert_eq!(sm.state(), Broken);
//! # }
//! ```
//!
//! This results in the following error:
//!
//! ```text
//! error[E0277]: the trait bound `Lock::Broken: sm::InitialState` is not satisfied
//!   --> src/lib.rs:417:10
//!    |
//! 21 | let sm = Machine::new(Broken);
//!    |          ^^^^^^^^^^^^ the trait `sm::InitialState` is not implemented for `Lock::Broken`
//!    |
//!    = note: required because of the requirements on the impl of `sm::NewMachine<Lock::Broken>` for `Lock::Machine<Lock::Broken>`
//! ```
//!
//! #### The End 👋
//!
//! And that's it! There's nothing else to it, except a declarative – and easy
//! to read – state machine construction macro, and a type-safe and
//! ownership-focused way of dealing with states and transitions, without any
//! runtime overhead.
//!
//! **Go forth and transition!**

#![forbid(
    future_incompatible,
    macro_use_extern_crate,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_compatibility,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    variant_size_differences,
)]
#![warn(
    non_snake_case,
    rust_2018_idioms,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    unused,
)]
#![deny(clippy::all)]

use core::fmt;

#[cfg(feature = "macro")]
pub use sm_macro::sm;

/// State is a custom [marker trait][m] that allows [unit-like structs][u] to be
/// used as states in a state machine.
///
/// If you are using the `sm!` macro, then there is no need to interact with
/// this trait.
///
/// [m]: https://doc.rust-lang.org/std/marker/index.html
/// [u]: https://doc.rust-lang.org/book/second-edition/ch05-01-defining-structs.html#unit-like-structs-without-any-fields
pub trait State: fmt::Debug + Eq + Clone {}

/// InitialState is a custom [marker trait][m] that allows a state to be used as
/// the initial state in a state machine. This trait is a superset of the
/// `State` trait.
///
/// If you are using the `sm!` macro, then there is no need to interact with
/// this trait.
///
/// [m]: https://doc.rust-lang.org/std/marker/index.html
pub trait InitialState: State {}

/// Event is a custom [marker trait][m] that allows [unit-like structs][u] to be
/// used as states in a state machine.
///
/// If you are using the `sm!` macro, then there is no need to interact with
/// this trait.
///
/// [m]: https://doc.rust-lang.org/std/marker/index.html
/// [u]: https://doc.rust-lang.org/book/second-edition/ch05-01-defining-structs.html#unit-like-structs-without-any-fields
pub trait Event: fmt::Debug + Eq + Clone {}

/// Machine provides the method required to query a state machine for its
/// current state.
///
/// If you are using the `sm!` macro, then there is no need to interact with
/// this trait.
pub trait Machine: fmt::Debug + Eq {
    /// State represents the current (static) state of the state machine.
    type State: State;

    /// Event represents the (optional) event that resulted in the current state
    /// of the machine.
    type Event: Event;

    /// state allows you to query the current state of the state machine.
    fn state(&self) -> Self::State;

    /// trigger allows you to query the event that triggered the current state
    /// of the machine.
    ///
    /// This returns an `Option`, which is `None` if the machine is in its
    /// initial state, caused by initialisation, not by an even-based
    /// transition.
    fn trigger(&self) -> Option<Self::Event>;
}

/// Initializer defines the `new` method on a machine, that accepts any state
/// marked as `InitialState`, and returns a new machine.
///
/// If you are using the `sm!` macro, then there is no need to interact with
/// this trait.
pub trait Initializer<S: InitialState> {
    /// Machine represents the machine which the implemented initialiser should
    /// return.
    type Machine: Machine<State = S, Event = NoneEvent>;

    /// new initialises a new machine, based on the provided `InitialState` as
    /// input.
    fn new(state: S) -> Self::Machine;
}

/// Transition provides the method required to transition from one state to
/// another.
///
/// If you are using the `sm!` macro, then there is no need to interact with
/// this trait.
pub trait Transition<E: Event>: fmt::Debug {
    /// Machine represents the machine on which the implemented transformation
    /// should execute.
    type Machine: Machine;

    /// transition consumes the state machine and returns a new machine in the
    /// correct state, based on the passed in event.
    fn transition(self, event: E) -> Self::Machine;
}

/// AsEnum provides the method to convert a state machine instance to an enum
/// type.
///
/// If you are using the `sm!` macro, then there is no need to interact with
/// this trait.
pub trait AsEnum: fmt::Debug {
    /// Enum is an enum that represents the current state machine as an enum
    /// variant.
    type Enum;

    /// as_enum consumes the state machine and returns a new enum variant that
    /// represents the consumed state machine.
    fn as_enum(self) -> Self::Enum;
}

/// NoneEvent is a semi-private event struct that is used to allow the
/// `Initializer` trait implementations to provide a simple API to initialise a
/// new machine.
///
/// Using this struct, the following works:
///
/// ```rust,ignore
/// let sm = TurnStile::new(Locked); // => TurnStile<Locked, None>
/// ```
///
/// Otherwise, we'd need to provide type annotations (using any available
/// events) for every machine invocation:
///
/// ```rust,ignore
/// let sm: TurnStile<Locked, Push> = TurnStile::new(Locked); // => TurnStile<Locked, None>
/// ```
///
/// In practice, you will never deal with this struct, as it's used as a
/// concrete type that is swapped for the `None` option at compile time.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NoneEvent;
impl Event for NoneEvent {}
