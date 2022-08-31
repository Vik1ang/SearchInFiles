use clap::Parser;
use indicatif::ProgressStyle;
use search_rs::adapter::excel::excel::ExcelFileType;
use search_rs::adapter::normal::normal::NormalFileType;
use search_rs::adapter::pdf::pdf::PdfFileType;
use search_rs::adapter::word::word::WordFileType;
use search_rs::adapter::SearchIn;
use search_rs::file::FileMata;
use search_rs::utils::get_folder_files;
use std::path::PathBuf;

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
        if file.extension == "pdf" {
            let file_type = PdfFileType::new(file, String::from(&pattern));
            file_type.search_in();
        } else if file.extension == "xlsx" || file.extension == "xls" {
            let file_type = ExcelFileType::new(file, String::from(&pattern));
            file_type.search_in();
        } else if ["docx", "doc"].contains(&&file.extension.as_str()) {
            let file_type = WordFileType::new(file, String::from(&pattern));
            file_type.search_in()
        } else {
            let file_type = NormalFileType::new(file, String::from(&pattern));
            file_type.search_in();
        }
        bar.inc(1);
    }
    bar.finish();

    Ok(())
}
