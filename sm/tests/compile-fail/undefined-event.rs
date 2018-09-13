#[macro_use]
extern crate sm;

sm!{
    Lock {
        States { Locked }
    }
}

fn main() {
    use Lock::*;
    let sm = Machine::new(Locked);

    sm.transition(Invalid);
    //~^ ERROR no method named `transition` found for type `Lock::Machine<Lock::Locked>` in the current scope
    //~^^ ERROR cannot find value `Invalid` in this scope
}
