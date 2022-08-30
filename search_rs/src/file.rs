use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct FileMata {
    pub path: PathBuf,
    pub extension: String,
}

impl FileMata {
    pub fn new(path_buf: PathBuf) -> Self {
        let ext = match path_buf.extension() {
            Some(e) => e.to_str().unwrap().to_string(),
            None => "".to_string(),
        };
        FileMata {
            path: path_buf,
            extension: ext,
        }
    }
}
