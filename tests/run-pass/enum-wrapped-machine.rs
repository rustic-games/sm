#[macro_use]
extern crate sm;

sm!{
    Lock { Locked, Unlocked }

    TurnKey { Locked => Unlocked }
}

fn main() {
    use Lock::*;

    let mut sm = Machine::new(Locked).as_enum();

    loop {
        sm = match sm {
            States::Locked(m) => m.transition(TurnKey).as_enum(),
            States::Unlocked(_) => {
                break;
            }
        }
    }
}
