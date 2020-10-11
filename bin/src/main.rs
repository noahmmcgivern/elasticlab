static LOCKFILE_NAME: &str = "lock";
static INFRA_NAME: &str = "infra/";
static STAGE_NAME: &str = "stage/";

use std::fs::File;
use std::fs;
use std::io::ErrorKind;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

use rpassword;
use serde::{Serialize, Deserialize};
use structopt::StructOpt;

#[derive(StructOpt)]
enum Cmd {
    #[structopt(
        about = "Destroy cloud infrastructure without warning"
    )]
    Des,
    #[structopt(
        about = "Show lockfile [DEFAULT] / stage / cloud"
    )]
    Get {
        #[structopt(
            about = "Show the specified location",
            subcommand
        )]
        location: Option<GetOption>,
    },
    #[structopt(
        about = "Apply stage to cloud"
    )]
    Go,
    #[structopt(
        about = "Apply lockfile to stage"
    )]
    Imp,
    #[structopt(
        about = "Set AWS credentials (Access Key / Secret Key)"
    )]
    Key,
    #[structopt(
        about = "Set infrastructure count / options"
    )]
    Set(Inf),
}

#[derive(StructOpt)]
enum GetOption {
    #[structopt(
        about = "Show lockfile [DEFAULT]"
    )]
    Lockf,
    #[structopt(
        about = "Show stage (Terraform files)"
    )]
    Stage,
    #[structopt(
        about = "Show cloud (`terraform show`)"
    )]
    Cloud
}

#[derive(StructOpt)]
enum Inf {
    #[structopt(
        about = "DynamoDB (NoSQL)",
        name = "dynamodb"
    )]
    DynamoDB {
        #[structopt(
            help = "Number of tables"
        )]
        n: u8,
        #[structopt(
            help = "Set DynamoDB options",
            subcommand
        )]
        option: Option<OptionDynamoDB>,
    },
    #[structopt(
        about = "Simple Storage Service (Bucket)"
    )]
    S3 {
        #[structopt(
            help = "Number of buckets"
        )]
        n: u8,
    },
}

#[derive(StructOpt)]
enum OptionDynamoDB {
    #[structopt(
        about = "Set read capacity for this table"
    )]
    ReadCapacity {
        n: u8,
    },
    #[structopt(
        about = "Set write capacity for this table"
    )]
    WriteCapacity {
        n: u8,
    }
}

//
//

// Lockfile structures

#[derive(Deserialize, Serialize, Debug)]
struct DynamoDB {
    name: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct S3 {
    bucket: String,
}

#[derive(Deserialize, Serialize, Default)]
struct Lockfile {
    dynamodb: Vec<DynamoDB>,
    s3: Vec<S3>,
}

impl std::fmt::Display for Lockfile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = |s: &str| format!("{}\n{}\n", s, "=".repeat(s.len()));
        let mut has_contents = false;

        if self.dynamodb.len() > 0 {
            write!(f, "{}", s("DynamoDB Tables")).unwrap();
            write!(f, "\n").unwrap();

            for i in self.dynamodb.iter() {
                write!(f, "{}\n", i.name).unwrap();
            }

            write!(f, "\n").unwrap();
            has_contents = true;
        }

        if self.s3.len() > 0 {
            write!(f, "{}", s("S3 Buckets")).unwrap();
            write!(f, "\n").unwrap();

            for i in self.s3.iter() {
                write!(f, "{}\n", i.bucket).unwrap();
            }

            write!(f, "\n").unwrap();
            has_contents = true;
        }

        if !has_contents {
            write!(f, "No state.\n").unwrap();
        }

        Ok(())
    }
}

//
//

// Commands

fn destroy() {
    init();

    Command::new("terraform")
            .arg("destroy")
            .arg("-auto-approve")
            .arg("-var-file=./secret.tfvars")
            .current_dir(STAGE_NAME)
            .status()
            .unwrap();
}

