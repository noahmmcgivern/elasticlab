use std::fs;
use std::process::Command;

use crate::init;
use crate::utils;

pub fn get_aws() -> String {
    init::init();

    let path = utils::stage_dir_path();

    match Command::new("terraform")
                  .arg("show")
                  .current_dir(path)
                  .output() {
                      Err(why) => panic!("Cannot show cloud: {}", why),
                      Ok(s) => {
                          let s = std::str::from_utf8(&s.stdout).unwrap();

                          if s.len() > 13 {
                            String::from(s)
                          } else {
                            String::from("No state.\n")
                          }
                        },
                    }
}

pub fn get_stage() -> String {
    utils::make_stage_dir();

    let path = utils::stage_dir_path();

    let paths = fs::read_dir(path).unwrap();

    let mut paths: Vec<String> = paths
    .map(|x| x.unwrap().file_name().into_string().unwrap() + "\n")
    .filter(|x| x.ends_with(".tf\n"))
    .filter(|x| !x.contains("variables.tf\n"))
    .collect();

    paths.sort();

    let mut s = String::new();

    for i in paths.iter() {
        s.push_str(&i)
    }

    if s.is_empty() {
        String::from("No state.\n")
    } else {
        s
    }
}
