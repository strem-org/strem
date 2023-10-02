//! The matching framework for SpREs.
//!

use crate::{
    datastream::frame::Frame,
    monitor::Monitor,
    symbolizer::ast::{SymbolicAbstractSyntaxTree, SymbolicFormula},
};

use self::regex::RegularExpression;

pub mod automata;
pub mod regex;

const OFFSET: usize = 1;

/// The interface to perform pattern matching.
///
/// The pattern matching is performed from a SpRE against a perception data
/// stream, accordingly.
pub struct Matcher<'a> {
    pub regex: RegularExpression,
    pub fmap: Vec<&'a SymbolicFormula>,
}

impl<'a> Matcher<'a> {
    /// Increment the matcher by a single frame.
    ///
    /// This involves first evaluating the frame against all spatial formulas and
    /// producing the set of possible symbols to transition on.
    pub fn step(&mut self, frame: &Frame) -> Option<Frame> {
        let monitor = Monitor::new();
        let mut symbols = Vec::new();

        for spform in self.fmap.iter() {
            if monitor.evaluate(frame, &spform.formula) {
                symbols.push(spform.symbol as u8);
            }
        }

        // Update the underlying RE with the set of symbols to transition on.
        self.regex.step(&symbols);

        if self.regex.ismatch() {
            return Some(frame.clone());
        }

        None
    }

    /// Run the matcher against a slice of frames.
    ///
    /// This is most common function to call.
    pub fn run(&mut self, frames: &[Frame]) -> Option<Match> {
        let mut start = frames.len();

        for frame in frames.iter().rev() {
            // If a match is found (i.e, Some), then the start position can be
            // extended. However, if a match is not found, it does not indicate
            // that we should break.
            //
            // For example, consider concatenation (e.g., `ab`). The first frame
            // will not produce a match, but the second frame will---assuming
            // that both frames satisfy the corresponding spatial formulas.
            if let Some(m) = self.step(frame) {
                start = m.index + self::OFFSET;
            }

            // Before we extend the start state of the match, we must check
            // if the underlying DFA holds a [`State::Dead`] state.
            //
            // If a dead state is present, the DFA is in no position to
            // continue; therefore, we should break and return the match
            // with start state we have found up until this point--assuming
            // one has been found, accordingly.
            if self.regex.isdead() {
                break;
            }
        }

        // At the end of matching, the EOI special byte should be checked. This
        // step is a necessary step of the `regex-automata` crate as there is an
        // additional state that requires an extra transition.
        if self.regex.eoi() && !self.regex.isdead() {
            start = 0;
        }

        if start < frames.len() {
            // Collect frames of with the match range.
            return Some(Match::new(frames.iter().skip(start).cloned().collect()));
        }

        None
    }

    pub fn reset(&mut self) {
        self.regex.reset();
    }
}

impl<'a> From<&'a SymbolicAbstractSyntaxTree> for Matcher<'a> {
    /// Construct a [`Matcher`] from a `[SymbolicAbstractSyntaxTree]`, accordingly.
    ///
    /// This indirectly creates a [`RegularExpression`] from the set of symbols
    /// that are mapped to spatial formulas.
    fn from(ast: &'a SymbolicAbstractSyntaxTree) -> Self {
        let regex = RegularExpression::from(ast);
        let fmap = ast.fmap();

        Self { regex, fmap }
    }
}

#[derive(Debug)]
pub struct Match {
    pub frames: Vec<Frame>,
}

impl Match {
    pub fn new(frames: Vec<Frame>) -> Self {
        Self { frames }
    }
}
