use alloc::vec::Vec;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::{braced, Ident};

use sm::event::{Event, Events};
use sm::initial_state::InitialStates;
use sm::state::{State, States};
use sm::transition::Transitions;

#[derive(Debug, PartialEq)]
pub(crate) struct Machines(Vec<Machine>);

impl Parse for Machines {
    /// example machines tokens:
    ///
    /// ```text
    /// TurnStile { ... }
    /// Lock { ... }
    /// MyStateMachine { ... }
    /// ```
    ///
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut machines: Vec<Machine> = Vec::new();

        while !input.is_empty() {
            // `TurnStile { ... }`
            //  ^^^^^^^^^^^^^^^^^
            let machine = Machine::parse(input)?;
            machines.push(machine);
        }

        Ok(Machines(machines))
    }
}

impl ToTokens for Machines {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote! {
            extern crate sm as _sm;
            use _sm::{AsEnum, Initializer, Machine as M, Transition};
        });

        for machine in &self.0 {
            machine.to_tokens(tokens);
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Machine {
    pub name: Ident,
    pub initial_states: InitialStates,
    pub transitions: Transitions,
}

impl Machine {
    fn states(&self) -> States {
        let mut states: Vec<State> = Vec::new();

        for t in &self.transitions.0 {
            if !states.iter().any(|s| s.name == t.from.name) {
                states.push(t.from.clone());
            }

            if !states.iter().any(|s| s.name == t.to.name) {
                states.push(t.to.clone());
            }
        }

        for i in &self.initial_states.0 {
            if !states.iter().any(|s| s.name == i.name) {
                states.push(State {
                    name: i.name.clone(),
                });
            }
        }

        States(states)
    }

    fn events(&self) -> Events {
        let mut events: Vec<Event> = Vec::new();

        for t in &self.transitions.0 {
            if !events.iter().any(|s| s.name == t.event.name) {
                events.push(t.event.clone());
            }
        }

        Events(events)
    }
}

impl Parse for Machine {
    /// example machine tokens:
    ///
    /// ```text
    /// TurnStile {
    ///     InitialStates { ... }
    ///
    ///     Push { ... }
    ///     Coin { ... }
    /// }
    /// ```
    ///
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        // `TurnStile { ... }`
        //  ^^^^^^^^^
        let name: Ident = input.parse()?;

        // `TurnStile { ... }`
        //              ^^^
        let block_machine;
        braced!(block_machine in input);

        // `InitialStates { ... }`
        //  ^^^^^^^^^^^^^^^^^^^^^
        let initial_states = InitialStates::parse(&block_machine)?;

        // `Push { ... }`
        //  ^^^^^^^^^^^^
        let transitions = Transitions::parse(&block_machine)?;

        Ok(Machine {
            name,
            initial_states,
            transitions,
        })
    }
}

impl ToTokens for Machine {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let initial_states = &self.initial_states;
        let states = &self.states();
        let events = &self.events();
        let machine_enum = MachineEnum { states };
        let transitions = &self.transitions;

        tokens.extend(quote! {
            #[allow(non_snake_case)]
            mod #name {
                use _sm::{AsEnum, Event, InitialState, Initializer, Machine as M, NoneEvent, State, Transition};

                #[derive(Debug, Eq, PartialEq)]
                pub struct Machine<S: State, E: Event>(S, Option<E>);

                impl<S: State, E: Event> M for Machine<S, E> {
                    type State = S;
                    type Event = E;

                    fn state(&self) -> Self::State {
                        self.0.clone()
                    }

                    fn trigger(&self) -> Option<Self::Event> {
                        self.1.clone()
                    }
                }

                impl<S: InitialState> Initializer<S> for Machine<S, NoneEvent> {
                    type Machine = Machine<S, NoneEvent>;

                    fn new(state: S) -> Self::Machine {
                        Machine(state, None)
                    }
                }

                #states
                #initial_states
                #events
                #machine_enum
                #transitions
            }
        });
    }
}

#[derive(Debug)]
#[allow(single_use_lifetimes)]
struct MachineEnum<'a> {
    states: &'a States,
}

