use std::env;
use std::path::PathBuf;

extern crate dirs;

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
    let home_dir = find_long_option_value(&mut args, "--home")?;
    conf::init(home_dir.map(|p| PathBuf::from(p)).or(dirs::home_dir()));

    if args.len() <= 1 {
        return Ok(());
    } else if let Some(_) = args.iter().find(|&x| x.eq("-h") || x.eq("--help")) {
        println!("{}", usage());
        return Ok(());
    } else if let Some(_) = args.iter().find(|&x| x.eq("--version")) {
        println!("Templar version: {}", VERSION);
        return Ok(());
    }

    args.remove(0);
    let is_release = !args[0].starts_with('-');
    if !is_release {
        return Err(format!("Found unknown argument '{}'", args[0]));
    }

    let release_name = args[0].to_owned();
    let current_version = find_option_value(&mut args, "-c", "--current")?;
    let next_version = find_option_value(&mut args, "-n", "--next")?;
    let tweet = find_option_value(&mut args, "-t", "--tweet")?;
    let pvt_line_range = find_option_value(&mut args, "-p", "--pvt-line-range")?;
    let jiras: Option<Vec<String>> = find_option_values(&mut args, "-j", "--jiras")?;
    let wip_jiras: Option<Vec<String>> = find_option_values(&mut args, "-w", "--wip-jiras")?;
    let release = Release {
        name: release_name,
        current_version: current_version.unwrap_or("1".to_string()),
        next_version: next_version.unwrap_or("2".to_string()),
        tweet: tweet.unwrap_or("default tweet".to_string()),
        pvt_line_range: pvt_line_range.unwrap_or("10-20".to_string()),
        jiras: jiras.unwrap_or(vec![]),
        wip_jiras: wip_jiras.unwrap_or(vec![]),
    };
    let is_parse = args.iter().find(|&x| x.eq("--parse")).is_some();
    if is_parse {
        println!("{:?}", release);
    }
    Ok(())
}

fn find_long_option_value(args: &mut Vec<String>, long: &str) -> Result<Option<String>, String> {
    return find_option_value(args, "", long);
}

fn find_option_value(args: &mut Vec<String>, short: &str, long: &str) -> Result<Option<String>, String> {
    let result = find_option_values(args, short, long)?;
    if let Some(values) = result {
        return match values.as_slice() {
            [] => Ok(None),
            [value] => Ok(Some(value.to_string())),
            _ => Err(format!("Found more than one values: {:?}", values)),
        };
    }
    return Ok(None);
}

fn find_option_values(args: &mut Vec<String>, short: &str, long: &str) -> Result<Option<Vec<String>>, String> {
    let mut iter = args.iter();
    let mut value_indices: Vec<usize> = vec![];
    return if let Some(flag_index) = iter.position(|x| (!x.is_empty() && x.eq(short)) || x.eq(long)) {
        let option = args.get(flag_index).unwrap();
        let mut values: Vec<String> = vec![];
        let mut index = flag_index + 1;
        while let Some(value) = args.get(index) {
            if value.starts_with("-") { break; }
            values.push(value.to_owned());
            value_indices.push(index);
            index += 1;
        }
        if values.is_empty() {
            return Err(format!("Missing option value(s) for: {}", option));
        }
        let original_len = args.len();
        args.remove(flag_index);
        for i in value_indices { args.remove(i - (original_len - args.len())); }
        Ok(Some(values))
    } else {
        Ok(None)
    };
}

//Usage format based on: http://docopt.org/
fn usage() -> String {
    let usage = r#"Templar.

Usage:
    templar <name> [-c |-n|-t|-p|-j|-w|--parse]
    templar -h | --help 
    templar --version

Options:
    -h, --help                  Show this screen
    --version                   Show version
    -c, --current CURRENT       Current release version [default: 1]
    -n, --next NEXT             Next release version [default: 2]
    -t, --tweet TWEET           Release short description [default: default tweet]
    -p, --pvt-line-range PVT    The PVT line range [default: 10-20]
    -j, --jiras JIRAS...        The jiras released [default: ]
    -w, --wip-jiras   JIRAS...  The work in progress jiras in the released [default: ]
    --parse                     Parses release options and prints them
    --home                      Override user's home directory (where '.templar' configuration resides)
"#;

    return usage.to_string();
}
