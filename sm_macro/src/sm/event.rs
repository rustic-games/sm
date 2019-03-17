use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::Ident;

#[allow(unused)]
#[derive(Debug)]
pub(crate) struct Events(pub Vec<Event>);

impl ToTokens for Events {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for event in &self.0 {
            event.to_tokens(tokens);

            let name = &event.name;
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

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Event {
    pub name: Ident,
}

impl Parse for Event {
    /// example event tokens:
    ///
    /// ```text
    /// Push
    /// ```
    ///
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let name = input.parse()?;

        Ok(Event { name })
    }
}

impl ToTokens for Event {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;

        tokens.extend(quote! {
            #[derive(Clone, Copy, Debug, Eq)]
            pub struct #name;
            impl Event for #name {}
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::TokenStream;
    use syn;
    use syn::parse_quote;

    #[test]
    fn test_event_parse() {
        let left: Event = syn::parse2(quote! { Push }).unwrap();
        let right = Event {
            name: parse_quote! { Push },
        };

        assert_eq!(left, right);
    }

    #[test]
    fn test_event_to_tokens() {
        let event = Event {
            name: parse_quote! { Push },
        };

        let left = quote! {
            #[derive(Clone, Copy, Debug, Eq)]
            pub struct Push;
            impl Event for Push {}
        };

        let mut right = TokenStream::new();
        event.to_tokens(&mut right);

        assert_eq!(format!("{}", left), format!("{}", right))
    }

    #[test]
    fn test_events_to_tokens() {
        let events = Events(vec![
            Event {
                name: parse_quote! { Push },
            },
            Event {
                name: parse_quote! { Coin },
            },
        ]);

        let left = quote! {
            #[derive(Clone, Copy, Debug, Eq)]
            pub struct Push;
            impl Event for Push {}

            impl PartialEq<Push> for Push {
                fn eq(&self, _: &Push) -> bool {
                    true
                }
            }

            impl PartialEq<Coin> for Push {
                fn eq(&self, _: &Coin) -> bool {
                    false
                }
            }

            #[derive(Clone, Copy, Debug, Eq)]
            pub struct Coin;
            impl Event for Coin {}

            impl PartialEq<Push> for Coin {
                fn eq(&self, _: & Push) -> bool {
                    false
                }
            }

            impl PartialEq<Coin> for Coin {
                fn eq(&self, _: & Coin) -> bool {
                    true
                }
            }
        };

        let mut right = TokenStream::new();
        events.to_tokens(&mut right);

        assert_eq!(format!("{}", left), format!("{}", right))
    }
}
