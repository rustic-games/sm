<img src="https://upload.wikimedia.org/wikipedia/commons/8/85/Fxemoji_u1F680.svg" width="70" align="left">

_SM serves as one of the building blocks for [an open-source game about space
engineering and exploration][rkt]. **As long as the game is in development, SM
will be maintained.**_

[rkt]: https://rustic.games

<br />

<img src="./logo.svg" align="right" title="SM logo by Jean Mertz" width="400" />

SM aims to be a **safe**, **fast** and **simple** state machine library.

* **safe** — Rust's type system, ownership model and exhaustive pattern matching
  prevent you from mis-using your state machines

* **fast** — zero runtime overhead, the machine is 100% static, all validation
  happens at compile-time

* **simple** — five traits, and one optional declarative macro, control-flow
  only, no business logic attached

---

<div align="right">

[![Latest Crate Version](https://img.shields.io/crates/v/sm.svg?logo=rust&label=version&logoColor=white&colorB=brightgreen)](https://crates.io/crates/sm "The latest released version on crates.io.")
[![Discord Chat](https://img.shields.io/discord/477552212156088320.svg?logo=discord&label=discord%20chat&logoColor=white)](https://discord.gg/Kc4qZWE "Ask a question or just enjoy your stay!")
[![Linux Build Status](https://img.shields.io/circleci/project/github/rusty-rockets/sm/master.svg?logo=linux&label=linux&logoColor=white)](https://circleci.com/gh/rusty-rockets/sm/tree/master "Linux builds run on CircleCI. Click to see more details.")
[![Windows Build Status](https://img.shields.io/appveyor/ci/rusty-rockets/sm/master.svg?logo=windows&label=windows&logoColor=white)](https://ci.appveyor.com/project/rusty-rockets/sm/branch/master "Windows builds run on AppVeyor. Click to see more details.")
[![Test Coverage Status](https://img.shields.io/codecov/c/github/rusty-rockets/sm/master.svg?logo=codeship&label=coverage&logoColor=white)](https://codecov.io/gh/rusty-rockets/sm "Code coverage is provided by Codecov. It's not 100% accurate, but good enough.")

</div>
<br />

{{readme}}

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
