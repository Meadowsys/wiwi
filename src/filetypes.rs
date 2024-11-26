use crate::prelude_std::*;
use std::path::Path;

pub struct FileInfo<'h> {
	// todo change to our own path type when I get that written maybe?
	filename: Option<&'h Path>,
	// todo change to our own path type when I get that written maybe?
	filepath: Option<&'h Path>,
	blob: Option<&'h [u8]>,

	// options
	parse_mode: Option<ParseMode>
}

enum ParseMode {
	MagicNumberOnly,
	FullFile
}
