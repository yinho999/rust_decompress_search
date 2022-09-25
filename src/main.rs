use regex::Regex;
use std::path::Path;

pub mod file_search;
pub mod zipping;
fn main() {
    let exit_code = real_main();
    std::process::exit(exit_code)
}

fn real_main() -> i32 {
    // get the file from args
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <file> <search_string>", args[0]);
        return 1;
    }
    let mut fname = Path::new(&args[1]);
    let regex = Regex::new(&args[2]).expect("This is a regex for searching text in file");

    // Unzip the file if the file is a zip file
    if fname.ends_with(".zip") {
        zipping::unzip_file(&fname);
        let names = fname.to_str().unwrap().split(".zip").collect::<Vec<&str>>();
        fname = Path::new(names[0]);
    }

    return 0;
}
