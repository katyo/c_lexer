//#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate phf;

pub mod token;
#[macro_use]
mod macros;
mod equivalence;
pub mod error;
mod identifier;
mod number;
mod state;
mod state_machine;
mod string;

/// Module for efficient string representation
pub mod internship {
    extern crate internship;

    pub use internship::*;
}

use self::{state_machine::parse, token::*};

/// Lexer implementation
#[derive(Debug, Copy, Clone)]
pub struct Lexer;

impl Lexer {
    /// Transform string to stream of tokens
    pub fn lex(s: &str) -> Result<Vec<Token>, error::Error> {
        let mut tokens = parse(s)?;
        tokens.push(Token::EOF);
        Ok(tokens)
    }
}
