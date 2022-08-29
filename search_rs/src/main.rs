use anyhow::{bail, Context, Result};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, value_parser)]
    pattern: String,
    #[clap(long, parse(from_os_str))]
    path: Option<PathBuf>,
}

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
        let content = std::fs::read_to_string(&file_lists[i])
            .with_context(|| format!("could not read file `{:?}`", &file_lists[i]))?;
        for line in content.lines() {
            if line.contains(&pattern) {
                println!("{}", line);
            }
        }
        // thread::sleep(Duration::from_secs(1));
        // pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
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
    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            if path.is_dir() {
                // get_folder_files(path, lst);
            } else if path.is_file() {
                lst.push(entry.path());
            }
            println!("{:?}", entry.path());
        }
    }
}