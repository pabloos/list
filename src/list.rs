
use std::fs;
use std::cmp::Ordering;
use std::path::Path;
use std::error::Error;

use crate::file::File;

pub fn list<Filter, Sort, Format>(path: &Path, filter: Filter, sorter: Sort, format: Format) -> Result<String, Box<dyn Error>> 
    where Filter: Fn(&&File) -> bool,
          Sort: FnMut(&&File, &&File) -> Ordering,
          Format: Fn(&&File) -> String
{
    let dir = fs::read_dir(path)?;

    let files: Vec<File> = dir.map(|entry| File::new(entry?))
                            .filter_map(Result::ok)
                            .collect();

    let mut filtered: Vec<&File> = files.iter()
                                        .filter(filter)
                                        .collect();

    filtered.sort_by(sorter);

    let lines: Vec<String> = filtered.iter()
                                    .map(format)
                                    .collect();

    Ok(lines.join(""))
}