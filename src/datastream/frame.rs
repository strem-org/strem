use self::sample::Sample;

pub mod sample;

/// A frame capture by the perception system.
///
/// This structure consider a single moment of time where all relevant channels
/// of the perception stream took a sample. It is also assumed that the system
/// used is fast enough where discrepency between channels does not
/// significanltly exist.
#[derive(Clone, Debug)]
pub struct Frame {
    pub index: usize,
    pub timestamp: f64,

    // A mapping between the channel name and data sample
    pub samples: Vec<Sample>,
}

impl Frame {
    /// Create a new [`Frame`].
    pub fn new(index: usize, timestamp: f64) -> Self {
        Frame {
            index,
            timestamp,
            samples: Vec::new(),
        }
    }
}
