use alloc::vec::{IntoIter, Vec};
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream, Result};
use syn::Ident;

#[derive(Debug, PartialEq)]
pub(crate) struct States(pub Vec<State>);

impl ToTokens for States {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for state in &self.0 {
            state.to_tokens(tokens);

            let name = &state.name;
            for other in &self.0 {
                let other = &other.name;
                let eq = name == other;

                tokens.extend(quote! {
                    impl PartialEq<#other> for #name {
                        fn eq(&self, _: & #other) -> bool {
                            #eq
                        }
                    }
                });
            }
        }
    }
}

#[allow(single_use_lifetimes)] // TODO: how to fix this?
impl<'a> IntoIterator for &'a States {
    type Item = State;
    type IntoIter = IntoIter<State>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.clone().into_iter()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct State {
    pub name: Ident,
}

impl Parse for State {
    /// example state tokens:
    ///
    /// ```text
    /// Locked
    /// ```
    ///
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let name = input.parse()?;

        Ok(State { name })
    }
}

impl ToTokens for State {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;

        tokens.extend(quote! {
            #[derive(Clone, Copy, Debug, Eq)]
            pub struct #name;
            impl State for #name {}
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
    fn test_state_parse() {
        let left: State = syn::parse2(quote! { Unlocked }).unwrap();
        let right = State {
            name: parse_quote! { Unlocked },
        };

        assert_eq!(left, right);
    }

    #[test]
    fn test_state_to_tokens() {
        let state = State {
            name: parse_quote! { Unlocked },
        };

        let left = quote! {
            #[derive(Clone, Copy, Debug, Eq)]
            pub struct Unlocked;
            impl State for Unlocked {}
        };

        let mut right = TokenStream::new();
        state.to_tokens(&mut right);

        assert_eq!(format!("{}", left), format!("{}", right))
    }

    #[test]
    fn test_states_to_tokens() {
        let states = States(vec![
            State {
                name: parse_quote! { Locked },
            },
            State {
                name: parse_quote! { Unlocked },
            },
        ]);

        let left = quote! {
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
        };

        let mut right = TokenStream::new();
        states.to_tokens(&mut right);

        assert_eq!(format!("{}", left), format!("{}", right))
    }
}
