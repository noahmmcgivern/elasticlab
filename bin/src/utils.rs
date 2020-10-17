use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

use dirs;

use crate::key;

pub fn read_path_to_string(pathbuf: &PathBuf) -> String {
    let path = pathbuf.as_path();
    let path_str = pathbuf.to_str().unwrap();

    match File::open(path) {
        Err(why) => match why.kind() {
            ErrorKind::NotFound => panic!("Could not find {}", path_str),
            _ => panic!("Could not open {}: {}", path_str, why)
        },
        Ok(mut file) => {
            let mut s = String::new();
            match file.read_to_string(&mut s) {
                Err(why) => panic!("Could not read {}: {}", path_str, why),
                Ok(_) => s,
            }
        },
    }
}

pub fn read_string_from_secret_file() -> String {
    let path = secret_file_path();
    let path_str = path.to_str().unwrap();

    match File::open(&path) {
        Err(why) => match why.kind() {
            ErrorKind::NotFound => {
                key::key();
                read_string_from_secret_file()
            },
            _ => panic!("Could not open {}: {}", path_str, why)
        },
        Ok(mut file) => {
            let mut s = String::new();
            match file.read_to_string(&mut s) {
                Err(why) => panic!("Could not read {}: {}", path_str, why),
                Ok(_) => s,
            }
        },
    }
}

pub fn write_string_to_stage_file(file_name: String, contents: String) {
    let mut path = stage_dir_path();
    path.push(&file_name);

    let mut file = match File::create(&path) {
        Err(why) => panic!("Could not create {}: {}", &file_name, why),
        Ok(file) => file,
    };

    match file.write_all(contents.as_bytes()) {
        Err(why) => panic!("Could not write to {}: {}", &file_name, why),
        Ok(_) => (),
    }
}

pub fn make_stage_dir() {
    let path = stage_dir_path();

    make_dir(path);
}

pub fn make_work_dir() {
    let path = work_dir_path();

    make_dir(path);
}

fn make_dir(path: PathBuf) {
    match fs::create_dir_all(path) {
        Err(why) => match why.kind() {
            ErrorKind::AlreadyExists => (),
            _ => panic!("Could not create directory: {}", why),
        },
        Ok(_) => (),
    }
}

pub fn infra_dir_path() -> PathBuf {
    dir_path("infra")
}

pub fn lock_file_path() -> PathBuf {
    dir_path("lock")
}

pub fn secret_file_path() -> PathBuf {
    dir_path("secret")
}

pub fn stage_dir_path() -> PathBuf {
    dir_path("stage")
}

fn dir_path(s: &str) -> PathBuf {
    let mut path = work_dir_path();
    path.push(s);
    path
}

fn work_dir_path() -> PathBuf {
    match dirs::home_dir() {
        Some(home) => {
            let mut p = PathBuf::new();
            p.push(home);
            p.push(".elasticlab");
            p
        },
        None => panic!("Cannot get home directory")
    }
}
