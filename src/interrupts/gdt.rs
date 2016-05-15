

bitflags! {
    flags CodeSegmentDescriptor: u64 {
        const PRESENT = 1 << 47,
        const CONFORMING = 1 << 42,
        const DPL_0 = 1 << 45,
        const DPL_1 = 1 << 46,
        const ONE_0 = 1 << 43,
        const ONE_1 = 1 << 44,
        const ONE_2 = 1 << 53,
    }
}

impl CodeSegmentDescriptor {
    pub fn new() -> Self {
        PRESENT | ONE_0 | ONE_1 | ONE_2
    }
}

bitflags! {
    flags DataSegmentDescriptor: u64 {
        const PRESENT = 1 << 47,
        const ONE_0 = 1 << 44,
    }
}

impl DataSegmentDescriptor {
    pub fn new() -> Self {
        PRESENT | ONE_0
    }
}
