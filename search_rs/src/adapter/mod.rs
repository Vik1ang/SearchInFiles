use colored::Colorize;
use std::fs;
use std::path::PathBuf;

pub mod normal;
pub mod pdf;
pub mod excel;

pub trait SearchIn {
    fn search_in(&self);
}

fn print_output(file_content: String, pattern: String, path: PathBuf) {
    let mut is_first = true;
    for line_tup in file_content.lines().enumerate() {
        let index = line_tup.0 + 1;
        let line = line_tup.1;
        if line.contains(&pattern) {
            if is_first {
                let file_name = fs::canonicalize(&path);
                let file_name = file_name.unwrap().to_str().unwrap().to_string();
                println!("{}", file_name.bold().blue().underline());
                is_first = false;
            }
            let output = line.replace(&pattern, &*format!("{}", &pattern.on_bright_red().bold()));
            println!("{}\t{}", format!("{}", index.to_string().cyan()), output);
        }
    }
}
