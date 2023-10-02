use linked_hash_set::LinkedHashSet;
use regex_automata::{
    dfa::{dense, Automaton as AutomatonTrait},
    HalfMatch,
};

use super::State;

/// The main interface of a DFA.
///
/// There are two kinds of DFAs: (1) forward, and (2) reverse. The forward DFAs
/// are responsible for finding the end index of a match; and the reverse DFAs
/// are responsible for finding the start index of a match.
pub enum DeterministicFiniteAutomata {
    Forward(Automaton),
    Reverse(Automaton),
}

impl DeterministicFiniteAutomata {
    pub fn step(&mut self, bytes: &[u8]) {
        match self {
            Self::Forward(a) => a.forward(bytes),
            Self::Reverse(a) => a.reverse(bytes),
        }
    }

    pub fn isdead(&self) -> bool {
        match self {
            Self::Forward(..) => panic!("forward not implemented"),
            Self::Reverse(a) => a.isdead || a.states.is_empty(),
        }
    }

    pub fn ismatch(&mut self) -> bool {
        match self {
            Self::Forward(..) => panic!("forward not implemented"),
            Self::Reverse(a) => {
                let res = a.ismatch;
                a.ismatch = false;

                res
            }
        }
    }

    pub fn eoi(&self) -> bool {
        match self {
            Self::Forward(..) => panic!("forward not implemented"),
            Self::Reverse(a) => {
                if let Some(..) = a.eoi() {
                    return true;
                }

                false
            }
        }
    }

    pub fn reset(&mut self) {
        match self {
            Self::Forward(..) => panic!("forward not implemented"),
            Self::Reverse(a) => {
                a.states.clear();
                a.states.insert(Automaton::initial(&a.automaton));

                // Reset the flags of the automaton.
                a.ismatch = false;
                a.isdead = false;
            }
        }
    }
}

/// The underlying representation of the `regex-automata` DFA. From their own
/// implementation, this is the default choice.
pub type AutomatonType = dense::DFA<Vec<u32>>;

/// A wrapper interface over the `regex-automata` library's DFA.
///
/// This wrapper enables the storage of the state of the automaton that is
/// otherwise note stored in the supplying library---supports online and
/// incremental pattern matching.
pub struct Automaton {
    pub automaton: AutomatonType,
    pub states: LinkedHashSet<State>,

    // To speed-up matching, we should store key states that the automaton is in,
    // so checking does not consist of unneeded iterations.
    pub ismatch: bool,
    pub isdead: bool,
}

impl Automaton {
    pub fn new(automaton: dense::DFA<Vec<u32>>) -> Self {
        let mut states = LinkedHashSet::new();

        // Upon creation of a new [`Automaton`], it should always be initialized
        // with the start state.
        states.insert(Automaton::initial(&automaton));

        Self {
            automaton,
            states,
            ismatch: false,
            isdead: false,
        }
    }

    /// Increment a forward DFA.
    ///
    /// This function should not be used if the constructed DFA is a reversed
    /// version of the pattern. For that, you should use the `reverse` function
    /// to perform a simliar objective.
    fn forward(&self, _bytes: &[u8]) {}

    /// Increment a reverse DFA.
    ///
    /// This function should not be used if the constructed DFA is a forward
    /// version of the pattern. For that, you should use the `forward` function
    /// to perform a simliar objective.
    fn reverse(&mut self, bytes: &[u8]) {
        // Retrieve the next set of states by transitioning on the provided set
        // of bytes, accordingly.
        self.states = self.transition(bytes);

        // Check for dead states from the set of next states transitioned.
        for state in self.states.iter() {
            match state {
                State::Accepting(..) => self.ismatch = true,
                State::Dead(..) => {
                    // If we see a dead state, we can return instantly as
                    // searching any further will not lead any promising
                    // results, accordingly.
                    self.isdead = true;
                    return;
                }
                _ => continue,
            }
        }
    }

    /// Transition to the next set of states from the bytes.
    fn transition(&self, bytes: &[u8]) -> LinkedHashSet<State> {
        let mut nexts = LinkedHashSet::new();

        for state in self.states.iter() {
            for byte in bytes.iter() {
                if let Some(id) = state.id() {
                    let id = self.automaton.next_state(*id, *byte);
                    let next = State::new(id, &self.automaton);

                    nexts.insert(next);
                }
            }
        }

        nexts
    }

    /// Transition to the next set of states from the EOI specialized byte.
    fn transitioneoi(&self) -> LinkedHashSet<State> {
        let mut nexts = LinkedHashSet::new();

        for state in self.states.iter() {
            if let Some(id) = state.id() {
                let id = self.automaton.next_eoi_state(*id);
                let next = State::new(id, &self.automaton);

                nexts.insert(next);
            }
        }

        nexts
    }

    /// Check if a match state would appear at EOI.
    ///
    /// The implementation of the `regex-automata` crate is designed in such a
    /// way that there is always an extra state that needs to be blankly
    /// transitioned to.
    fn eoi(&self) -> Option<HalfMatch> {
        let states = self.transitioneoi();

        for state in states.iter() {
            match state {
                State::Accepting(id) => {
                    return Some(HalfMatch::new(self.automaton.match_pattern(*id, 0), 0));
                }
                _ => continue,
            }
        }

        None
    }

    /// Retrieve the initial [`State`] to start from an Automata.
    ///
    /// Note: This implementation is "Jerry-Rigged". Unless a future API
    /// exposes a nicer way to retrieve the start state of a DFA, then this is
    /// the current workaround with the `regex-automata` crate given that we
    /// are modifying the transition procedure.
    ///
    /// The start state will always be the same as multiple patterns are not
    /// supported. Therefore, multiple start states are not possible in the
    /// current use of the `regex-automata` library.
    ///
    /// For further information, see `regex_automata::util::start`.
    fn initial(automaton: &dense::DFA<Vec<u32>>) -> State {
        let id = automaton.start_state_reverse(None, &[1], 0, 1);
        State::new(id, automaton)
    }
}
