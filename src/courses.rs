use std::fs;
use std::io;
use std::path::Path;
use std::os::unix::fs::symlink;
use std::fs::remove_file;

use crate::parser;

macro_rules! base_path { () => ( "/home/kaj/school/" ) }
macro_rules! semester_path { () => ( concat!(base_path!(), "semester") ) }
macro_rules! course_path { () => ( concat!(base_path!(), "course") ) }

pub fn get_current() -> io::Result<String> {
    if !Path::new(course_path!()).exists() {
        return Ok("null".to_string());
    }

    let current = fs::read_link(course_path!())?;
    
    if let Some(name) = current.file_name() {
        return Ok(name.to_string_lossy().to_string());
    }

    return Ok("null".to_string())
}

pub fn command(args: Vec<String>) {
    let courses: Vec<parser::File> = parser::read_files(semester_path!().to_string());

    if args.len() <= 2 {
        for course in courses {
            println!("{}", course.name);
        }

        return;
    }

    for course in courses {
        if course.name != args[2] {
            continue;
        }

        if Path::new(course_path!()).exists() {
            let _ = remove_file(course_path!());
        }

        let _ = symlink(course.path, course_path!());
    }
}
