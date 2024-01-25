use std::error::Error;

use crate::datastream::frame::Frame;
use crate::symbolizer::ast::SymbolicAbstractSyntaxTree;

use super::super::matcher::Matching;
use super::automata::dfa::forward::DeterministicFiniteAutomata;
use super::automata::dfa::{forward, DeterministicFiniteAutomaton};
use super::Match;

/// An interface for [`Matching`] offline.
///
/// This [`Matcher`] uses a forward-based algorithm to perform matching over the
/// provided input.
pub struct Matcher<'a> {
    pub dfa: DeterministicFiniteAutomata<'a>,
}

impl Matching for Matcher<'_> {
    fn leftmost(&self, frames: &[Frame]) -> Result<Option<Match>, Box<dyn Error>> {
        let start: usize = frames.first().unwrap().index;

        let end = self
            .dfa
            .run(frames)?
            .into_iter()
            .filter(|m| start != start + m.offset())
            .map(|m| start + m.offset())
            .max();

        if let Some(end) = end {
            return Ok(Some(Match::new(start, end)));
        }

        Ok(None)
    }
}

impl<'a> From<&'a SymbolicAbstractSyntaxTree> for Matcher<'a> {
    fn from(ast: &'a SymbolicAbstractSyntaxTree) -> Self {
        // Construct the DFA.
        //
        // Here we use the forward factory to construct a DFA from the s-AST
        // provided that is first converted into an RE.
        //
        // # Panics
        //
        // Here, we assume that all previous methods have passed where we can
        // safely assume that constructing a valid DFA is guaranteed. This may
        // need further handled in the future for patterns that may break the
        // underlying library used.
        let dfa = forward::build(ast).unwrap();

        Matcher { dfa }
    }
}
