mod info;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
name = "pr",
about = "Rust CLI to convert image"
)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
#[structopt(setting = structopt::clap::AppSettings::SubcommandRequired)]
struct Opts {
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    debug: bool,

    #[structopt(subcommand)]
    cmd: Tool,
}

#[derive(Debug, StructOpt)]
enum Tool {
    #[structopt(name = "info")]
    Info(info::Command),
}
#[tokio::main]
async fn main() {
    let args = Opts::from_args();
    match args.cmd {
        Tool::Info(args) => {
            println!("{:?}", args)
            // info::test(args.file)
        }
    }
    // let opts = Opts::from_args();
    // let args = Tool::from_args();
    // println!("{:?}", opts);
    // println!("{:?}", tools);
}