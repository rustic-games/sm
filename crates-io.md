SM aims to be a **safe**, **fast** and **simple** state machine library.

- **safe** — Rust's type system, ownership model and exhaustive pattern matching
  prevent you from mis-using your state machines

- **fast** — zero runtime overhead, the machine is 100% static, all validation
  happens at compile-time

- **simple** — five traits, and one optional declarative macro, control-flow
  only, no business logic attached

---

You might be looking for:

- [An overview of SM][book]
- [Our GitHub repository][repo]
- [Examples][examples]
- [API documentation][api]

[book]: https://github.com/rusty-rockets/sm/blob/master/README.md#descriptive-example
[repo]: https://github.com/rusty-rockets/sm
[examples]: https://github.com/rusty-rockets/sm/tree/master/examples
[api]: https://docs.rs/sm

## Quick Example

```rust
extern crate sm;
use sm::sm;

sm! {
    Lock {
        States { Locked, Unlocked, Broken }

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
}
```
