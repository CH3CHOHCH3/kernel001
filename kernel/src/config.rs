pub const PAGE_SIZE: usize = 4096;
pub const PHSY_MEM_TOP: usize = 0x88000000;

#[macro_export]
macro_rules! pgroundup {
    ($add: expr) => {
        (((add + PAGE_SIZE - 1) >> 12) << 12)
    }
}

#[macro_export]
macro_rules! pgrounddown {
    ($add: expr) => {
        ((add >> 12) << 12)
    }
}