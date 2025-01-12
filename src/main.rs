use std::env;

mod courses;
mod semesters;

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
}
