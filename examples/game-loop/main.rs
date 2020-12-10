use rand::{
    distributions::{Distribution, Uniform},
    prelude::*,
};
use sm::sm;
use std::{
    ops::Add,
    time::{Duration, Instant},
};

sm! {
    // `GameTick` is the name of our state machine. The machine handles a single
    // game tick (or a single run of the game loop) from start to finish.
    GameTick {
        // `InitialStates` defines the states in which the machine can be
        // initialised. Any state not defined here cannot be used as the first
        // state when creating a new instance of the GameTick machine.
        //
        // In this case, we only allow the machine to be initialised in the
        // `Idle` state, and let the game loop logic transition the machine from
        // that starting point, based on the below defined valid transitions.
        InitialStates { Idle }

        // The first defined event is called `DrainAccumulatedTime`. When this
        // event is triggered, the game loop is instructed to update the game
        // state for as many times as the "accumulated time" allows for. See the
        // implementation below for more details.
        //
        // This event can be triggered only when the machine is either in the
        // `Idle` or `Updating` state.
        DrainAccumulatedTime {
            Idle, Updating => Updating
        }

        // `Render` is triggered from the `Updating` state, after all
        // accumulated time has been drained for a single game tick.
        Render {
            Updating => Rendering
        }

        // After rendering, the `CompletedRendering` event is triggered, which
        // signals the state machine that this single game tick is completed.
        CompletedRendering {
            Rendering => Finished
        }
    }
}

// By exporting the relevant module and enum variants, we can be a bit less
// verbose in the rest of our code. If we had defined multiple state machines,
// we would not be able to do this in the global scope, as there would be
// overlap in naming.
use crate::GameTick::{Variant::*, *};

// We'll define the `Nanoseconds` alias to make it easier to reason about
// numbers representing a timestamp.
type Nanoseconds = u64;

// This is a convenience constant, to make the rest of the code a bit easier to
// parse.
#[allow(non_upper_case_globals)]
const nanoseconds_per_second: u64 = 1_000_000_000;

/// Config contains the variables that you could potentially expose to the
/// player via a preferences menu.
struct Config {
    /// `fixed_updates_per_second` sets the game state update to a fixed
    /// interval. This is what decouples your game update behaviour from the
    /// speed at which the game is rendered to the screen (FPS).
    ///
    /// For more details, see:
    /// * https://www.koonsolo.com/news/dewitters-gameloop/
    /// * https://gafferongames.com/post/fix_your_timestep/
    fixed_updates_per_second: u64,

    /// `max_frames_per_second` is an optional configuration value that allows
    /// you to cap the frames per second being rendered to the screen. This is
    /// useful if you don't need 200 FPS, but are perfectly fine with 100 FPS
    /// and not waste extra GPU/CPU cycles on the 100 extra renders.
    ///
    /// If set to `None`, no frame limiting is applied.
    ///
    /// TODO: unimplemented!
    #[allow(dead_code)]
    max_frames_per_second: Option<u64>,
}

// `TestTunables` are solely for the purpose of tweaking the variables of this
// example, they are not part of any real game loop, but simply represent the
// variable state a real production loop would include.
struct TestTunables {
    // Render min and max duration set the range of _milliseconds_ the render
    // handler will sleep before returning. This simulates actual visual
    // rendering of a real game engine. If both are equal, an exact value will
    // be used.
    render_min_duration: u64,
    render_max_duration: u64,

    // Similar to render min/max, but for the update handler.
    update_min_duration: u64,
    update_max_duration: u64,

    // Total number of "ticks" the engine should run. Each tick equals a single
    // game loop cycle, which equals seeing a single GameTick state machine
    // instance going from start to finish.
    min_ticks: u32,
    max_ticks: u32,

    // A random number generator, used to generate numbers between the above
    // min/max configurations.
    rng: rand::rngs::ThreadRng,
}

