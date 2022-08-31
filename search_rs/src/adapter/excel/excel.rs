use crate::adapter::{print_output, SearchIn};
use crate::file::FileMata;
use calamine::{open_workbook, Reader, Xls, Xlsx};
use std::path::PathBuf;

pub struct ExcelFileType {
    file_meta: FileMata,
    pattern: String,
}

impl ExcelFileType {
    pub fn new(file_meta: FileMata, pattern: String) -> Self {
        ExcelFileType { file_meta, pattern }
    }
}

impl SearchIn for ExcelFileType {
    fn search_in(&self) {
        if self.file_meta.extension == "xls" {
            let t = open_workbook(&self.file_meta.path);
            if let Err(_) = t {
                return;
            }
            let mut wb: Xls<_> = t.unwrap();
            let sheets = wb.worksheets();
            let mut file_content = String::new();
            for sht in sheets {
                for row in sht.1.rows() {
                    for r in row {
                        if !r.is_empty() {
                            file_content = format!("{} {}", file_content, r.to_string());
                        }
                    }
                    file_content += "\r\n";
                }
            }
            print_output(
                file_content,
                String::from(&self.pattern),
                PathBuf::from(&self.file_meta.path),
            )
        } else {
            let mut wb: Xlsx<_> = open_workbook(&self.file_meta.path).unwrap();
            let sheets = wb.worksheets();
            let mut file_content = String::new();
            for sht in sheets {
                for row in sht.1.rows() {
                    for r in row {
                        if !r.is_empty() {
                            file_content = format!("{} {}", file_content, r.to_string());
                        }
                    }
                    file_content += "\r\n";
                }
            }
            print_output(
                file_content,
                String::from(&self.pattern),
                PathBuf::from(&self.file_meta.path),
            )
        }
    }
}
