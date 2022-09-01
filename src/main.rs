use color_eyre::eyre::*;
use directories::UserDirs;
use lib_garden::write::write;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "garden")]
struct Opt {
    #[structopt(parse(from_os_str, short = "p", long, env))]
    garden_path: Option<PathBuf>,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    Write {
        #[structopt(short, long)]
        title: Option<String>,
    },
}

fn get_default_dir() -> Result<PathBuf> {
    let user_dirs = UserDirs::new().ok_or_else(|| eyre!("Could not find home director"))?;
    Ok(user_dirs.home_dir().join(".garden"))
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let opt = Opt::from_args();
    // debug
    dbg!(&opt);

    let garden_path = match opt.garden_path {
        Some(pathbuf) => Ok(pathbuf),
        None => get_default_dir().wrap_err("`garden_path` was not supplied"),
    };

    match opt.cmd {
        Command::Write { title } => write(title),
    }
}