fn main() {
    let config = Config {
        fixed_updates_per_second: 100,
        max_frames_per_second: None,
    };

    let mut tunables = TestTunables {
        render_min_duration: 1,
        render_max_duration: 1,

        update_min_duration: 1,
        update_max_duration: 1,

        min_ticks: 1000,
        max_ticks: 1000,

        rng: thread_rng(),
    };

    // We'll add some data points to keep track of and print at the end of the
    // example.
    let mut idle_count: u32 = 0;
    let mut update_count: u32 = 0;
    let mut render_count: u32 = 0;

    // `update_interval` is the minimum amount of time (in nanoseconds) that
    // needs to pass before we trigger a game state update. This is a fixed
    // delta, to give us a predictable game simulation, and decouple our
    // simulation from the capabilities of the host in terms of rendering
    // performance.
    //
    // Think of it like this: after every frame render, we've given ourselves
    // some time to perform game state updates. We'll perform those updates at
    // the interval defined here, and we'll continue those updates for as long
    // as we don't have to render the next frame.
    let update_interval: Nanoseconds = nanoseconds_per_second / config.fixed_updates_per_second;

    // // ...
    // let _render_max_interval: Nanoseconds = if config.max_frames_per_second == 0
    // {     0
    // } else {
    //     nanoseconds_per_second / config.max_frames_per_second
    // };

    // `total_time` is the total accumulation of passed time (in nanoseconds).
    // This is a monotonically increasing value. The value is passed to the
    // update handler of the game, which can use this when needed.
    //
    // This uses `u64` as the storage type, using nanoseconds as the unit of
    // measurement, a single game session can run more than 500 years before we
    // get an integer overflow.
    #[allow(unused_variables)]
    let mut total_time: Nanoseconds = 0;

    // `last_step_timestamp` is the timestamp at the end of the last game step,
    // represented as an `Instant`. This value is updated after each game step,
    // allowing us to determine how long the last step took, and how much time
    // we have to run our update handler.
    let mut last_step_timestamp: Instant = Instant::now();

    // `accumulated_time` is the total time available (in nanoseconds) for the
    // update handler to run. It is based off of the `current_time` value. After
    // each update step, we subtract the `delta_time` from the remaining
    // `accumulated_time`.
    //
    // When the accumulated time falls below the delta time, we render another
    // frame to the screen, and send the remaining accumulated time to the
    // render handler. We do this, so that the renderer can figure out how much
    // time there was left between the last game update and the next.
    //
    // Say for example that we moved to position X = 10 on the last update, and
    // the following is true:
    //
    // * we move 1X per update
    // * we update the game state 100 times per second (so we need 10 milliseconds
    //   per update)
    // * our `accumulated_time` has 5 milliseconds remaining (remember, we _need_ 10
    //   milliseconds to update the game, so the last 5 milliseconds are kept
    //   around)
    //
    // we now know that if we had 10 milliseconds remaining, the character
    // would've moved to X = 11. But since we only had 5 milliseconds left, the
    // character position wasn't updated in the last cycle. However, as soon as
    // we add 5 more milliseconds to our accumulator in the next cycle, it will
    // move to that X = 11 position.
    //
    // So, instead of rendering our character as "stopped" on X = 10, we'll
    // instead interpolate that we were at X = 10 in the last update, we would
    // have moved to X = 11 if we had 10 more milliseconds, but we only have 5
    // milliseconds left, which is 50% of a full movement, so we'll render the
    // character at X = 10 + 0.5 = 10.5.
    //
    // If, during the next update cycle, the character is moved to X = 11, we
    // can render the character there, and we've had three frames, the first one
    // rendering the character at position 10, the second frame at 10.5, and the
    // third at 11.
    //
    // If, however, it turns out the player instead instructed the character to
    // stop after the first frame (when the game still had the character
    // positioned at X = 10), we'll have to move the character back on the
    // screen. This will cause a (mostly unnoticeable) "stutter", but the fact
    // is that most of the time, the character would have ended up at X = 11,
    // making it a worthy trade off to have a once-every-while bad
    // interpolation, instead of constantly stuttering images due to not
    // interpolating the remaining accumulated update time every cycle.
    let mut accumulated_time: Nanoseconds = 0;

    // In a real implementation, we would loop until the player initiates a game
    // shutdown. In this example, we loop for a fixed amount of cycles, and then
    // terminate the example.
    let mut i = 0;
    let max_count: u32;

    if tunables.min_ticks == tunables.max_ticks {
        max_count = tunables.min_ticks;
    } else {
        max_count = Uniform::from(tunables.min_ticks..tunables.max_ticks).sample(&mut tunables.rng);
    }

    let mut total_duration = Duration::from_secs(0);
    let mut game_is_running = true;

    // This while statement represents the main game loop. Every loop inside
    // this block is called a "game tick", and it represents the advancement of
    // the game state by one single step. The continuous execution of those
    // steps is what makes the game _tick_ (no pun intended).
    while game_is_running {
        // We create a new state machine, starting in the `Idle` state. We then
        // take the enum variant of that state machine, and assign it to a
        // mutable variable. We'll use this variable to assign a new machine
        // variant in the loop below, to allow the next step of the game loop to
        // go into the right matching arm.
        let mut sm = Machine::new(Idle).as_enum();

        // This inner loop moves the state machine forward from one state to the
        // next, until the end of the machine is reached. For each game tick,
        // the machine transitions from Idle, to Updating, Rendering and finally
        // to Finished.
        loop {
            // we pattern match against the state machine variant, and execute
            // the right arm logic, based on the current state of the machine.
            // Each arm of the match returns a state machine variant, which we
            // assign to the `sm` variable, to be used in the next loop cycle.
            sm = match sm {
                // `InitialIdle` is the starting state of the game tick.
                //
                // It is only triggered once in every game tick, since the state
                // machine can only _once_ be in its "initial" mode.
                InitialIdle(m) => {
                    handle_idle(&mut idle_count, &mut i);

                    let last_step_duration = last_step_timestamp.elapsed();
                    accumulated_time += last_step_duration.as_nanos() as u64;
                    total_duration = total_duration.add(last_step_duration);

                    last_step_timestamp = Instant::now();

                    m.transition(DrainAccumulatedTime).as_enum()
                }

                // Every time we end up in this match arm, we are asked to
                // update the game state (for example, updating the physics).
                UpdatingByDrainAccumulatedTime(m) => {
                    // We check if there's enough time accumulated to actually
                    // update a single game update. The required available time
                    // depends on the configured updates per second.
                    if accumulated_time >= update_interval {
                        handle_update(&mut update_count, &mut tunables);

                        accumulated_time -= update_interval;
                        total_time += update_interval;

                        // After triggering a single game state update, we make
                        // sure the state machine transitions back into this
                        // state again, so that we can try to update the game
                        // another time, depending on if there is enough time
                        // left to do so.
                        m.transition(DrainAccumulatedTime).as_enum()

                    // If not enough time remains to perform a game update, the
                    // state machine is advanced to the next state, which causes
                    // the game state to be rendered to the screen.
                    } else {
                        m.transition(Render).as_enum()
                    }
                }

                // The rendering match arm is triggered _after_ the game has
                // been updated. In this state, the current game state is
                // rendered to the screen.
                RenderingByRender(m) => {
                    handle_render(&mut render_count, &mut tunables);

                    m.transition(CompletedRendering).as_enum()
                }

                // Finally, after the game state has been rendered, the
                // execution of the current game tick is done. When this match
                // arm is done executing, the game will transition to the start
                // of the next tick.
                FinishedByCompletedRendering(_) => break,
            }
        }

        if i >= max_count {
            game_is_running = false
        }

        // This is the end of a single "game tick". At this point, our state
        // machine has made sure we've updated the game state, and rendered the
        // new sate to the screen. This whole process will start all over from
        // the beginning at the next game tick.
    }

    // Print some debug statistics to the terminal, to give a visual
    // representation of what happened from game start to finish.
    statistics(idle_count, update_count, render_count, total_duration);
}

