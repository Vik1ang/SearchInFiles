use lopdf::Document;
use crate::adapter::SearchIn;
use crate::file::FileMata;

pub struct PdfFileType {
    file_meta: FileMata,
    pattern: String,
}

impl PdfFileType {
    pub fn new(file_meta: FileMata, pattern: String) -> Self {
        PdfFileType { file_meta, pattern }
    }
}

impl SearchIn for PdfFileType {
    fn search_in(&self) {
        todo!()
    }
}