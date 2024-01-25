use regex_automata::dfa::{dense, Automaton};
use regex_automata::util::primitives::StateID;

pub mod dfa;

/// The underlying representation of the `regex-automata` DFA. From their own
/// implementation, this is the default choice.
pub type AutomatonType = dense::DFA<Vec<u32>>;

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum State {
    Start(StateID),
    Accepting(StateID),
    Dead(StateID),
    Normal(StateID),
}

impl State {
    /// Create a new [`State`].
    ///
    /// The underlying [`AutomatonType`] must be passed in order to check the
    /// exact state type as represented by the DFA.
    pub fn new(sid: StateID, automaton: &AutomatonType) -> Self {
        if automaton.is_start_state(sid) {
            Self::Start(sid)
        } else if automaton.is_match_state(sid) {
            Self::Accepting(sid)
        } else if automaton.is_dead_state(sid) {
            Self::Dead(sid)
        } else {
            Self::Normal(sid)
        }
    }

    /// Retreive the associated [`StateID`].
    #[inline]
    pub fn id(&self) -> &StateID {
        match self {
            State::Start(sid) => sid,
            State::Accepting(sid) => sid,
            State::Dead(sid) => sid,
            State::Normal(sid) => sid,
        }
    }
}
