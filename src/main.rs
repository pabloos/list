use std::path::Path;
use std::fs::read_dir;

fn print_files(dir: &Path) {
	for entry in read_dir(dir).unwrap() {
		let file_name = entry.unwrap().file_name().into_string().unwrap();

		println!("{}", file_name);
	}
}

fn main() {
    let path = Path::new(".");

	print_files(path);
}