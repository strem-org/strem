use std::error::Error;

use regex_automata::HalfMatch;

use crate::datastream::frame::Frame;

pub mod forward;
pub mod reverse;

/// A trait for all DFA's.
///
/// This trait provides a common interface that all matchers may use.
pub trait DeterministicFiniteAutomaton {
    /// Run the DFA.
    ///
    /// The main interface for which all DFA's must implement is to simulate the
    /// corresponding DFA and return a set of valid [`HalfMatch`].
    fn run(&self, haystack: &[Frame]) -> Result<Vec<HalfMatch>, Box<dyn Error>>;
}

/// The default size to offset all matches by.
///
/// This is set as the end part of a match is exclusive (i.e., open), so the
/// actual end index should be offset, accordingly.
pub const OFFSET: usize = 1;
