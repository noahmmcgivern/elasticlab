use structopt::clap::arg_enum;
use structopt::StructOpt;

mod des;
mod get;
mod go;
mod imp;
mod init;
mod key;
mod lockfile;
mod opt;
mod set;
mod utils;

#[derive(StructOpt)]
#[structopt(about = "Control AWS Data Analytics Infrastructure.")]
enum Cmd {
    #[structopt(about = "Destroy cloud infrastructure without warning")]
    Des,
    #[structopt(about = "Show lock [DEFAULT] / stage / aws")]
    Get {
        #[structopt(
            help = "Location from which to get information",
            default_value = "Lock",
            possible_values = &Location::variants(),
            case_insensitive = true
        )]
        location: Location,
    },
    #[structopt(about = "Apply stage to cloud")]
    Go,
    #[structopt(about = "Apply lockfile to stage")]
    Imp,
    #[structopt(about = "Set AWS credentials (Access Key / Secret Key)")]
    Key,
    #[structopt(about = "Set infrastructure options")]
    Opt {
        #[structopt(
            help = "Amazon Web Service name",
            possible_values = &Inf::variants(),
            case_insensitive = true
        )]
        infrastructure: Inf,
        #[structopt(help = "Resource number")]
        number: u8,
        #[structopt(
            help = "Option key",
            possible_values = &Opti::variants(),
            case_insensitive = true
        )]
        option: Opti,
        #[structopt(help = "Option value")]
        value: String,
    },
    #[structopt(about = "Set infrastructure count")]
    Set {
        #[structopt(
            help = "Amazon Web Service name",
            possible_values = &Inf::variants(),
            case_insensitive = true
        )]
        infrastructure: Inf,
        #[structopt(help = "Number of resources")]
        number: u8,
    },
}

arg_enum! {
    pub enum Inf {
        DynamoDB,
        EMR,
        S3,
    }
}

arg_enum! {
    enum Location {
        Aws,
        Lock,
        Stage,
    }
}

arg_enum! {
    pub enum Opti {
        ReadCapacity,
        WriteCapacity,
    }
}

fn main() {
    let args = Cmd::from_args();

    return match args {
        Cmd::Des => des::destroy(),
        Cmd::Get { location } => match location {
            Location::Aws => print!("{}", get::get_aws()),
            Location::Lock => print!("{}", lockfile::read_lock_file()),
            Location::Stage => print!("{}", get::get_stage()),
        },
        Cmd::Go => go::go(),
        Cmd::Imp => imp::import(),
        Cmd::Key => key::key(),
        Cmd::Opt {
            infrastructure,
            number,
            option,
            value,
        } => opt::opt(infrastructure, number, option, value),
        Cmd::Set {
            infrastructure,
            number,
        } => {
            set::set(infrastructure, number);
            imp::import();
        }
    };
}
