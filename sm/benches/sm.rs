#[macro_use]
extern crate criterion;
extern crate sm;

use criterion::Criterion;
use sm::sm;

sm!{
    GameLoop {
        InitialStates { Idle }

        None {
            Simulating, Rendering => Idle
        }

        Simulate {
            Idle => Simulating
        }

        Render {
            Idle => Rendering
        }
    }
}

use self::GameLoop::Variant::*;
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

    c.bench_function("match - Idle . Simulating . Idle . Rendering . Idle", |b| {
        b.iter(|| {
            let mut sm = Machine::new(Idle).as_enum();

            {
                sm = match sm {
                    InitialIdle(m) => {
                        let m = m.transition(Simulate);
                        assert_eq!(m.state(), Simulating);
                        m.as_enum()
                    }
                    SimulatingBySimulate(_) => unreachable!(),
                    IdleByNone(_) => unreachable!(),
                    RenderingByRender(_) => unreachable!(),
                };

                sm = match sm {
                    InitialIdle(_) => unreachable!(),
                    SimulatingBySimulate(m) => {
                        let m = m.transition(None);
                        assert_eq!(m.state(), Idle);
                        m.as_enum()
                    }
                    IdleByNone(_) => unreachable!(),
                    RenderingByRender(_) => unreachable!(),
                };

                sm = match sm {
                    InitialIdle(_) => unreachable!(),
                    SimulatingBySimulate(_) => unreachable!(),
                    IdleByNone(m) => {
                        let m = m.transition(Render);
                        assert_eq!(m.state(), Rendering);
                        m.as_enum()
                    }
                    RenderingByRender(_) => unreachable!(),
                };

                let _ = match sm {
                    InitialIdle(_) => unreachable!(),
                    SimulatingBySimulate(_) => unreachable!(),
                    IdleByNone(_) => unreachable!(),
                    RenderingByRender(m) => {
                        let m = m.transition(None);
                        assert_eq!(m.state(), Idle);
                        m.as_enum()
                    }
                };
            };
        })
    });

    c.bench_function("loop - Idle . Simulating . Idle . Rendering . Idle", |b| {
        b.iter(|| {
            let mut sm = Machine::new(Idle).as_enum();

            loop {
                sm = match sm {
                    InitialIdle(m) => {
                        let m = m.transition(Simulate);
                        assert_eq!(m.state(), Simulating);
                        m.as_enum()
                    }
                    SimulatingBySimulate(m) => {
                        let m = m.transition(None);
                        assert_eq!(m.state(), Idle);
                        m.as_enum()
                    }
                    IdleByNone(m) => {
                        let m = m.transition(Render);
                        assert_eq!(m.state(), Rendering);
                        m.as_enum()
                    }
                    RenderingByRender(m) => {
                        let m = m.transition(None);
                        assert_eq!(m.state(), Idle);
                        break;
                    }
                }
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
