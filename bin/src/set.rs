use std::io::Write;

use crate::lockfile;
use crate::Inf;

pub fn set(infrastructure: Inf, number: u8) {
    let lockfile = lockfile::read_lock_file();
    let number: u16 = number.into();

    let lockfile = match infrastructure {
        Inf::DynamoDB => {
            let dynamodb: Vec<lockfile::DynamoDB> = (1..number+1)
            .map(|x| lockfile::DynamoDB { name: format!("elasticlab-dynamodb-{}", x) })
            .collect();

            lockfile::Lockfile { dynamodb: dynamodb, ..lockfile }
        },
        Inf::EMR => {
            let emr: Vec<lockfile::EMR> = (1..number+1)
            .map(|x| lockfile::EMR { name: format!("elasticlab-emr-{}", x) })
            .collect();

            lockfile::Lockfile { emr: emr, ..lockfile }
        },
        Inf::S3 => {
            let s3: Vec<lockfile::S3> = (1..number+1)
            .map(|x| lockfile::S3 { name: format!("elasticlab-s3-{}", x) })
            .collect();

            lockfile::Lockfile { s3: s3, ..lockfile }
        },
    };

    let mut file = lockfile::write_lock_file();

    let serialized = serde_json::to_string(&lockfile).unwrap();

    match file.write_all(serialized.as_bytes()) {
        Err(why) => panic!("Could not write to lockfile: {}", why),
        Ok(_) => println!("Wrote to lockfile"),
    }
}