#[allow(single_use_lifetimes)]
impl<'a> ToTokens for MachineEnum<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let states = &self.states.0;
        let states: &Vec<Ident> = &states.into_iter().map(|s| s.name.clone()).collect();

        // https://git.io/fArHW
        let states2 = states;
        let states3 = states;
        let states4 = states;

        tokens.extend(quote!{
            #[derive(Debug)]
            pub enum States<E: Event> {
                #(#states(Machine<#states2, E>)),*
            }

            #(
                impl<E: Event> AsEnum for Machine<#states3, E> {
                    type Enum = States<E>;

                    fn as_enum(self) -> Self::Enum {
                        States::#states4(self)
                    }
                }
            )*
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{format, vec};
    use proc_macro2::TokenStream;
    use sm::initial_state::InitialState;
    use sm::transition::Transition;
    use syn;
    use syn::parse_quote;

    #[test]
    fn test_machine_parse() {
        let left: Machine = syn::parse2(quote! {
           TurnStile {
               InitialStates { Locked, Unlocked }

               Coin { Locked => Unlocked }
               Push { Unlocked => Locked }
           }
        }).unwrap();

        let right = Machine {
            name: parse_quote! { TurnStile },
            initial_states: InitialStates(vec![
                InitialState {
                    name: parse_quote! { Locked },
                },
                InitialState {
                    name: parse_quote! { Unlocked },
                },
            ]),
            transitions: Transitions(vec![
                Transition {
                    event: Event {
                        name: parse_quote! { Coin },
                    },
                    from: State {
                        name: parse_quote! { Locked },
                    },
                    to: State {
                        name: parse_quote! { Unlocked },
                    },
                },
                Transition {
                    event: Event {
                        name: parse_quote! { Push },
                    },
                    from: State {
                        name: parse_quote! { Unlocked },
                    },
                    to: State {
                        name: parse_quote! { Locked },
                    },
                },
            ]),
        };

        assert_eq!(left, right);
    }

    #[test]
    fn test_machine_to_tokens() {
        let machine = Machine {
            name: parse_quote! { TurnStile },
            initial_states: InitialStates(vec![
                InitialState {
                    name: parse_quote! { Unlocked },
                },
                InitialState {
                    name: parse_quote! { Locked },
                },
            ]),
            transitions: Transitions(vec![Transition {
                event: Event {
                    name: parse_quote! { Push },
                },
                from: State {
                    name: parse_quote! { Unlocked },
                },
                to: State {
                    name: parse_quote! { Locked },
                },
            }]),
        };

        let left = quote! {
            #[allow(non_snake_case)]
            mod TurnStile {
                use _sm::{AsEnum, Event, InitialState, Machine as M, NewMachine, State, Transition};

                #[derive(Debug, Eq, PartialEq)]
                pub struct Machine<S: State>(S);

                impl<S: State> M for Machine<S> {
                    type State = S;

                    fn state(&self) -> S {
                        self.0.clone()
                    }
                }

                impl<S: InitialState> NewMachine<S> for Machine<S> {
                    type Machine = Machine<S>;

                    fn new(state: S) -> Self::Machine {
                        Machine(state)
                    }
                }

                #[derive(Clone, Copy, Debug, Eq)]
                pub struct Unlocked;
                impl State for Unlocked {}

                impl PartialEq<Unlocked> for Unlocked {
                    fn eq(&self, _: & Unlocked) -> bool {
                        true
                    }
                }

                impl PartialEq<Locked> for Unlocked {
                    fn eq(&self, _: & Locked) -> bool {
                        false
                    }
                }

                #[derive(Clone, Copy, Debug, Eq)]
                pub struct Locked;
                impl State for Locked {}

                impl PartialEq<Unlocked> for Locked {
                    fn eq(&self, _: &Unlocked) -> bool {
                        false
                    }
                }

                impl PartialEq<Locked> for Locked {
                    fn eq(&self, _: &Locked) -> bool {
                        true
                    }
                }

                impl InitialState for Unlocked {}
                impl InitialState for Locked {}

                #[derive(Clone, Copy, Debug, Eq)]
                pub struct Push;
                impl Event for Push {}

                impl PartialEq<Push> for Push {
                    fn eq(&self, _: &Push) -> bool {
                        true
                    }
                }

                #[derive(Debug)]
                pub enum States {
                    Unlocked(Machine<Unlocked>),
                    Locked(Machine<Locked>)
                }

                impl AsEnum for Machine<Unlocked> {
                    type Enum = States;

                    fn as_enum(self) -> Self::Enum {
                        States::Unlocked(self)
                    }
                }

                impl AsEnum for Machine<Locked> {
                    type Enum = States;

                    fn as_enum(self) -> Self::Enum {
                        States::Locked(self)
                    }
                }

                impl Transition<Push> for Machine<Unlocked> {
                    type Machine = Machine<Locked>;

                    fn transition(self, _: Push) -> Self::Machine {
                        Machine(Locked)
                    }
                }
            }
        };

        let mut right = TokenStream::new();
        machine.to_tokens(&mut right);

        assert_eq!(format!("{}", left), format!("{}", right))
    }

    #[test]
    fn test_machines_parse() {
        let left: Machines = syn::parse2(quote! {
           TurnStile {
               InitialStates { Locked, Unlocked }

               Coin { Locked => Unlocked }
               Push { Unlocked => Locked }
           }

           Lock {
               InitialStates { Locked, Unlocked }

               TurnKey {
                   Locked => Unlocked
                   Unlocked => Locked
                }
           }
        }).unwrap();

        let right = Machines(vec![
            Machine {
                name: parse_quote! { TurnStile },
                initial_states: InitialStates(vec![
                    InitialState {
                        name: parse_quote! { Locked },
                    },
                    InitialState {
                        name: parse_quote! { Unlocked },
                    },
                ]),
                transitions: Transitions(vec![
                    Transition {
                        event: Event {
                            name: parse_quote! { Coin },
                        },
                        from: State {
                            name: parse_quote! { Locked },
                        },
                        to: State {
                            name: parse_quote! { Unlocked },
                        },
                    },
                    Transition {
                        event: Event {
                            name: parse_quote! { Push },
                        },
                        from: State {
                            name: parse_quote! { Unlocked },
                        },
                        to: State {
                            name: parse_quote! { Locked },
                        },
                    },
                ]),
            },
            Machine {
                name: parse_quote! { Lock },
                initial_states: InitialStates(vec![
                    InitialState {
                        name: parse_quote! { Locked },
                    },
                    InitialState {
                        name: parse_quote! { Unlocked },
                    },
                ]),
                transitions: Transitions(vec![
                    Transition {
                        event: Event {
                            name: parse_quote! { TurnKey },
                        },
                        from: State {
                            name: parse_quote! { Locked },
                        },
                        to: State {
                            name: parse_quote! { Unlocked },
                        },
                    },
                    Transition {
                        event: Event {
                            name: parse_quote! { TurnKey },
                        },
                        from: State {
                            name: parse_quote! { Unlocked },
                        },
                        to: State {
                            name: parse_quote! { Locked },
                        },
                    },
                ]),
            },
        ]);

        assert_eq!(left, right);
    }

    #[test]
    fn test_machines_to_tokens() {
        let machines = Machines(vec![
            Machine {
                name: parse_quote! { TurnStile },
                initial_states: InitialStates(vec![
                    InitialState {
                        name: parse_quote! { Locked },
                    },
                    InitialState {
                        name: parse_quote! { Unlocked },
                    },
                ]),
                transitions: Transitions(vec![
                    Transition {
                        event: Event {
                            name: parse_quote! { Coin },
                        },
                        from: State {
                            name: parse_quote! { Locked },
                        },
                        to: State {
                            name: parse_quote! { Unlocked },
                        },
                    },
                    Transition {
                        event: Event {
                            name: parse_quote! { Push },
                        },
                        from: State {
                            name: parse_quote! { Unlocked },
                        },
                        to: State {
                            name: parse_quote! { Locked },
                        },
                    },
                ]),
            },
            Machine {
                name: parse_quote! { Lock },
                initial_states: InitialStates(vec![
                    InitialState {
                        name: parse_quote! { Locked },
                    },
                    InitialState {
                        name: parse_quote! { Unlocked },
                    },
                ]),
                transitions: Transitions(vec![
                    Transition {
                        event: Event {
                            name: parse_quote! { TurnKey },
                        },
                        from: State {
                            name: parse_quote! { Locked },
                        },
                        to: State {
                            name: parse_quote! { Unlocked },
                        },
                    },
                    Transition {
                        event: Event {
                            name: parse_quote! { TurnKey },
                        },
                        from: State {
                            name: parse_quote! { Unlocked },
                        },
                        to: State {
                            name: parse_quote! { Locked },
                        },
                    },
                ]),
            },
        ]);

        let left = quote! {
            extern crate sm as _sm;
            use _sm::{AsEnum, Machine as M, NewMachine, Transition};

            #[allow(non_snake_case)]
            mod TurnStile {
                use _sm::{AsEnum, Event, InitialState, Machine as M, NewMachine, State, Transition};

                #[derive(Debug, Eq, PartialEq)]
                pub struct Machine<S: State>(S);

                impl<S: State> M for Machine<S> {
                    type State = S;

                    fn state(&self) -> S {
                        self.0.clone()
                    }
                }

                impl<S: InitialState> NewMachine<S> for Machine<S> {
                    type Machine = Machine<S>;

                    fn new(state: S) -> Self::Machine {
                        Machine(state)
                    }
                }

                #[derive(Clone, Copy, Debug, Eq)]
                pub struct Locked;
                impl State for Locked {}

                impl PartialEq<Locked> for Locked {
                    fn eq(&self, _: &Locked) -> bool {
                        true
                    }
                }

                impl PartialEq<Unlocked> for Locked {
                    fn eq(&self, _: &Unlocked) -> bool {
                        false
                    }
                }

                #[derive(Clone, Copy, Debug, Eq)]
                pub struct Unlocked;
                impl State for Unlocked {}

                impl PartialEq<Locked> for Unlocked {
                    fn eq(&self, _: & Locked) -> bool {
                        false
                    }
                }

                impl PartialEq<Unlocked> for Unlocked {
                    fn eq(&self, _: & Unlocked) -> bool {
                        true
                    }
                }

                impl InitialState for Locked {}
                impl InitialState for Unlocked {}

                #[derive(Clone, Copy, Debug, Eq)]
                pub struct Coin;
                impl Event for Coin {}

                impl PartialEq<Coin> for Coin {
                    fn eq(&self, _: &Coin) -> bool {
                        true
                    }
                }

                impl PartialEq<Push> for Coin {
                    fn eq(&self, _: &Push) -> bool {
                        false
                    }
                }

                #[derive(Clone, Copy, Debug, Eq)]
                pub struct Push;
                impl Event for Push {}

                impl PartialEq<Coin> for Push {
                    fn eq(&self, _: &Coin) -> bool {
                        false
                    }
                }

                impl PartialEq<Push> for Push {
                    fn eq(&self, _: &Push) -> bool {
                        true
                    }
                }

                #[derive(Debug)]
                pub enum States {
                    Locked(Machine<Locked>),
                    Unlocked(Machine<Unlocked>)
                }

                impl AsEnum for Machine<Locked> {
                    type Enum = States;

                    fn as_enum(self) -> Self::Enum {
                        States::Locked(self)
                    }
                }

                impl AsEnum for Machine<Unlocked> {
                    type Enum = States;

                    fn as_enum(self) -> Self::Enum {
                        States::Unlocked(self)
                    }
                }

                impl Transition<Coin> for Machine<Locked> {
                    type Machine = Machine<Unlocked>;

                    fn transition(self, _: Coin) -> Self::Machine {
                        Machine(Unlocked)
                    }
                }

                impl Transition<Push> for Machine<Unlocked> {
                    type Machine = Machine<Locked>;

                    fn transition(self, _: Push) -> Self::Machine {
                        Machine(Locked)
                    }
                }
            }

            #[allow(non_snake_case)]
            mod Lock {
                use _sm::{AsEnum, Event, InitialState, Machine as M, NewMachine, State, Transition};

                #[derive(Debug, Eq, PartialEq)]
                pub struct Machine<S: State>(S);

                impl<S: State> M for Machine<S> {
                    type State = S;

                    fn state(&self) -> S {
                        self.0.clone()
                    }
                }

                impl<S: InitialState> NewMachine<S> for Machine<S> {
                    type Machine = Machine<S>;

                    fn new(state: S) -> Self::Machine {
                        Machine(state)
                    }
                }

                #[derive(Clone, Copy, Debug, Eq)]
                pub struct Locked;
                impl State for Locked {}

                impl PartialEq<Locked> for Locked {
                    fn eq(&self, _: &Locked) -> bool {
                        true
                    }
                }

                impl PartialEq<Unlocked> for Locked {
                    fn eq(&self, _: &Unlocked) -> bool {
                        false
                    }
                }

                #[derive(Clone, Copy, Debug, Eq)]
                pub struct Unlocked;
                impl State for Unlocked {}

                impl PartialEq<Locked> for Unlocked {
                    fn eq(&self, _: & Locked) -> bool {
                        false
                    }
                }

                impl PartialEq<Unlocked> for Unlocked {
                    fn eq(&self, _: & Unlocked) -> bool {
                        true
                    }
                }

                impl InitialState for Locked {}
                impl InitialState for Unlocked {}

                #[derive(Clone, Copy, Debug, Eq)]
                pub struct TurnKey;
                impl Event for TurnKey {}

                impl PartialEq<TurnKey> for TurnKey {
                    fn eq(&self, _: &TurnKey) -> bool {
                        true
                    }
                }

                #[derive(Debug)]
                pub enum States {
                    Locked(Machine<Locked>),
                    Unlocked(Machine<Unlocked>)
                }

                impl AsEnum for Machine<Locked> {
                    type Enum = States;

                    fn as_enum(self) -> Self::Enum {
                        States::Locked(self)
                    }
                }

                impl AsEnum for Machine<Unlocked> {
                    type Enum = States;

                    fn as_enum(self) -> Self::Enum {
                        States::Unlocked(self)
                    }
                }

                impl Transition<TurnKey> for Machine<Locked> {
                    type Machine = Machine<Unlocked>;

                    fn transition(self, _: TurnKey) -> Self::Machine {
                        Machine(Unlocked)
                    }
                }

                impl Transition<TurnKey> for Machine<Unlocked> {
                    type Machine = Machine<Locked>;

                    fn transition(self, _: TurnKey) -> Self::Machine {
                        Machine(Locked)
                    }
                }
            }
        };

        let mut right = TokenStream::new();
        machines.to_tokens(&mut right);

        assert_eq!(format!("{}", left), format!("{}", right))
    }
}