fn get() -> Lockfile {
    let lockfile_path = Path::new(LOCKFILE_NAME);

    match File::open(&lockfile_path) {
        Err(why) => match why.kind() {
            ErrorKind::NotFound => {
                make_lockfile(lockfile_path);
                get()
            },
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
        },
    }
}

fn get_cloud() -> String {
    init();

    match Command::new("terraform")
                  .arg("show")
                  .current_dir(STAGE_NAME)
                  .output() {
                      Err(why) => panic!("Cannot show cloud: {}", why),
                      Ok(s) => {
                        let o = std::str::from_utf8(&s.stdout).unwrap();
                        String::from(o)
                    },
                  }
}

fn get_stage() -> String {
    make_stagedir();

    let paths = fs::read_dir(STAGE_NAME).unwrap();

    let mut paths: Vec<String> = paths
    .map(|x| format!("{}\n", x.unwrap().file_name().to_str().unwrap()))
    .filter(|x| x.ends_with(".tf\n"))
    .filter(|x| !x.contains("variables.tf\n"))
    .collect();

    paths.sort();

    let mut s = String::new();

    for i in paths.iter() {
        s.push_str(&i)
    }

    if s.is_empty() { String::from("No state.\n") } else { s }
}

fn go() {
    init();

    let s = match Command::new("terraform")
        .arg("plan")
        .arg("-out=./plan")
        .arg("-var-file=./secret.tfvars")
        .current_dir(STAGE_NAME)
        .output() {
            Err(why) => panic!("Cannot plan: {}", why),
            Ok(s) => {
                let o = std::str::from_utf8(&s.stdout).unwrap();
                String::from(o)
            },
        };

    println!("{}", s);

    let s = if s.contains("No changes.") { String::new() } else {
        let count = s.lines().count();
        let remove_margin = 6;

        if count < remove_margin { s } else {
            let iter = s.lines().take(count - remove_margin);

            let mut s = String::new();

            for i in iter {
                s.push_str(&format!("{}\n", i))
            }

            s
        }
    };

    if s.is_empty() {
        println!("No changes");
        return
    }

    println!("{}", &s);

    match Command::new("terraform")
                  .arg("apply")
                  .arg("./plan")
                  .current_dir(STAGE_NAME)
                  .status() {
                    Err(why) => panic!("Cannot apply: {}", why),
                    Ok(_) => (),
                  }
}

