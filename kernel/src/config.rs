const PAGE_SIZE: u64 = 4096;
const PHSY_MEM_SIZE: u64 = 128*1024*1024;

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