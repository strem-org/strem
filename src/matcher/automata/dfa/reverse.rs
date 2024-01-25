use std::collections::{HashMap, HashSet};
use std::error::Error;

use regex_automata::dfa::{dense, Automaton, StartError, StartKind};
use regex_automata::nfa::thompson;
use regex_automata::util::start::Config;
use regex_automata::util::syntax;
use regex_automata::{Anchored, HalfMatch, PatternID};

use crate::compiler::ir::ast::SpatialFormula;
use crate::datastream::frame::Frame;
use crate::matcher::automata::{AutomatonType, State};
use crate::monitor::Monitor;
use crate::symbolizer::ast::SymbolicAbstractSyntaxTree as AST;

use super::{DeterministicFiniteAutomaton, OFFSET};

/// A reverse matching DFA.
///
/// This DFA is configured for anchored searches. Therefore, it should be ran
/// only to find the start position of a search.
pub struct DeterministicFiniteAutomata<'a> {
    pub automata: AutomatonType,
    pub fmap: HashMap<char, &'a SpatialFormula>,
}

impl DeterministicFiniteAutomaton for DeterministicFiniteAutomata<'_> {
    /// Simulate the DFA.
    ///
    /// This simulates the DFA on a slice of [`Frame`]. The default behavior is
    /// to find the longest leftmost match. It is assumed that all matches are
    /// anchored (i.e., a match always begins at the first frame provided).
    ///
    /// As a result of this behavior, it is recommended to call run incrementally
    /// to collect all possible matches over the complete haystack.
    fn run(&self, haystack: &[Frame]) -> Result<Vec<HalfMatch>, Box<dyn Error>> {
        let mut mats = Vec::new();
        let mut states = HashSet::new();

        // Initialize states with the start state of the DFA.
        states.insert(self.initial()?);

        for (at, frame) in haystack.iter().enumerate().rev() {
            // Get the next set of states.
            //
            // This should generate a new [`HashSet`] with only the next set of
            // states. We do not keep a historical record of previously visited
            // states to reduce memory usage.
            states = states
                .into_iter()
                .flat_map(|state| self.transition(state, frame))
                .collect();

            // For each state, take action upon it.
            //
            // It is important to produce any [`HalfMatch`] when an
            // [`State::Accepting`] is seen.
            for state in states.iter() {
                match state {
                    State::Accepting(..) => {
                        // Because reported matches follow a half-open range
                        // (i.e., start is inclusive, and end is exclusive), we
                        // add 1 to the [`HalfMatch`].
                        //
                        // For more information, see:
                        // https://github.com/rust-lang/regex/blob/027eebd6fde307076603530c999afcfd271bb037/regex-automata/src/dfa/search.rs#L271
                        mats.push(HalfMatch::new(PatternID::new(0)?, at + OFFSET));
                    }
                    _ => continue,
                }
            }

            // Return current matches if all states are dead.
            //
            // This is checked after producing potential [`HalfMatch`] as the
            // condition to exit is only when all branches are dead---contrary to
            // single branch execution.
            if states.iter().all(|state| matches!(state, State::Dead(..))) {
                return Ok(mats);
            }
        }

        for state in states {
            if let Some(m) = self.eoi(state)? {
                mats.push(m);
            }
        }

        Ok(mats)
    }
}

impl<'a> DeterministicFiniteAutomata<'a> {
    /// Create a new reverse-matching DFA.
    ///
    /// This function is exposed if a different configuration is requierd.
    /// Otherwise, for all other cases, use the [`self::build`] interface to
    /// construct this DFA.
    pub fn new(automata: AutomatonType, fmap: HashMap<char, &'a SpatialFormula>) -> Self {
        DeterministicFiniteAutomata { automata, fmap }
    }

    /// Take the next transition on the `Frame`.
    ///
    /// For this implementation, whether to take a transition is determined by
    /// whether the [`Monitor`] evaluates to true on the [`Frame`]. The cases are
    /// as follows:
    ///
    /// I. If true, transition on the corresponding symbol from the [`State`].
    /// II. If false, transition on a blank symbol from the [`State`].
    ///
    /// For (II), this is similar to transitioning on a byte that is not in teh
    /// pattern of a traditional RE.
    fn transition(&self, state: State, frame: &Frame) -> HashSet<State> {
        let monitor = Monitor::new();
        let mut nexts = HashSet::new();

        for (symbol, formula) in self.fmap.iter() {
            if monitor.evaluate(frame, formula) {
                let sid = self.automata.next_state(*state.id(), *symbol as u8);
                let next = State::new(sid, &self.automata);

                nexts.insert(next);
            }
        }

        if nexts.is_empty() {
            let sid = self.automata.next_state(*state.id(), b'Z');
            let next = State::new(sid, &self.automata);

            nexts.insert(next);
        }

        nexts
    }

    /// Check EOI.
    ///
    /// The End of Input (EOI) is checked for a final match. If taking the EOI
    /// transition results in a match state, then return as final match.
    fn eoi(&self, state: State) -> Result<Option<HalfMatch>, Box<dyn Error>> {
        if let State::Accepting(..) = self.transitioneoi(state) {
            return Ok(Some(HalfMatch::new(PatternID::new(0)?, 0)));
        }

        Ok(None)
    }

    /// Take the extra byte transition.
    ///
    /// This function must be called as the last transition before checking if a
    /// the DFA is in a matching state.
    ///
    /// For more information, see
    /// [here](https://docs.rs/regex-automata/0.4.3/regex_automata/dfa/trait.Automaton.html#tymethod.next_eoi_state).
    fn transitioneoi(&self, state: State) -> State {
        State::new(self.automata.next_eoi_state(*state.id()), &self.automata)
    }

    /// Retrieve the initial [`State`] to start from an Automata.
    ///
    /// For further information, see `regex_automata::util::start`.
    fn initial(&self) -> Result<State, StartError> {
        // Retrieve the start state.
        //
        // The start state is anchored as all inputs to this
        // [`DeterministicFiniteAutomata`] begin searching at index 0. Therefore,
        // matches are only found starting from the beginning (i.e., anchored).
        let sid = self
            .automata
            .start_state(&Config::new().anchored(Anchored::Yes))?;

        // The start state shall never be the match state.
        //
        // This is true as all matches in the DFA configurations are delayed by
        // a single transition (i.e., byte).
        //
        // For more information, see:
        // [here](https://github.com/rust-lang/regex/blob/027eebd6fde307076603530c999afcfd271bb037/regex-automata/src/dfa/search.rs#L552).
        debug_assert!(!self.automata.is_match_state(sid));

        Ok(State::new(sid, &self.automata))
    }
}

/// Build a reverse searching DFA.
///
/// The `regex-automata` library is used primarily here to construct the
/// underlying state machine that performs matching. We then wrap this result
/// into a [`DeterministicFiniteAutomata`] for simple interfacing.
pub fn build(ast: &AST) -> Result<DeterministicFiniteAutomata, Box<dyn Error>> {
    let automata = dense::Builder::new()
        .configure(
            dense::Config::new()
                .minimize(true)
                .accelerate(false)
                .start_kind(StartKind::Anchored)
                .specialize_start_states(true),
        )
        .syntax(syntax::Config::new().unicode(false).utf8(true))
        .thompson(thompson::Config::new().reverse(true).utf8(true))
        .build(&super::super::super::regexify(ast))?;

    let fmap = ast
        .fmap()
        .iter()
        .map(|x| (x.symbol, &x.formula))
        .collect::<HashMap<char, &SpatialFormula>>();

    Ok(DeterministicFiniteAutomata::new(automata, fmap))
}
