pub mod dfa;

use std::error::Error;

use regex_automata::{
    dfa::{dense, Automaton as AutomatonTrait},
    nfa::thompson,
    util::id::StateID,
    SyntaxConfig,
};

use dfa::{Automaton, AutomatonType, DeterministicFiniteAutomata};

/// A convenience interface for building DFAs.
///
/// The interface makes a few assumptions about the configurations of the
/// resulting DFA that are standard.
#[derive(Default)]
pub struct Builder {
    pub reverse: bool,
}

impl Builder {
    pub fn new() -> Self {
        Builder { reverse: false }
    }

    /// Toggle whether a reverse DFA is built.
    ///
    /// This should be configured/called before calling out the the `build`
    /// method, accordingly.
    pub fn reverse(mut self) -> Self {
        self.reverse = !self.reverse;
        self
    }

    pub fn build(&self, pattern: &str) -> Result<DeterministicFiniteAutomata, Box<dyn Error>> {
        let automaton = Automaton::new(
            dense::Builder::new()
                .configure(dense::Config::new().minimize(true))
                .syntax(SyntaxConfig::new().unicode(false).utf8(true))
                .thompson(thompson::Config::new().reverse(self.reverse).utf8(true))
                .build(pattern)?,
        );

        if self.reverse {
            Ok(DeterministicFiniteAutomata::Reverse(automaton))
        } else {
            Ok(DeterministicFiniteAutomata::Forward(automaton))
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum State {
    Normal(StateID),
    Initial(StateID),
    Accepting(StateID),
    Dead(StateID),
    Unknown(StateID),
}

impl State {
    /// Create a new wrapped [`State`].
    ///
    /// The automaton must be passed in order to perform state identifier kind
    /// identification (e.g., start, match, etc).
    pub fn new(id: StateID, automaton: &AutomatonType) -> Self {
        if automaton.is_special_state(id) {
            if automaton.is_start_state(id) {
                Self::Initial(id)
            } else if automaton.is_match_state(id) {
                Self::Accepting(id)
            } else if automaton.is_dead_state(id) {
                Self::Dead(id)
            } else {
                Self::Unknown(id)
            }
        } else {
            Self::Normal(id)
        }
    }

    pub fn id(&self) -> Option<&StateID> {
        match self {
            State::Normal(id) => Some(id),
            State::Initial(id) => Some(id),
            State::Accepting(id) => Some(id),
            State::Dead(id) => Some(id),
            State::Unknown(id) => Some(id),
        }
    }
}
