use std::fs;
use std::path::Path;
use std::os::unix::fs::symlink;
use std::fs::remove_file;

macro_rules! base_path { () => ( "/home/kaj/school/" ) }
macro_rules! semester_path { () => ( concat!(base_path!(), "semester") ) }
macro_rules! course_path { () => ( concat!(base_path!(), "course") ) }

struct Course {
    name: String,
    path: String
}

fn read_courses() -> Vec<Course> {
    let mut courses = Vec::new();
    match fs::read_dir(semester_path!()) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let name: String = entry.file_name().to_str().expect("course name could not be read").to_string();
                        let path: String = entry.path().to_str().expect("course path could not be read").to_string();

                        if name.starts_with(".") || entry.path().is_symlink() {
                            continue;
                        }
                        
                        courses.push(Course { name, path });
                    },
                    Err(_error) => println!("file not found"),
                }
            }
        }
        Err(_error) => println!("{} does not exist", base_path!()),
    }

    return courses;
}

pub fn command(args: Vec<String>) {
    let courses: Vec<Course> = read_courses();

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
