use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use crate::utils;

fn make_dummy(path: &Path) {
    let provider = String::from("provider \"aws\" {}");

    let mut file = match File::create(&path) {
        Err(why) => panic!("Could not create dummy.tf for init: {}", why),
        Ok(file) => file,
    };

    match file.write_all(provider.as_bytes()) {
        Err(why) => panic!("Could not write to dummy.tf for init: {}", why),
        Ok(_) => (),
    }
}

pub fn init() {
    utils::make_stage_dir();

    let mut stage_path = utils::stage_dir_path();

    stage_path.push(".terraform");
    let path = stage_path.as_path();
    if path.exists() {
        return;
    }

    println!("Fetching Terraform AWS plugin... ");

    stage_path.pop();
    stage_path.push("variables.tf");
    let path = stage_path.as_path();
    let made_dummy = if path.exists() {
        false
    } else {
        stage_path.pop();
        stage_path.push("dummy.tf");
        let path = stage_path.as_path();
        make_dummy(path);
        true
    };

    stage_path.pop();
    let path = stage_path.as_path();
    match Command::new("terraform")
        .arg("init")
        .current_dir(path)
        .output()
    {
        Err(why) => panic!("Cannot init: {}", why),
        Ok(_) => (),
    };

    if made_dummy {
        stage_path.push("dummy.tf");
        let path = stage_path.as_path();
        fs::remove_file(path).unwrap();
    }

    println!("Done!");
}
