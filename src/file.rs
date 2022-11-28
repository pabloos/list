
use std::fs;
use std::fmt;
use std::error::Error;
use std::os::unix::fs::{PermissionsExt, MetadataExt};

use chrono::{DateTime, Local};

use crate::owners::*;
use crate::permissions::Section;

const DIR_MARK: char = 'd';
const BLK_MARK: char = '-';

const TIME_FORMAT: &str = "%_d %b %H:%M";

pub struct File {
	name: String,
	is_dir: bool,
	size: u64,
	permisions: Section,
	modified: DateTime<Local>,

	num_links: u64,

	user_name: String,
	group_name: String
}

impl File {
	pub fn new(entry: fs::DirEntry) -> Result<Self, Box<dyn Error>> {
		let metadata = entry.metadata()?;

		let mode = metadata.permissions().mode();

		let (uid, gid) = (metadata.uid(), metadata.gid());

		Ok(File {
			name: entry.file_name().into_string().unwrap(),
			is_dir: entry.file_type()?.is_dir(),

			num_links: metadata.nlink(),
			size: metadata.len(),
			modified: DateTime::from(metadata.modified()?),

			user_name: get_user_name(uid).unwrap(),
			group_name: get_group_name(gid).unwrap(),

			permisions: Section::new(mode as u16),
		})
	}

	pub fn is_hidden(&self) -> bool {
		self.name.chars().nth(0).unwrap() == '.'
	}

	pub fn is_dir(&self) -> bool {
		self.is_dir
	}

	pub fn get_name(&self) -> String {
		format!("{}", self.name)
	}

	pub fn size(&self) -> u64 {
		self.size
	}

	pub fn modified(&self) -> DateTime<Local> {
		self.modified
	}
}

// traits impl
impl fmt::Display for File {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dir_mark = if self.is_dir { DIR_MARK } else { BLK_MARK };
		
		write!(f, "{}{} {} {} {} {:>5} {} {}\n",
			dir_mark,
			self.permisions,
			self.num_links,
			self.user_name,
			self.group_name,
			self.size,
			self.modified.format(TIME_FORMAT),
			self.name
		)
    }
}