use std::fs;

use crate::lockfile;
use crate::utils;

fn clear_stage() {
    let stage_path = utils::stage_dir_path();
    let paths = fs::read_dir(&stage_path).unwrap();

    for path in paths {
        let path = path.unwrap().path();

        if path.is_file() {
            if !path.to_str().unwrap().contains("terraform.tfstate") {
                fs::remove_file(path).unwrap()
            }
        }
    }
}

fn read_string_from_infra_file(file_name: &str) -> String {
    let mut path = utils::infra_dir_path();
    path.push(file_name);

    utils::read_path_to_string(&path)
}

pub fn import() {
    utils::make_stage_dir();

    if !utils::infra_dir_path().exists() {
        println!("Could not find infrastructure folder (infra)");
        return;
    }

    clear_stage();

    let file_name = "variables.tf";
    let contents = read_string_from_infra_file(file_name);
    utils::write_string_to_stage_file(file_name.to_string(), contents);

    let lockfile = lockfile::read_lock_file();

    let file_name = "dynamodb.tf";
    let contents = read_string_from_infra_file(file_name);
    for i in lockfile.dynamodb.iter() {
        let name = i.name.to_owned();
        let file_name = name.clone() + ".tf";
        let contents = contents.replace("NAME", &name);

        utils::write_string_to_stage_file(file_name, contents)
    }

    let file_name = "emr.tf";
    let contents = read_string_from_infra_file(file_name);
    for i in lockfile.emr.iter() {
        let name = i.name.to_owned();
        let file_name = name.clone() + ".tf";
        let contents = contents.replace("NAME", &name);

        utils::write_string_to_stage_file(file_name, contents)
    }

    let file_name = "s3.tf";
    let contents = read_string_from_infra_file(file_name);
    for i in lockfile.s3.iter() {
        let name = i.name.to_owned();
        let file_name = name.clone() + ".tf";
        let contents = contents.replace("NAME", &name);

        utils::write_string_to_stage_file(file_name, contents)
    }

    println!("Staged infrastructure from lockfile");
}
