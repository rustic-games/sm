# Examples

## TurnStile (manual)

See [`turnstile-manual`](./turnstile-manual).

This example contains a [turnstile](https://en.wikipedia.org/wiki/Turnstile)
state machine implementation, using traits only, without the provided `sm`
macro.

## TurnStile (macro)

See [`turnstile-macro`](./turnstile-macro).

This example contains a similar turnstile implementation, using the exact same
runtime logic, but using the `sm` macro to construct the state machine
declaratively.

## Game Loop

See [`game-loop`](./game-loop).

This is an example on how to use the state machine inside a game loop. The
implemented game loop has no real logic, other than the `updating` and
`rendering` states, but it gets the idea acros on how the library can be used
for such use cases.
