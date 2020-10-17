use std::fs::File;
use std::io::Write;

use rpassword;

use crate::utils;

fn get_credentials() -> (String, String) {
    let access_key = rpassword::read_password_from_tty(Some("Access Key: ")).unwrap();
    let secret_key = rpassword::read_password_from_tty(Some("Secret Key: ")).unwrap();

    (access_key, secret_key)
}

fn set_credentials(access_key: &str, secret_key: &str) {
    let mut path = utils::infra_dir_path();

    if !path.exists() {
        println!("Could not find infrastructure folder (infra)");
        return;
    }

    path.push("secret.tfvars.template");

    let secret_contents = utils::read_path_to_string(&path);
    let secret_contents = secret_contents.replace("AK", &access_key);
    let secret_contents = secret_contents.replace("SK", &secret_key);

    let path = utils::secret_file_path();

    let mut secret_file = match File::create(path) {
        Err(why) => panic!("Could not create secret.tfvars: {}", why),
        Ok(file) => file,
    };

    match secret_file.write_all(secret_contents.as_bytes()) {
        Err(why) => panic!("Could not write to secret.tfvars: {}", why),
        Ok(_) => (),
    }
}

pub fn key() {
    let (access_key, secret_key) = get_credentials();
    set_credentials(&access_key, &secret_key)
}
