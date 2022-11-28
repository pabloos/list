

use crate::file::File;

// -l
pub fn long(file: &&File) -> String {
    file.to_string()
}

// default
pub fn short(file: &&File) -> String {
    format!("{:<15}", file.get_name())
}