# Game Loop

This example shows a (non-functioning) game loop implemented using a declarative
state machine definition.

The goal of the state machine in this example is to:

* create a clear division between different states of the game loop
* show how to use the state machine in a loop construct

The state machine is defined like this:

```rust
GameTick {
    InitialStates { Idle }

    DrainAccumulatedTime {
        Idle, Updating => Updating
    }

    Render {
        Updating => Rendering
    }

    CompletedRendering {
        Rendering => Finished
    }
}
```

## How it works

This is an example on how to use the state machine inside a game loop. The
implemented game loop has no real logic, other than the `updating` and
`rendering` states, but it gets the idea acros on how the library can be used
for such use cases.

You can run the program using `cargo run --example game-loop`.

The program executes 1000 _game ticks_ by default, and prints the
_updates per second_ and _frames per second_ results at the end of execution.

For more details on how the implementation works, see the inline code comments.