fn import() {
    make_stagedir();

    let paths = fs::read_dir(STAGE_NAME).unwrap();

    for path in paths {
        let path = path.unwrap().path();

        if path.is_file() {
            if !path.to_str().unwrap().contains("terraform.tfstate") {
                fs::remove_file(path).unwrap()
            }
        }
    }

    if !(Path::new("infra").exists()) {
        println!("Could not find infrastructure folder (infra)");
        return
    }

    //
    //
    
    // secret.tfvars

    let secret_string = format!("{}secret.tfvars", &INFRA_NAME);
    let secret_contents = read_secret_contents(&secret_string);

    let secret_string = format!("{}secret.tfvars", &STAGE_NAME);
    let secret_path = Path::new(&secret_string);

    let mut secret_file = match File::create(&secret_path) {
        Err(why) => panic!("Could not create secret.tfvars: {}", why),
        Ok(file) => file,
    };

    match secret_file.write_all(secret_contents.as_bytes()) {
        Err(why) => panic!("Could not write to secret.tfvars: {}", why),
        Ok(_) => (),
    }

    //
    //

    // variables.tf

    let variables_string = format!("{}variables.tf", &INFRA_NAME);
    let variables_contents = read_file_to_string(&variables_string);

    let variables_string = format!("{}variables.tf", &STAGE_NAME);
    let variables_path = Path::new(&variables_string);

    let mut variables_file = match File::create(&variables_path) {
        Err(why) => panic!("Could not create variables.tf: {}", why),
        Ok(file) => file,
    };

    match variables_file.write_all(variables_contents.as_bytes()) {
        Err(why) => panic!("Could not write to variables.tf: {}", why),
        Ok(_) => (),
    }

    //
    //

    // resources

    let lockfile = get();

    //
    //

    // DynamoDB

    let dynamodb_string = format!("{}dynamodb.tf", &INFRA_NAME);
    let dynamodb_contents = read_file_to_string(&dynamodb_string);

    for i in lockfile.dynamodb.iter() {
        let n = format!("{}{}.tf", &STAGE_NAME, i.name);
        let p = Path::new(&n);
        
        let mut file = match File::create(&p) {
            Err(why) => panic!("Could not create {}.tf: {}", i.name, why),
            Ok(file) => file,
        };

        let dynamodb_contents = dynamodb_contents.replace("NAME", &i.name);

        match file.write_all(dynamodb_contents.as_bytes()) {
            Err(why) => panic!("Could not write to {}.tf: {}", i.name, why),
            Ok(_) => (),
        }
    }

    //
    //

    // S3

    let s3_string = format!("{}s3.tf", &INFRA_NAME);
    let s3_contents = read_file_to_string(&s3_string);

    for i in lockfile.s3.iter() {
        let n = format!("{}{}.tf", &STAGE_NAME, i.bucket);
        let p = Path::new(&n);
        
        let mut file = match File::create(&p) {
            Err(why) => panic!("Could not create {}.tf: {}", i.bucket, why),
            Ok(file) => file,
        };

        let s3_contents = s3_contents.replace("BUCKET", &i.bucket);

        match file.write_all(s3_contents.as_bytes()) {
            Err(why) => panic!("Could not write to {}.tf: {}", i.bucket, why),
            Ok(_) => (),
        }
    }

    println!("Staged infrastructure from lockfile");
}

fn init() {
    make_stagedir();

    let path = format!("{}{}", STAGE_NAME, ".terraform");

    let path = Path::new(&path);

    if path.exists() {
        return
    }

    println!("Fetching Terraform AWS plugin...");

    let path = format!("{}{}", STAGE_NAME, "dummy.tf");

    let path = Path::new(&path);

    let mut file = match File::create(&path) {
        Err(why) => panic!("Could not create dummy.tf for init: {}", why),
        Ok(file) => file,
    };

    let provider = String::from("provider \"aws\" {}");

    match file.write_all(provider.as_bytes()) {
        Err(why) => panic!("Could not write to dummy.tf for init: {}", why),
        Ok(_) => (),
    }

    let s = match Command::new("terraform")
                  .arg("init")
                  .current_dir(STAGE_NAME)
                  .output() {
                      Err(why) => panic!("Cannot init: {}", why),
                      Ok(_) => (),
                  };

    fs::remove_file(path).unwrap();
    println!("Done!");
}

fn key() {
    let (access_key, secret_key) = get_credentials();
    set_credentials(&access_key, &secret_key)
}

fn set(infrastructure: Inf) {
    let lockfile = get();

    let lockfile = match infrastructure {
        Inf::DynamoDB { n, option } => {
            let m: u16 = n.into();

            let dynamodb: Vec<DynamoDB> = (1..m+1)
            .map(|x| DynamoDB { name: format!("elasticlab-dynamodb-{}", x) })
            .collect();

            Lockfile { dynamodb: dynamodb, ..lockfile }
        },
        Inf::S3 { n } => {
            let m: u16 = n.into();

            let s3: Vec<S3> = (1..m+1)
            .map(|x| S3 { bucket: format!("elasticlab-s3-{}", x) })
            .collect();
            
            Lockfile { s3: s3, ..lockfile }
        },
    };

    let lockfile_path = Path::new(LOCKFILE_NAME);
    
    let mut file = make_lockfile(lockfile_path);

    let serialized = serde_json::to_string(&lockfile).unwrap();

    match file.write_all(serialized.as_bytes()) {
        Err(why) => panic!("Could not write to lockfile: {}", why),
        Ok(_) => println!("Wrote to lockfile"),
    }

    import()
}

