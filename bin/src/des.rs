use std::process::Command;

use crate::init;
use crate::utils;

pub fn destroy() {
    init::init();

    let file_name = "secret.tfvars";
    let contents = utils::read_string_from_secret_file();
    utils::write_string_to_stage_file(file_name.to_string(), contents);

    let path = utils::stage_dir_path();
    Command::new("terraform")
            .arg("destroy")
            .arg("-auto-approve")
            .arg("-var-file=./secret.tfvars")
            .current_dir(path)
            .status()
            .unwrap();
}
