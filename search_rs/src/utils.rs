use crate::file;
use file::FileMata;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn get_folder_files(path: &PathBuf, lst: &mut Vec<FileMata>) {
    for entry in WalkDir::new(path.to_str().unwrap())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let file = FileMata::new(entry.into_path());
        let file_name = match file.path.file_name() {
            Some(f) => f.to_str().unwrap().to_string(),
            None => "~".to_string(),
        };
        // let file_name = file.path.file_name();
        if !file_name.starts_with("~") {
            lst.push(file);
        }
    }
}
