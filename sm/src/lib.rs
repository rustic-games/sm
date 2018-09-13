//! TODO: documentation

#![no_std]
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
#![feature(tool_lints)]
#![deny(clippy::all)]

use core::fmt;

/// State is a custom [marker trait][m] that allows [unit-like structs][u] to be
/// used as states in a state machine.
///
/// If you are using the `sm!` macro, then there is no need to interact with
/// this trait.
///
/// [m]: https://doc.rust-lang.org/std/marker/index.html
/// [u]: https://doc.rust-lang.org/book/second-edition/ch05-01-defining-structs.html#unit-like-structs-without-any-fields
pub trait State: fmt::Debug + Eq + Clone {}

/// Event is a custom [marker trait][m] that allows [unit-like structs][u] to be
/// used as states in a state machine.
///
/// If you are using the `sm!` macro, then there is no need to interact with
/// this trait.
///
/// [m]: https://doc.rust-lang.org/std/marker/index.html
/// [u]: https://doc.rust-lang.org/book/second-edition/ch05-01-defining-structs.html#unit-like-structs-without-any-fields
pub trait Event: fmt::Debug + Eq {}

/// Machine provides the method required to query a state machine for its
/// current state.
///
/// If you are using the `sm!` macro, then there is no need to interact with
/// this trait.
pub trait Machine: fmt::Debug + Eq {
    /// State represents the current (static) state of the state machine.
    type State;

    /// state is a convenience method to query the current state of the state
    /// machine.
    fn state(&self) -> Self::State;
}

/// Transition provides the method required to transition from one state to
/// another.
///
/// If you are using the `sm!` macro, then there is no need to interact with
/// this trait.
pub trait Transition<T, E> {
    /// transition consumes the state machine and returns a new state machine in
    /// the correct state, based on the passed in event.
    fn transition(self, event: E) -> T;
}

/// AsEnum provides the method to convert a state machine instance to an enum
/// type.
///
/// If you are using the `sm!` macro, then there is no need to interact with
/// this trait.
pub trait AsEnum<S: State> {
    /// Enum is an enum that represents the current state machine as an enum
    /// variant.
    type Enum;

    /// as_enum consumes the state machine and returns a new enum variant that
    /// represents the consumed state machine.
    fn as_enum(self) -> Self::Enum;
}
