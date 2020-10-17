use std::process::Command;

use crate::init;
use crate::utils;

pub fn go() {
    init::init();

    let file_name = "secret.tfvars";
    let contents = utils::read_string_from_secret_file();
    utils::write_string_to_stage_file(file_name.to_string(), contents);

    let path = &utils::stage_dir_path();
    let s = match Command::new("terraform")
        .arg("plan")
        .arg("-out=./plan")
        .arg("-var-file=./secret.tfvars")
        .current_dir(path)
        .output()
    {
        Err(why) => panic!("Cannot plan: {}", why),
        Ok(s) => {
            let s = std::str::from_utf8(&s.stdout).unwrap();
            String::from(s)
        }
    };

    println!("{}", s);

    let s = if s.contains("No changes.") {
        String::new()
    } else {
        let count = s.lines().count();
        let remove_margin = 6;

        if count < remove_margin {
            s
        } else {
            let iter = s.lines().take(count - remove_margin);

            let mut s = String::new();

            for i in iter {
                s.push_str(&(i.to_owned() + "\n"))
            }

            s
        }
    };

    if s.is_empty() {
        println!("No changes");
        return;
    }

    println!("{}", &s);

    match Command::new("terraform")
        .arg("apply")
        .arg("./plan")
        .current_dir(path)
        .status()
    {
        Err(why) => panic!("Cannot apply: {}", why),
        Ok(_) => (),
    }
}
