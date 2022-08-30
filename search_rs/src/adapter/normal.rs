use crate::adapter::{print_output, SearchIn};
use crate::file::FileMata;
use std::fs;
use std::path::PathBuf;

pub struct NormalFileType {
    file_meta: FileMata,
    pattern: String,
}

impl NormalFileType {
    pub fn new(file_meta: FileMata, pattern: String) -> Self {
        NormalFileType { file_meta, pattern }
    }
}

impl SearchIn for NormalFileType {
    fn search_in(&self) {
        let file_content = fs::read_to_string(&self.file_meta.path);
        let file_content = match file_content {
            Ok(s) => s,
            Err(_) => format!(""),
        };

        if file_content.is_empty() {
            return;
        }

        print_output(
            file_content,
            String::from(&self.pattern),
            PathBuf::from(&self.file_meta.path),
        )
    }
}
