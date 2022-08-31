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
        let file_name = file.path.file_name().unwrap().to_str().unwrap();
        if !file_name.starts_with("~") {
            lst.push(file);
        }
    }
}
