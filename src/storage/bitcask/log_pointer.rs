pub struct LogPointer {
    pub gen: u64,
    pub pos: u64,
    pub len: u64,
}

impl LogPointer {
    pub fn new(gen: u64, pos: u64, len: u64) -> LogPointer {
        LogPointer { gen, pos, len }
    }
}
