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
                assert_eq!(sm.state(), Idle);

                let sm = sm.event(Simulate);
                assert_eq!(sm.state(), Simulating);

                let sm = sm.event(None);
                assert_eq!(sm.state(), Idle);

                let sm = sm.event(Render);
                assert_eq!(sm.state(), Rendering);

                let sm = sm.event(None);
                assert_eq!(sm.state(), Idle);
            })
        },
    );

    c.bench_function("GameLoop – match; Idle => Simulating => Idle => Rendering => Idle", |b| {
        b.iter(|| {
            let mut sm = Machine::new(Idle).as_enum();

            {
                sm = match sm {
                    States::Idle(m) => {
                        let m = m.event(Simulate);
                        assert_eq!(m.state(), Simulating);
                        m.as_enum()
                    },
                    States::Simulating(m) => m.event(None).as_enum(),
                    States::Rendering(m) => m.event(None).as_enum(),
                };

                sm = match sm {
                    States::Idle(m) => m.event(Simulate).as_enum(),
                    States::Simulating(m) => {
                        let m = m.event(None);
                        assert_eq!(m.state(), Idle);
                        m.as_enum()
                    },
                    States::Rendering(m) => m.event(None).as_enum(),
                };

                sm = match sm {
                    States::Idle(m) => {
                        let m = m.event(Render);
                        assert_eq!(m.state(), Rendering);
                        m.as_enum()

                    },
                    States::Simulating(m) => m.event(None).as_enum(),
                    States::Rendering(m) => m.event(None).as_enum(),
                };

                let _ = match sm {
                    States::Idle(m) => m.event(Simulate).as_enum(),
                    States::Simulating(m) => m.event(None).as_enum(),
                    States::Rendering(m) => {
                        let m = m.event(None);
                        assert_eq!(m.state(), Idle);
                        m.as_enum()
                    },
                };
            };
        })
    });

    c.bench_function(
        "GameLoop – loop; Idle => Simulating => Idle => Rendering => Idle",
        |b| {
            b.iter(|| {
                let mut sm = Machine::new(Idle).as_enum();
                let mut i = 0;

                loop {
                    sm = match sm {
                        States::Idle(m) => {
                            if i == 0 {
                                i += 1;
                                let m = m.event(Simulate);
                                assert_eq!(m.state(), Simulating);
                                m.as_enum()
                            } else if i == 1 {
                                i += 1;
                                let m = m.event(Render);
                                assert_eq!(m.state(), Rendering);
                                m.as_enum()
                            } else {
                                break;
                            }
                        },
                        States::Simulating(m) => {
                            let m = m.event(None);
                            assert_eq!(m.state(), Idle);
                            m.as_enum()
                        },
                        States::Rendering(m) => {
                            let m = m.event(None);
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
