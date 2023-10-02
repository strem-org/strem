use self::frame::Frame;

pub mod frame;
pub mod reader;

#[derive(Debug)]
pub struct DataStream {
    pub frames: Vec<Frame>,
}

impl DataStream {
    pub fn new() -> Self {
        Self { frames: Vec::new() }
    }
}
