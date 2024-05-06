#![no_std]
#![forbid(unsafe_code)]

use aster_frame::prelude::*;

#[aster_main]
pub fn kern() {
    println!("[kernel] halt");
    loop {}
}

#[cfg(ktest)]
mod tests {
    #[ktest]
    fn it_works() {
        let memory_regions = aster_frame::boot::memory_regions();
        assert!(!memory_regions.is_empty());
    }
}
