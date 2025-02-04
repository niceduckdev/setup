use std::fs;
use std::io;
use std::path::Path;
use std::os::unix::fs::symlink;
use std::fs::remove_file;

use crate::parser;

macro_rules! base_path { () => ( "/home/kaj/school/" ) }
macro_rules! semester_path { () => ( concat!(base_path!(), "semester") ) }

pub fn get_current() -> io::Result<String> {
    if !Path::new(semester_path!()).exists() {
        return Ok("null".to_string());
    }

    let current = fs::read_link(semester_path!())?;
    
    if let Some(name) = current.file_name() {
        return Ok(name.to_string_lossy().to_string());
    }

    return Ok("null".to_string())
}

pub fn command(args: Vec<String>) {
    let semesters: Vec<parser::File> = parser::read_files(base_path!().to_string());

    if args.len() <= 2 {
        for semester in semesters {
            println!("{}", semester.name);
        }

        return;
    }

    for semester in semesters {
        if semester.name != args[2] {
            continue;
        }

        if Path::new(semester_path!()).exists() {
            let _ = remove_file(semester_path!());
        }

        let _ = symlink(semester.path, semester_path!());
    }
}