fn handle_idle(count: &mut u32, i: &mut u32) {
    // This is just here to make sure our game loop ends!
    *i += 1;

    *count += 1;
}

fn handle_update(count: &mut u32, tunables: &mut TestTunables) {
    *count += 1;

    let n: u64;
    if tunables.update_min_duration == tunables.update_max_duration {
        n = tunables.update_min_duration;
    } else {
        n = Uniform::from(tunables.update_min_duration..tunables.update_max_duration)
            .sample(&mut tunables.rng);
    }

    // Simulate game update duration...
    std::thread::sleep(Duration::from_millis(n));
}

fn handle_render(count: &mut u32, tunables: &mut TestTunables) {
    *count += 1;

    let n: u64;
    if tunables.render_min_duration == tunables.render_max_duration {
        n = tunables.render_min_duration;
    } else {
        n = Uniform::from(tunables.render_min_duration..tunables.render_max_duration)
            .sample(&mut tunables.rng);
    }

    // Simulate frame rendering duration...
    std::thread::sleep(Duration::from_millis(n));
}

fn statistics(idle: u32, update: u32, render: u32, elapsed: Duration) {
    println!(
        "{duration_secs:>3.*}.{duration_millis:03.*}s: [ups: {ups:>3}, updates: {updates:>3}] \
         [fps: {fps:>3}, renders: {renders:>3}] [loops:{idle:>4}]",
        3,
        3,
        duration_secs = elapsed.as_secs(),
        duration_millis = elapsed.subsec_millis(),
        ups = (update as u64) * nanoseconds_per_second / elapsed.as_nanos() as u64,
        updates = update,
        fps = (render as u64) * nanoseconds_per_second / elapsed.as_nanos() as u64,
        renders = render,
        idle = idle
    );
}
