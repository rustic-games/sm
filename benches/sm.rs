#[macro_use]
extern crate criterion;
#[macro_use]
extern crate sm;

use criterion::Criterion;
use sm::*;

sm!{
    GameLoop { Idle, Simulating, Rendering }

    None {
        Simulating => Idle
        Rendering => Idle
    }

    Simulate {
        Idle => Simulating
    }

    Render {
        Idle => Rendering
    }
}

use self::GameLoop::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function(
        "GameLoop; Idle => Simulating => Idle => Rendering => Idle",
        |b| {
            b.iter(|| {
                let sm = Machine::new(Idle);
                let _ = sm.state();

                let sm = sm.event(Simulate);
                let _ = sm.state();

                let sm = sm.event(None);
                let _ = sm.state();

                let sm = sm.event(Render);
                let _ = sm.state();

                let sm = sm.event(None);
                let _ = sm.state();
            })
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
