use crate::adapter::SearchIn;
use crate::file::FileMata;
use colored::Colorize;
use std::fs;

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

        let mut is_first = true;
        for line_tup in file_content.lines().enumerate() {
            let index = line_tup.0 + 1;
            let line = line_tup.1;
            if line.contains(&self.pattern) {
                if is_first {
                    let file_name = fs::canonicalize(&self.file_meta.path);
                    let file_name = file_name.unwrap().to_str().unwrap().to_string();
                    println!("{}", file_name.bold().blue().underline());
                    is_first = false;
                }
                let output = line.replace(
                    &self.pattern,
                    &*format!("{}", &self.pattern.on_bright_red().bold()),
                );
                println!("{}\t{}", format!("{}", index.to_string().cyan()), output);
            }
        }
    }
}
