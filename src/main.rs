mod config;

use config::Opt;
use failure::Error;
use structopt::StructOpt;

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    opt.valid()?;
    println!("{:?}", opt);
    Ok(())
}
