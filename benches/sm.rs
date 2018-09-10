#[macro_use]
extern crate criterion;
#[macro_use]
extern crate sm;

use criterion::Criterion;

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
        "base - Idle . Simulating . Idle . Rendering . Idle",
        |b| {
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
        },
    );

    c.bench_function("match - Idle . Simulating . Idle . Rendering . Idle", |b| {
        b.iter(|| {
            let mut sm = Machine::new(Idle).as_enum();

            {
                sm = match sm {
                    States::Idle(m) => {
                        let m = m.transition(Simulate);
                        assert_eq!(m.state(), Simulating);
                        m.as_enum()
                    },
                    States::Simulating(m) => m.transition(None).as_enum(),
                    States::Rendering(m) => m.transition(None).as_enum(),
                };

                sm = match sm {
                    States::Idle(m) => m.transition(Simulate).as_enum(),
                    States::Simulating(m) => {
                        let m = m.transition(None);
                        assert_eq!(m.state(), Idle);
                        m.as_enum()
                    },
                    States::Rendering(m) => m.transition(None).as_enum(),
                };

                sm = match sm {
                    States::Idle(m) => {
                        let m = m.transition(Render);
                        assert_eq!(m.state(), Rendering);
                        m.as_enum()

                    },
                    States::Simulating(m) => m.transition(None).as_enum(),
                    States::Rendering(m) => m.transition(None).as_enum(),
                };

                let _ = match sm {
                    States::Idle(m) => m.transition(Simulate).as_enum(),
                    States::Simulating(m) => m.transition(None).as_enum(),
                    States::Rendering(m) => {
                        let m = m.transition(None);
                        assert_eq!(m.state(), Idle);
                        m.as_enum()
                    },
                };
            };
        })
    });

    c.bench_function(
        "loop - Idle . Simulating . Idle . Rendering . Idle",
        |b| {
            b.iter(|| {
                let mut sm = Machine::new(Idle).as_enum();
                let mut i = 0;

                loop {
                    sm = match sm {
                        States::Idle(m) => {
                            if i == 0 {
                                i += 1;
                                let m = m.transition(Simulate);
                                assert_eq!(m.state(), Simulating);
                                m.as_enum()
                            } else if i == 1 {
                                i += 1;
                                let m = m.transition(Render);
                                assert_eq!(m.state(), Rendering);
                                m.as_enum()
                            } else {
                                break;
                            }
                        },
                        States::Simulating(m) => {
                            let m = m.transition(None);
                            assert_eq!(m.state(), Idle);
                            m.as_enum()
                        },
                        States::Rendering(m) => {
                            let m = m.transition(None);
                            assert_eq!(m.state(), Idle);
                            m.as_enum()
                        },
                    }
                }
            })
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
