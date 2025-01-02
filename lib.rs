#![doc = include_str!("README.md")]
#![no_std]
#![forbid(unsafe_code)]

mod internal;
use internal::{Iter, PseudoFunc};

mod iter_scan;
mod scan_clone;
mod scan_copy;
mod scan_state_clone;
mod scan_state_copy;
mod scan_with_tuple;

pub use iter_scan::IterScan;
pub use scan_clone::ScanClone;
pub use scan_copy::ScanCopy;
pub use scan_state_clone::ScanStateClone;
pub use scan_state_copy::ScanStateCopy;
pub use scan_with_tuple::ScanWithTuple;
