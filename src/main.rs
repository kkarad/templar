use std::env;
use std::path::PathBuf;

extern crate dirs;

mod option;
mod usage;
mod conf;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
struct Release {
    name: String,
    current_version: String,
    next_version: String,
    tweet: String,
    pvt_line_range: String,
    jiras: Vec<String>,
    wip_jiras: Vec<String>,
}

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

    let home_dir = option::find_long_value(&mut general_args, "--home")?;
    conf::init(home_dir.map(|p| PathBuf::from(p)).or(dirs::home_dir()));

    if option::find(&mut general_args, "-h", "--help")? {
        println!("{}", usage::main());
        return Ok(());
    } else if option::find_long(&mut general_args, "--version")? {
        println!("Templar version: {}", VERSION);
        return Ok(());
    }

    let command = command_args.first().ok_or("No command specified")?.to_owned();
    command_args.remove(0);
    return if command.eq(&"release".to_string()) {
        handle_release(&mut command_args.to_vec())
    } else {
        Err(format!("Unknown command '{}'", command))
    };
}

fn handle_release(args: &mut Vec<String>) -> Result<(), String> {
    if option::find(args, "-h", "--help")? {
        println!("{}", usage::release());
        return Ok(());
    }
    let release_name = args.first().ok_or("Release name is missing")?.to_owned();
    let current_version = option::find_value(args, "-c", "--current")?;
    let next_version = option::find_value(args, "-n", "--next")?;
    let tweet = option::find_value(args, "-t", "--tweet")?;
    let pvt_line_range = option::find_value(args, "-p", "--pvt-line-range")?;
    let jiras: Option<Vec<String>> = option::find_values(args, "-j", "--jiras")?;
    let wip_jiras: Option<Vec<String>> = option::find_values(args, "-w", "--wip-jiras")?;
    let release = Release {
        name: release_name,
        current_version: current_version.unwrap_or("1".to_string()),
        next_version: next_version.unwrap_or("2".to_string()),
        tweet: tweet.unwrap_or("default tweet".to_string()),
        pvt_line_range: pvt_line_range.unwrap_or("10-20".to_string()),
        jiras: jiras.unwrap_or(vec![]),
        wip_jiras: wip_jiras.unwrap_or(vec![]),
    };
    if option::find_long(args, "--parse")? {
        println!("{:?}", release);
    }
    Ok(())
}