//
//

// Utilities

fn get_credentials() -> (String, String) {
    let access_key = rpassword::read_password_from_tty(Some("Access Key: ")).unwrap();
    let secret_key = rpassword::read_password_from_tty(Some("Secret Key: ")).unwrap();

    (access_key, secret_key)
}

fn set_credentials(access_key: &str, secret_key: &str) {
    if !Path::new("infra").exists() {
        println!("Could not find infrastructure folder (infra)");
        return
    }

    let secret_string = format!("{}secret.tfvars.template", &INFRA_NAME);
    let secret_contents = read_file_to_string(&secret_string);

    let secret_string = format!("{}secret.tfvars", &INFRA_NAME);
    let secret_path = Path::new(&secret_string);

    let mut secret_file = match File::create(&secret_path) {
        Err(why) => panic!("Could not create secret.tfvars: {}", why),
        Ok(file) => file,
    };

    let secret_contents = secret_contents.replace("AK", &access_key);
    let secret_contents = secret_contents.replace("SK", &secret_key);

    match secret_file.write_all(secret_contents.as_bytes()) {
        Err(why) => panic!("Could not write to secret.tfvars: {}", why),
        Ok(_) => (),
    }
}

fn make_lockfile(path: &Path) -> File {
    let exists = path.exists();

    let mut f = match File::create(&path) {
        Err(why) => panic!("Could not create lockfile: {}", why),
        Ok(file) => file,
    };

    if exists { f } else {
        let l = Lockfile::default();
        let s = serde_json::to_string(&l).unwrap();

        match f.write_all(s.as_bytes()) {
            Err(why) => panic!("Could not write to lockfile: {}", why),
            Ok(_) => match File::create(&path) {
                Err(why) => panic!("Could not create lockfile: {}", why),
                Ok(file) => file,
            },
        }
    }
}

fn make_stagedir() {
    let stagedir = Path::new(STAGE_NAME);

    match fs::create_dir(stagedir) {
        Err(why) => match why.kind() {
            ErrorKind::AlreadyExists => (),
            _ => panic!("Could not create stage directory: {}", why),
        },
        Ok(_) => (),
    }
}

fn read_file_to_string(path_string: &str) -> String {
    let path = Path::new(path_string);

    match File::open(path) {
        Err(why) => match why.kind() {
            ErrorKind::NotFound => panic!("Could not find {}", path_string),
            _ => panic!("Could not open {}: {}", path_string, why)
        },
        Ok(mut file) => {
            let mut s = String::new();
            match file.read_to_string(&mut s) {
                Err(why) => panic!("Could not read {}: {}", path_string, why),
                Ok(_) => s,
            }
        },
    }
}

fn read_secret_contents(secret_string: &str) -> String {
    let secret_path = Path::new(secret_string);

    match File::open(secret_path) {
        Err(why) => match why.kind() {
            ErrorKind::NotFound => {
                key();
                read_secret_contents(secret_string)
            },
            _ => panic!("Could not open secret.tfvars: {}", why)
        },
        Ok(mut file) => {
            let mut s = String::new();
            match file.read_to_string(&mut s) {
                Err(why) => panic!("Could not read secret.tfvars: {}", why),
                Ok(_) => s,
            }
        },
    }
}

//
//

// main

fn main() {
    let args = Cmd::from_args();

    return match args {
        Cmd::Des => destroy(),
        Cmd::Get { location } => match location {
            Some(x) => match x {
                GetOption::Cloud => print!("{}", get_cloud()),
                GetOption::Lockf => print!("{}", get()),
                GetOption::Stage => print!("{}", get_stage()),
            },
            None => print!("{}", get()),
        },
        Cmd::Go => go(),
        Cmd::Imp => import(),
        Cmd::Key => key(),
        Cmd::Set(infrastructure) => set(infrastructure),
    }
}
