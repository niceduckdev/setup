use std::fs;

pub struct File {
    pub name: String,
    pub path: String
}

pub fn read_files(dir: String) -> Vec<File> {
    let mut files = Vec::new();
    match fs::read_dir(&dir) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let name: String = entry.file_name().to_str().expect("file name could not be read").to_string();
                        let path: String = entry.path().to_str().expect("file path could not be read").to_string();

                        if name.starts_with(".") || entry.path().is_symlink() {
                            continue;
                        }
                        
                        files.push(File { name, path });
                    },
                    Err(_error) => println!("file not found"),
                }
            }
        }
        Err(_error) => println!("{} does not exist", dir),
    }

    return files;
}
