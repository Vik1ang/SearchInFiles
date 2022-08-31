use crate::adapter::{print_output, SearchIn};
use crate::file::FileMata;
use dotext::{Docx, MsDoc};
use std::io::Read;
use std::path::PathBuf;

pub struct WordFileType {
    file_meta: FileMata,
    pattern: String,
}

impl WordFileType {
    pub fn new(file_meta: FileMata, pattern: String) -> Self {
        WordFileType { file_meta, pattern }
    }
}

impl SearchIn for WordFileType {
    fn search_in(&self) {
        let file = Docx::open(&self.file_meta.path);
        if let Err(_) = file {
            return;
        }
        let mut file = file.unwrap();
        let mut file_content = String::new();
        file.read_to_string(&mut file_content)
            .expect("fail to read string");
        print_output(
            file_content,
            String::from(&self.pattern),
            PathBuf::from(&self.file_meta.path),
        )
    }
}
