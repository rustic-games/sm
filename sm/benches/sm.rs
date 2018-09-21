#[macro_use]
extern crate criterion;
extern crate sm;

use criterion::Criterion;
use sm::sm;

sm!{
    GameLoop {
        InitialStates { Idle, Simulating, Rendering }

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
}

use self::GameLoop::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("base - Idle . Simulating . Idle . Rendering . Idle", |b| {
        b.iter(|| {
            let sm = Machine::new(Idle);
            assert_eq!(sm.state(), Idle);

            let sm = sm.transition(Simulate);
            assert_eq!(sm.state(), Simulating);

            let sm = sm.transition(None);
            assert_eq!(sm.state(), Idle);

            let sm = sm.transition(Render);
            assert_eq!(sm.state(), Rendering);

            let sm = sm.transition(None);
            assert_eq!(sm.state(), Idle);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
