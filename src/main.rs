extern crate dirs;
mod option;
mod usage;
mod conf;
mod release;

use std::env;
use std::path::PathBuf;
use conf::Conf;
pub use release::Context;
use crate::release::{Console, Output};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    std::process::exit(match run() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {}", err);
            eprintln!("For more information try --help");
            1
        }
    });
}

fn run() -> Result<(), String> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0); //remove executable name
    let commands = vec!["release".to_string()];
    let (mut general_args, mut command_args) = option::split_args(args, commands);

    let home_dir = option::find_long_value(&mut general_args, "--home")?
        .map(PathBuf::from).or_else(dirs::home_dir).ok_or("Unable to locate home directory")?;
    let conf = conf::init(home_dir)?;

    if option::find(&mut general_args, "-h", "--help")? {
        println!("{}", usage::main());
        return Ok(());
    } else if option::find_long(&mut general_args, "--version")? {
        println!("Templar version: {}", VERSION);
        return Ok(());
    }

    let command = command_args.first().ok_or("No command specified")?.to_owned();
    command_args.remove(0);
    if command.eq(&"release".to_string()) {
        handle_release(conf, &mut command_args.to_vec())
    } else {
        Err(format!("Unknown command '{}'", command))
    }
}

fn handle_release(conf: Conf, args: &mut Vec<String>) -> Result<(), String> {
    if option::find(args, "-h", "--help")? {
        println!("{}", usage::release());
        return Ok(());
    }
    let release_name = args.first().ok_or("Release name is missing")?.to_owned();
    let release = conf.release(&release_name).ok_or(format!("Unknown release: {}", release_name))?;
    let current_version = option::find_value(args, "-c", "--current")?;
    let next_version = option::find_value(args, "-n", "--next")?;
    let tweet = option::find_value(args, "-t", "--tweet")?;
    let pvt_line_range = option::find_value(args, "-p", "--pvt-line-range")?;
    let jiras: Option<Vec<String>> = option::find_values(args, "-j", "--jiras")?;
    let wip_jiras: Option<Vec<String>> = option::find_values(args, "-w", "--wip-jiras")?;
    let context = Context::new(
        release_name,
        current_version.unwrap_or_else(|| "1".to_string()),
        next_version.unwrap_or_else(|| "2".to_string()),
        tweet.unwrap_or_else(|| "default tweet".to_string()),
        pvt_line_range.unwrap_or_else(|| "10-20".to_string()),
        jiras.unwrap_or_else(|| vec![]),
        wip_jiras.unwrap_or_else(|| vec![]),
    );
    if option::find_long(args, "--parse")? {
        println!("{:?}", context);
        return Ok(());
    }
    Console::new().print(release.templates(), context)
}
