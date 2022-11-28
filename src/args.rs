
use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    // target path
    #[clap(value_parser, default_value = ".")]  
    pub path: String,

    // filters
    #[clap(short, action, conflicts_with = "all")]
    pub dirs: bool,

    #[clap(short, action, conflicts_with = "dirs")]
    pub all: bool,

    // sorts by
    #[clap(short, action, conflicts_with = "size")]
    pub time: bool,

    #[clap(short, action, conflicts_with = "time" )]
    pub size: bool,

    // format
    #[clap(short, action)]
    pub long: bool,
}