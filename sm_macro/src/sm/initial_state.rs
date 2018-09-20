use alloc::vec::Vec;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{braced, Ident, Token};

#[derive(Debug, PartialEq)]
pub(crate) struct InitialStates(pub Vec<InitialState>);

impl Parse for InitialStates {
    /// example initial states tokens:
    ///
    /// ```text
    /// InitialStates { Locked, Unlocked }
    /// ```
    ///
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut initial_states: Vec<InitialState> = Vec::new();

        // `InitialStates { ... }`
        //  ^^^^^^^^^^^^^
        let block_name: Ident = input.parse()?;

        if block_name != "InitialStates" {
            return Err(input.error("expected `InitialStates { ... }` block"));
        }

        // `InitialStates { ... }`
        //                  ^^^
        let block_initial_states;
        braced!(block_initial_states in input);

        // `InitialStates { Locked, Unlocked }`
        //                  ^^^^^^  ^^^^^^^^
        let punctuated_initial_states: Punctuated<Ident, Token![,]> =
            block_initial_states.parse_terminated(Ident::parse)?;

        for name in punctuated_initial_states {
            initial_states.push(InitialState { name });
        }

        Ok(InitialStates(initial_states))
    }
}

impl ToTokens for InitialStates {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for state in &self.0 {
            state.to_tokens(tokens);
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct InitialState {
    pub name: Ident,
}

impl Parse for InitialState {
    /// example initial state tokens:
    ///
    /// ```text
    /// Locked
    /// ```
    ///
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let name = input.parse()?;

        Ok(InitialState { name })
    }
}

impl ToTokens for InitialState {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;

        tokens.extend(quote! {
            impl InitialState for #name {}
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{format, vec};
    use quote::quote;
    use syn::{parse2, parse_quote};

    #[test]
    fn test_initial_state_parse() {
        let left: InitialState = parse2(quote! { Unlocked }).unwrap();
        let right = InitialState {
            name: parse_quote! { Unlocked },
        };

        assert_eq!(left, right);
    }

    #[test]
    fn test_initial_state_to_tokens() {
        let initial_state = InitialState {
            name: parse_quote! { Unlocked },
        };

        let left = quote! {
            impl InitialState for Unlocked {}
        };

        let mut right = TokenStream::new();
        initial_state.to_tokens(&mut right);

        assert_eq!(format!("{}", left), format!("{}", right))
    }

    #[test]
    fn test_initial_states_parse() {
        let left: InitialStates = parse2(quote! {
            InitialStates { Locked, Unlocked }
        }).unwrap();

        let right = InitialStates(vec![
            InitialState {
                name: parse_quote! { Locked },
            },
            InitialState {
                name: parse_quote! { Unlocked },
            },
        ]);

        assert_eq!(left, right);
    }

    #[test]
    fn test_initial_states_to_tokens() {
        let initial_states = InitialStates(vec![
            InitialState {
                name: parse_quote! { Locked },
            },
            InitialState {
                name: parse_quote! { Unlocked },
            },
        ]);

        let left = quote! {
            impl InitialState for Locked {}
            impl InitialState for Unlocked {}
        };

        let mut right = TokenStream::new();
        initial_states.to_tokens(&mut right);

        assert_eq!(format!("{}", left), format!("{}", right))
    }
}
