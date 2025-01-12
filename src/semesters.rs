use std::fs;
use std::path::Path;
use std::os::unix::fs::symlink;
use std::fs::remove_file;

macro_rules! base_path { () => ( "/home/kaj/school/" ) }
macro_rules! semester_path { () => ( concat!(base_path!(), "semester") ) }

struct Semester {
    name: String,
    path: String
}

fn read_semesters() -> Vec<Semester> {
    let mut semesters = Vec::new();
    match fs::read_dir(base_path!()) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let name: String = entry.file_name().to_str().expect("semester name could not be read").to_string();
                        let path: String = entry.path().to_str().expect("semester path could not be read").to_string();

                        if name.starts_with(".") || entry.path().is_symlink() {
                            continue;
                        }
                        
                        semesters.push(Semester { name, path });
                    },
                    Err(_error) => println!("file not found"),
                }
            }
        }
        Err(_error) => println!("{} does not exist", base_path!()),
    }

    return semesters;
}

pub fn command(args: Vec<String>) {
    let semesters: Vec<Semester> = read_semesters();

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
