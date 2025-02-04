use std::env;

mod courses;
mod semesters;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        return;
    }

    if args[1] == "courses" {
        courses::command(args);
        return;
    }
    else if args[1] == "semesters" {
        semesters::command(args);
        return;
    }
    else if args[1] == "current" {
        let semester = semesters::get_current();
        let course = courses::get_current();
        println!("{}/{}", semester.unwrap(), course.unwrap());
        return;
    }
}
