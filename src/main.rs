use structopt::StructOpt;
use std::io::Result;
use std::path::PathBuf;

mod dots;

use dots::Dots;

#[derive(StructOpt, Debug)]
#[structopt(name = "dots", about = "A dotfiles manager")]
struct Opt {
    #[structopt(short, long, parse(from_os_str), default_value = "./.dots.toml")]
    config: PathBuf,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Initialize your dotfiles
    Init,
    /// Install your dotfiles
    Install,
    /// Update your dotfiles
    Update,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let dots = Dots::from_config(&opt.config)?;

    match opt.cmd {
        Command::Init => dots.init(),
        Command::Install => dots.install(),
        Command::Update => dots.update(),
    };

    Ok(())
}
