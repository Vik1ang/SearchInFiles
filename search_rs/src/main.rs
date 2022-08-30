use anyhow::{bail, Context, Result};
use clap::Parser;
use std::path::PathBuf;
use walkdir::WalkDir;

use is_executable::is_executable;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, value_parser)]
    pattern: String,
    #[clap(long, parse(from_os_str))]
    path: Option<PathBuf>,
}

const SEARCH_FILE_EXTENSION: Vec<String> = vec![];

fn main() -> Result<()> {
    let args = Args::parse();
    let path = match &args.path {
        Some(p) => PathBuf::from(&p),
        None => PathBuf::from("."),
    };
    let pattern: String = args.pattern;

    // let content = std::fs::read_to_string(&path)
    //     .with_context(|| format!("could not read file `{:?}`", path))?;

    let mut file_lists = Vec::new();
    if path.is_dir() {
        get_folder_files(&path, &mut file_lists);
    } else if path.is_file() {
        file_lists.push(path);
    } else {
        bail!("Error location")
    }

    // for line in content.lines() {
    //     if line.contains(&pattern) {
    //         println!("{}", line);
    //     }
    // }
    let lst_size = file_lists.len();
    let pb = indicatif::ProgressBar::new(lst_size as u64);
    for i in 0..lst_size {
        let content = std::fs::read_to_string(&file_lists[i]);
        let content = match content {
            Ok(s) => s,
            Err(_) => format!(""),
        };
        for line in content.lines() {
            if line.contains(&pattern) {
                println!("file: [{:?}], content: {}", &file_lists[i], line);
            }
        }
        // thread::sleep(Duration::from_secs(1));
        // pb.println(format!("[+] finished #{}", i));
        // pb.inc(1);
    }
    pb.finish_with_message("done");

    Ok(())
}

// fn validate_pattern(pattern: Option<String>) -> Result<String, String> {
//     match pattern {
//         Some(p) => Ok(p.to_string()),
//         None => Err(format!("Missing attribute: pattern"))
//     }
// }

fn get_folder_files(path: &PathBuf, lst: &mut Vec<PathBuf>) {
    for entry in WalkDir::new(path.to_str().unwrap())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if !is_executable(entry.path()) {
            lst.push(entry.into_path());
        }
    }
}
