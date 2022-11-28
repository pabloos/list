
use std::cmp::Ordering;
use crate::file::File;

// -t
pub fn time(f1: &&File, f2: &&File) -> Ordering {
    f1.modified().cmp(&f2.modified())
}

// -S
pub fn size(f1: &&File, f2: &&File) -> Ordering {
    f1.size().cmp(&f2.size())
}

// default
pub fn no_order(_: &&File, _: &&File) -> Ordering {
    Ordering::Equal
}