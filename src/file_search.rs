use anyhow::Error;
use regex::Regex;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct Match {
    line_num: i32,
    line: String,
    regex: String,
    file_name: PathBuf,
}

pub fn search_dir<P, Func, ErrFunc>(
    path: P,
    regex: &Regex,
    function: &Func,
    error_function: &ErrFunc,
) -> Result<(), Error>
where
    P: AsRef<Path>,
    Func: Fn(&Path, Vec<Match>),
    ErrFunc: Fn(&Path, Error),
{
    let path = path.as_ref();
    // Get data from path
    let meta_data = fs::metadata(path)?;
    Ok(())
}
