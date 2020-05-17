pub struct LogPointer {
    pub pos: u64,
    pub len: u64,
}

impl LogPointer {
    pub fn new(pos: u64, len: u64) -> LogPointer {
        LogPointer { pos, len }
    }
}
