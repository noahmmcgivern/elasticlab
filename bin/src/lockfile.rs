use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Write;

use crate::utils;

#[derive(Deserialize, Serialize, Debug)]
pub struct DynamoDB {
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EMR {
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct S3 {
    pub name: String,
}

#[derive(Deserialize, Serialize, Default)]
pub struct Lockfile {
    pub dynamodb: Vec<DynamoDB>,
    pub emr: Vec<EMR>,
    pub s3: Vec<S3>,
}

impl std::fmt::Display for Lockfile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = |s: &str| format!("{}\n{}\n", s, "=".repeat(s.len()));
        let mut has_contents = false;

        let mut display = String::new();

        if self.dynamodb.len() > 0 {
            display.push_str(&s("DynamoDB Tables"));
            display.push_str("\n");

            for i in self.dynamodb.iter() {
                display.push_str(&(i.name.to_owned() + "\n"));
            }

            display.push_str("\n");
            has_contents = true;
        }

        if self.emr.len() > 0 {
            display.push_str(&s("EMR Clusters"));
            display.push_str("\n");

            for i in self.emr.iter() {
                display.push_str(&(i.name.to_owned() + "\n"));
            }

            display.push_str("\n");
            has_contents = true;
        }

        if self.s3.len() > 0 {
            display.push_str(&s("S3 Buckets"));
            display.push_str("\n");

            for i in self.s3.iter() {
                display.push_str(&(i.name.to_owned() + "\n"));
            }

            display.push_str("\n");
            has_contents = true;
        }

        if !has_contents {
            display.push_str("No state.\n");
        }

        write!(f, "{}", display)
    }
}

pub fn read_lock_file() -> Lockfile {
    let file = utils::lock_file_path();

    match File::open(&file) {
        Err(why) => match why.kind() {
            ErrorKind::NotFound => {
                write_lock_file();
                read_lock_file()
            }
            _ => {
                panic!("Could not open lockfile: {}", why);
            }
        },
        Ok(mut file) => {
            let mut s = String::new();
            match file.read_to_string(&mut s) {
                Err(why) => panic!("Could not read lockfile: {}", why),
                Ok(_) => match serde_json::from_str::<Lockfile>(&s) {
                    Err(_) => Lockfile::default(),
                    Ok(l) => l,
                },
            }
        }
    }
}

pub fn write_lock_file() -> File {
    utils::make_work_dir();
    let lock_file = utils::lock_file_path();
    let exists = lock_file.exists();

    let create_lock_file = || match File::create(&lock_file) {
        Err(why) => panic!("Could not create lockfile: {}", why),
        Ok(file) => file,
    };

    let mut f = create_lock_file();

    if exists {
        f
    } else {
        let l = Lockfile::default();
        let s = serde_json::to_string(&l).unwrap();

        match f.write_all(s.as_bytes()) {
            Err(why) => panic!("Could not write to lockfile: {}", why),
            Ok(_) => create_lock_file(),
        }
    }
}
