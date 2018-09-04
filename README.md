[![Crate Version](https://img.shields.io/crates/v/sm.svg?logo=rust&label=crates.io&logoColor=white&colorB=brightgreen)](https://crates.io/crates/sm)
[![Chat](https://img.shields.io/discord/477552212156088320.svg?logo=discord&label=discord)](https://discord.gg/Kc4qZWE "Ask a question or just enjoy your stay!")
[![Build Status](https://img.shields.io/appveyor/ci/JeanMertz/sm/master.svg?logo=appveyor&label=appveyor)](https://ci.appveyor.com/project/JeanMertz/sm/branch/master)
[![Build Status](https://img.shields.io/circleci/project/github/rusty-rockets/sm/master.svg?logo=circleci&label=circleci)](https://circleci.com/gh/rusty-rockets/sm/tree/master)
[![Build Status](https://img.shields.io/travis/rusty-rockets/sm/master.svg?logo=travis&label=travis)](https://travis-ci.org/rusty-rockets/sm)
[![Coverage Status](https://img.shields.io/codecov/c/github/rusty-rockets/sm/master.svg?logo=codeship&label=codecov)](https://codecov.io/gh/rusty-rockets/sm)
[![Average time to resolve an issue](https://isitmaintained.com/badge/resolution/rusty-rockets/sm.svg)](https://isitmaintained.com/project/rusty-rockets/sm "Average time to resolve an issue")
[![Percentage of issues still open](https://isitmaintained.com/badge/open/rusty-rockets/sm.svg)](https://isitmaintained.com/project/rusty-rockets/sm "Percentage of issues still open")

# 💋 SM – a static State Machine library

SM allows you to define a collection of states and events using Rust's type
system. You can query the current state, and execute transitions between
states. State machine usage is validated at compile-time. Undefined behavior
is not an option.

The implementation ensures a zero-sized abstraction that uses Rust's
type-system and ownership model to guarantee valid transitions between
states using events, and makes sure previous states are no longer usable
after transitioning away to another state. Rust validates correct usage of
the state machine at compile-time, no runtime checking occurs when using the
library.

The library exposes the `sm!` macro, which allows you to declaratively build
the state machine.

## Examples

### Quick Example

```rust
#[macro_use] extern crate sm;

sm! {
    Lock { Locked, Unlocked, Broken }

    TurnKey {
        Locked => Unlocked
        Unlocked => Locked
    }

    Break {
        Locked => Broken
        Unlocked => Broken
    }
}

fn main() {
    use Lock::*;
    let sm = Machine::new(Locked);
    let sm = sm.event(TurnKey);

    assert_eq!(sm.state(), Unlocked);
}
```

### Descriptive Example

The below example explains step-by-step how to create a new state machine
using the provided macro, and then how to use the created machine in your
code by querying states, and transitioning between states by triggering
events.

#### Declaring a new State Machine

First, we import the macro from the crate:

```rust
#[macro_use] extern crate sm;
```

Next, we initiate the macro declaration:

```rust
sm! {
```

Then, provide a name for the machine, and declare its states:

```rust
    Lock { Locked, Unlocked, Broken }
```

Finally, we declare one or more events and the associated transitions:

```rust
    TurnKey {
        Locked => Unlocked
        Unlocked => Locked
    }

    Break {
        Locked => Broken
        Unlocked => Broken
    }
}
```

And we're done. We've defined our state machine structure, and the valid
transitions, and can now use this state machine in our code.

#### Using your State Machine

You can initialise the machine as follows:

```rust
let sm = Lock::Machine::new(Lock::Locked);
```

We can make this a bit less verbose by bringing our machine into scope:

```rust
use Lock::*;
let sm = Machine::new(Locked);
```

We've initialised our machine in the `Locked` state. You can get the current
state of the machine by sending the `state()` method to the machine:

```rust
let state = sm.state();
assert_eq!(state, Locked);
```

You can use the above method to model your own domain logic based on the
current state of the machine using any conditional expression.

Finally, as per our declaration, we can transition this machine to the
`Unlocked` state by triggering the `TurnKey` event:

```rust
let sm = sm.event(TurnKey);
assert_eq!(sm.state(), Unlocked);
```

#### A word about Type-Safety and Ownership

It's important to realise that we've _consumed_ the original machine in the
above example, and got a newly initialised machine back in the `Unlocked`
state.

This allows us to safely use the machine without having to worry about
multiple readers using the machine in different states.

All these checks are applied on compile-time, so the following example would
fail to compile:

```rust
let sm2 = sm.event(TurnKey);
assert_eq!(sm.state(), Locked);
```

This fails with the following compilation error:

```rust
error[E0382]: use of moved value: `sm`
  --> src/lib.rs:140:12
   |
14 | let sm2 = sm.event(TurnKey);
   |           -- value moved here
15 | assert_eq!(sm.state(), Locked);
   |            ^^ value used here after move
   |
   = note: move occurs because `sm` has type `Lock::Machine<Lock::Locked>`, which does not implement the `Copy` trait
```

Similarly, we cannot execute undefined transitions, these are also caught by
the compiler:

```rust
let sm = sm.event(TurnKey);
assert_eq!(sm.state(), Broken);
```

This fails with the following compilation error:

```rust
error[E0599]: no method named `event` found for type `Lock::Machine<Lock::Broken>` in the current scope
  --> src/lib.rs:246:13
   |
3  | / sm! {
4  | |    Lock { Locked, Unlocked, Broken }
5  | |    TurnKey {
6  | |        Locked => Unlocked
...  |
13 | |    }
14 | | }
   | |_- method `event` not found for this
...
19 |   let sm = sm.event(TurnKey);
   |               ^^^^^
   |
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `event`, perhaps you need to implement it:
           candidate #1: `Lock::Transition`
   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
```

The error message is not great (and can potentially be improved in the
future), but any error telling you `event` is not implemented, or the passed
in event type is invalid is an indication that you are trying to execute an
illegal state transition.

#### The End 💋

And that's it! There's nothing else to it, except a declarative – and easy
to read – state machine construction macro, and a type-safe and
ownership-focused way of dealing with states and transitions, without any
runtime overhead.

**Go forth and transition!**

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
