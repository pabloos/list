

use crate::file::File;

// no filter tag
pub fn default(file: &&File) -> bool {
    !file.is_hidden()
}

// -a
pub fn all(_: &&File) -> bool {
    true
}

// -d
pub fn dirs(file: &&File) -> bool {
    file.is_dir()
}