<img src="/rocket.svg" width="50" align="left" title="Rusty Rockets">

_SM serves as one of the building blocks for [an open-source game about space
engineering and exploration][rkt]. **This library is in active use and
development.**_

[rkt]: https://rustic.games

<br />

<img src="/logo.svg" align="right" title="SM logo by Jean Mertz" width="400" />

SM aims to be a **safe**, **fast** and **simple** state machine library.

- **safe** — Rust's type system, ownership model and exhaustive pattern matching
  prevent you from mis-using your state machines

- **fast** — zero runtime overhead, the machine is 100% static, all validation
  happens at compile-time

- **simple** — five traits, and one optional declarative macro, control-flow
  only, no business logic attached

---

<div align="right">

[![Latest Crate Version](https://img.shields.io/crates/v/sm.svg?logo=rust&label=version&logoColor=white&colorB=brightgreen)](https://crates.io/crates/sm "The latest released version on crates.io.")
[![Discord Chat](https://img.shields.io/discord/477552212156088320.svg?logo=discord&label=discord%20chat&logoColor=white)](https://discord.gg/Kc4qZWE "Ask a question or just enjoy your stay!")
[![Build Status](https://img.shields.io/circleci/project/github/rustic-games/sm/master.svg?logo=linux&label=linux&logoColor=white)](https://circleci.com/gh/rustic-games/sm/tree/master "Linux builds run on CircleCI. Click to see more details.")
[![Test Coverage Status](https://img.shields.io/codecov/c/github/rustic-games/sm/master.svg?logo=codeship&label=coverage&logoColor=white)](https://codecov.io/gh/rustic-games/sm "Code coverage is provided by Codecov. It's not 100% accurate, but good enough.")

</div>
<br />

Using this library, you declaratively define your state machines as as set
of _states_, connected via _transitions_, triggered by _events_. You can
query the current state of the machine, or pattern match against all
possible machine variants.

The implementation ensures a zero-sized abstraction that uses Rust's
type-system and ownership model to guarantee valid transitions between
states using events, and makes sure previous states are no longer accessible
after transitioning away to another state. Rust validates correct usage of
the state machine at compile-time, no runtime checking occurs when using the
library.

The library exposes the `sm!` macro, which allows you to declaratively build
the state machine.

## Examples

### Quick Example

```rust
extern crate sm;
use sm::sm;

sm! {
    Lock {
        InitialStates { Locked, Unlocked }

        TurnKey {
            Locked => Unlocked
            Unlocked => Locked
        }

        Break {
            Locked, Unlocked => Broken
        }
    }
}

fn main() {
    use Lock::*;
    let lock = Machine::new(Locked);
    let lock = lock.transition(TurnKey);

    assert_eq!(lock.state(), Unlocked);
    assert_eq!(lock.trigger().unwrap(), TurnKey);
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
extern crate sm;
use sm::sm;
```

Next, we initiate the macro declaration:

```rust
sm! {
```

Then, provide a name for the machine, and declare a list of allowed initial
states:

```rust
    Lock {
        InitialStates { Locked, Unlocked }
```

Finally, we declare one or more events and the associated transitions:

```rust
        TurnKey {
            Locked => Unlocked
            Unlocked => Locked
        }

        Break {
            Locked, Unlocked => Broken
        }
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

While you _can_ use `sm.state()` with conditional branching to execute your
code based on the current state, this can be a bit tedious, it's less
idiomatic, and it prevents you from using one extra compile-time validation
tool in our toolbox: using Rust's exhaustive pattern matching requirement to
ensure you've covered all possible state variants in your business logic.

While `sm.state()` returns the state as a unit-like struct (which itself is
a [ZST], or Zero Sized Type), you can use the `sm.as_enum()` method to get
the state machine back as an enum variant.

[zst]: https://doc.rust-lang.org/nomicon/exotic-sizes.html#zero-sized-types-zsts

Using the enum variant and pattern matching, you are able to do the
following:

```rust
use Lock::Variant::*;

match sm.as_enum() {
    InitialLocked(m) => {
        assert_eq!(m.state(), Locked);
        assert!(m.trigger().is_none());
    }
    InitialUnlocked(m) => {
        assert_eq!(m.state(), Unlocked);
        assert!(m.trigger().is_none());
    }
    LockedByTurnKey(m) => {
        assert_eq!(m.state(), Locked);
        assert_eq!(m.trigger().unwrap(), TurnKey);
    }
    UnlockedByTurnKey(m) => {
        assert_eq!(m.state(), Unlocked);
        assert_eq!(m.trigger().unwrap(), TurnKey);
    }
    BrokenByBreak(m) => {
        assert_eq!(m.state(), Broken);
        assert_eq!(m.trigger().unwrap(), Break);
    }
}
```

Each state configured with `InitialStates` has its own variant named
`Initial<State>`. Next to those, each valid state + event combination also
has its own variant, named `<state>By<event>`.

The compiler won't be satisfied until you've either exhausted all possible
enum variants, or you explicitly opt-out of matching all variants, either
way, you can be much more confident that your code won't break if you add a
new state down the road, but forget to add it to a pattern match somewhere
deep inside your code-base.

To transition this machine to the `Unlocked` state, we send the `transition`
method, using the `TurnKey` event:

```rust
let sm = sm.transition(TurnKey);
assert_eq!(sm.state(), Unlocked);
```

Because multiple events can lead to a single state, it's also important to
be able to determine what event caused the machine to transition to the
current state. We can ask this information using the `trigger()` method:

```rust
assert_eq!(sm.trigger().unwrap(), TurnKey);
```

The `trigger()` method returns `None` if no state transition has taken place
yet (ie. the machine is still in its initial state), and `Some(Event)` if
one or more transitions have taken place.

#### A word about Type-Safety and Ownership

It's important to realise that we've _consumed_ the original machine in the
above example when we transitioned the machine to a different state, and got
a newly initialised machine back in the `Unlocked` state.

This allows us to safely use the machine without having to worry about
multiple readers using the machine in different states.

All these checks are applied on compile-time, so the following example would
fail to compile:

```rust
let sm2 = sm.transition(TurnKey);
assert_eq!(sm.state(), Locked);
```

This fails with the following compilation error:

```text
error[E0382]: use of moved value: `sm`
  --> src/lib.rs:315:12
   |
22 | let sm2 = sm.transition(TurnKey);
   |           -- value moved here
23 | assert_eq!(sm.state(), Locked);
   |            ^^ value used here after move
   |
   = note: move occurs because `sm` has type `Lock::Machine<Lock::Locked>`, which does not implement the `Copy` trait
```

Similarly, we cannot execute undefined transitions, these are also caught by
the compiler:

```rust
assert_eq!(sm.state(), Broken);

let sm = sm.transition(TurnKey);
```

This fails with the following compilation error:

```text
error[E0599]: no method named `transition` found for type `Lock::Machine<Lock::Broken>` in the current scope
  --> src/lib.rs:360:13
   |
4  | sm! {
   | --- method `transition` not found for this
...
25 | let sm = sm.transition(TurnKey);
   |             ^^^^^^^^^^
   |
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `transition`, perhaps you need to implement it:
           candidate #1: `sm::Transition`
```

The error message is not great (and can potentially be improved in the
future), but any error telling you `transition` is not implemented, or the
passed in event type is invalid is an indication that you are trying to
execute an illegal state transition.

Finally, we are confined to initialising a new machine in only the states
that we defined in `InitialStates`:

```rust
let sm = Machine::new(Broken);
```

This results in the following error:

```
error[E0277]: the trait bound `Lock::Broken: sm::InitialState` is not satisfied
  --> src/lib.rs:417:10
   |
21 | let sm = Machine::new(Broken);
   |          ^^^^^^^^^^^^ the trait `sm::InitialState` is not implemented for `Lock::Broken`
   |
   = note: required because of the requirements on the impl of `sm::NewMachine<Lock::Broken>` for `Lock::Machine<Lock::Broken>`
```

#### The End 👋

And that's it! There's nothing else to it, except a declarative – and easy
to read – state machine construction macro, and a type-safe and
ownership-focused way of dealing with states and transitions, without any
runtime overhead.

**Go forth and transition!**

## Building

You can install Nix, and use `nix-build` to obtain a build environment and build the project according to the
shipped `Cargo.lock` file.

Otherwise, install Cargo/Rust and use `cargo build` as usual.

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
