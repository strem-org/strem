use super::reader::Sample;

/// A `Frame` represents a single moment of time.
///
/// The frame includes information regarding all relevant sensor samples such
/// that the time difference between sample readings is negligible.
#[derive(Clone, Debug)]
pub struct Frame {
    pub index: usize,
    pub samples: Vec<Sample>,
}

impl Frame {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            samples: Vec::new(),
        }
    }
}
