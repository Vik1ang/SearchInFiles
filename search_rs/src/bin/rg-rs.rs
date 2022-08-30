use std::fs;
use clap::Parser;
use colored::Colorize;
use indicatif::ProgressStyle;
use search_rs::file::FileMata;
use std::path::PathBuf;

use search_rs::utils::get_folder_files;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, value_parser)]
    pattern: String,
    #[clap(long, parse(from_os_str))]
    path: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // set current folder default if user does not input
    let path = match &args.path {
        Some(p) => PathBuf::from(&p),
        None => PathBuf::from("."),
    };
    let pattern: String = args.pattern;

    let mut file_lists = Vec::new();

    if path.is_dir() {
        get_folder_files(&path, &mut file_lists);
    } else {
        file_lists.push(FileMata::new(path));
    }

    let bar = indicatif::ProgressBar::new(file_lists.len() as u64);
    bar.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
            .unwrap()
            .progress_chars("##-"),
    );

    for file in file_lists {
        let file_content = std::fs::read_to_string(&file.path);
        let file_content = match file_content {
            Ok(s) => s,
            Err(_) => format!(""),
        };

        if file_content.is_empty() {
            continue;
        }
        let mut is_first = true;
        for line_tup in file_content.lines().enumerate() {
            let index = line_tup.0 + 1;
            let line = line_tup.1;
            if line.contains(&pattern) {
                if is_first {
                    let file_name = fs::canonicalize(&file.path);
                    let file_name = file_name.unwrap().to_str().unwrap().to_string();
                    println!("{}", file_name.bold().blue().underline());
                    is_first = false;
                }
                let output =
                    line.replace(&pattern, &*format!("{}", &pattern.on_bright_red().bold()));
                println!("{}\t{}", format!("{}", index.to_string().cyan()), output);
            }
        }
        bar.inc(1);
    }
    bar.finish();

    Ok(())
}
