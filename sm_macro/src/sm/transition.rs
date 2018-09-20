use alloc::vec::Vec;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::token::Comma;
use syn::{braced, Token};

use sm::event::Event;
use sm::state::State;

#[derive(Debug, PartialEq)]
pub(crate) struct Transitions(pub Vec<Transition>);

impl Parse for Transitions {
    /// example transitions tokens:
    ///
    /// ```text
    /// Push { ... }
    /// Coin { ... }
    /// ```
    ///
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut transitions: Vec<Transition> = Vec::new();
        while !input.is_empty() {
            // `Coin { Locked, Unlocked => Unlocked }`
            //  ^^^^
            let event = Event::parse(input)?;

            // `Coin { Locked, Unlocked => Unlocked }`
            //         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
            let block_transition;
            braced!(block_transition in input);

            while !block_transition.is_empty() {
                let mut from_states: Vec<State> = Vec::new();

                // `Coin { Locked, Unlocked => Unlocked }`
                //                          ^^
                while !block_transition.peek(Token![=>]) {
                    // `Coin { Locked, Unlocked => Unlocked }`
                    //               ^
                    if block_transition.peek(Token![,]) {
                        let _: Comma = block_transition.parse()?;
                        continue;
                    }

                    // `Coin { Locked, Unlocked => Unlocked }`
                    //         ^^^^^^  ^^^^^^^^
                    from_states.push(State::parse(&block_transition)?);
                }

                // `Coin { Locked, Unlocked => Unlocked }`
                //                          ^^
                let _: Token![=>] = block_transition.parse()?;

                // `Coin { Locked, Unlocked => Unlocked }`
                //                             ^^^^^^^^
                let to = State::parse(&block_transition)?;

                for from in from_states {
                    let event = event.clone();
                    let to = to.clone();

                    transitions.push(Transition { event, from, to })
                }
            }
        }

        Ok(Transitions(transitions))
    }
}

impl ToTokens for Transitions {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for transition in &self.0 {
            transition.to_tokens(tokens);
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Transition {
    pub event: Event,
    pub from: State,
    pub to: State,
}

impl ToTokens for Transition {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let event = &self.event.name;
        let from = &self.from.name;
        let to = &self.to.name;

        tokens.extend(quote! {
            impl Transition<#event> for Machine<#from> {
                type Machine = Machine<#to>;

                fn transition(self, _: #event) -> Self::Machine {
                    Machine(#to)
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{format, vec};
    use proc_macro2::TokenStream;
    use syn;
    use syn::parse_quote;

    #[test]
    fn test_transition_to_tokens() {
        let transition = Transition {
            event: Event {
                name: parse_quote! { Push },
            },
            from: State {
                name: parse_quote! { Locked },
            },
            to: State {
                name: parse_quote! { Unlocked },
            },
        };

        let left = quote! {
            impl Transition<Push> for Machine<Locked> {
                type Machine = Machine<Unlocked>;

                fn transition(self, _: Push) -> Self::Machine {
                    Machine::new(Unlocked)
                }
            }
        };

        let mut right = TokenStream::new();
        transition.to_tokens(&mut right);

        assert_eq!(format!("{}", left), format!("{}", right))
    }

    #[test]
    fn test_transitions_parse() {
        let left: Transitions = syn::parse2(quote! {
            Push { Locked, Unlocked => Locked }
            Coin { Locked, Unlocked => Unlocked }
        }).unwrap();

        let right = Transitions(vec![
            Transition {
                event: Event {
                    name: parse_quote! { Push },
                },
                from: State {
                    name: parse_quote! { Locked },
                },
                to: State {
                    name: parse_quote! { Locked },
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
                    name: parse_quote! { Coin },
                },
                from: State {
                    name: parse_quote! { Unlocked },
                },
                to: State {
                    name: parse_quote! { Unlocked },
                },
            },
        ]);

        assert_eq!(left, right);
    }

    #[test]
    fn test_transitions_to_tokens() {
        let transitions = Transitions(vec![
            Transition {
                event: Event {
                    name: parse_quote! { Push },
                },
                from: State {
                    name: parse_quote! { Locked },
                },
                to: State {
                    name: parse_quote! { Locked },
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
                    name: parse_quote! { Coin },
                },
                from: State {
                    name: parse_quote! { Unlocked },
                },
                to: State {
                    name: parse_quote! { Unlocked },
                },
            },
        ]);

        let left = quote! {
            impl Transition<Push> for Machine<Locked> {
                type Machine = Machine<Locked>;

                fn transition(self, _: Push) -> Self::Machine {
                    Machine::new(Locked)
                }
            }

            impl Transition<Push> for Machine<Unlocked> {
                type Machine = Machine<Locked>;

                fn transition(self, _: Push) -> Self::Machine {
                    Machine::new(Locked)
                }
            }

            impl Transition<Coin> for Machine<Locked> {
                type Machine = Machine<Unlocked>;

                fn transition(self, _: Coin) -> Self::Machine {
                    Machine::new(Unlocked)
                }
            }

            impl Transition<Coin> for Machine<Unlocked> {
                type Machine = Machine<Unlocked>;

                fn transition(self, _: Coin) -> Self::Machine {
                    Machine::new(Unlocked)
                }
            }
        };

        let mut right = TokenStream::new();
        transitions.to_tokens(&mut right);

        assert_eq!(format!("{}", left), format!("{}", right))
    }
}
