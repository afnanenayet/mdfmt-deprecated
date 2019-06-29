mod config;

use config::Opt;

use structopt::StructOpt;

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
