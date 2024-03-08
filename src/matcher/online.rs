use std::error::Error;

use crate::datastream::frame::Frame;
use crate::symbolizer::ast::SymbolicAbstractSyntaxTree;

use super::super::matcher::Matching;
use super::automata::dfa::reverse::DeterministicFiniteAutomata;
use super::automata::dfa::{reverse, DeterministicFiniteAutomaton};
use super::Match;

/// An interface for [`Matching`] online.
///
/// This [`Matcher`] uses a reverse-based algorithm to perform matching over the
/// provided input.
pub struct Matcher<'a> {
    pub dfa: DeterministicFiniteAutomata<'a>,
}

impl<'a> Matching for Matcher<'a> {
    /// Find the leftmost match from the sequence of [`Frame`].
    ///
    /// This algorithm utilizes an anchored reverse DFA. Therefore, the `start`
    /// index needs to be found, accordingly.
    ///
    /// The indices of the [`Match`] returned correspond to the indices relative
    /// to the sequences of [`Frame`] provided. Therefore, it is not necessarily
    /// true that the [`Match::start`] and [`Match::end`] values correspond to
    /// the actual index of the [`Frame`].
    ///
    /// # Example
    ///
    /// If a provided set of six [`Frame`] have indices that range from 5 to 10,
    /// and the last two match, then the [`Match`] range will be from [4, 6) and
    /// not [8,10).
    ///
    /// As such, the [`Match`] acts as the index relative to the length of the
    /// slice of [`Frame`] provided.
    fn leftmost(&self, frames: &[Frame]) -> Result<Option<Match>, Box<dyn Error>> {
        let end: usize = frames.len();

        let start = self
            .dfa
            .run(frames)?
            .into_iter()
            .filter(|m| end != m.offset())
            .map(|m| m.offset())
            .min();

        if let Some(start) = start {
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
        let dfa = reverse::build(ast).unwrap();

        Matcher { dfa }
    }
}
