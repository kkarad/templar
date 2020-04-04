use std::env;
use std::process;
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
    wip_jiras: Vec<String>
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let home_dir = find_long_option_value(&args, "--home").unwrap_or_else(|err| {
        error(&err);
        process::exit(1);
    });
    conf::init(home_dir.map(|p| PathBuf::from(p)).or(dirs::home_dir()));
    
    if args.len() <= 1 {
        return ;
    } else if let Some(_) = args.iter().find(|&x| x.eq("-h") || x.eq("--help")) {
        println!("{}", usage());
        return;
    } else if let Some(_) = args.iter().find(|&x| x.eq("--version")) {
        println!("Templar version: {}", VERSION);
        return;
    }

    args.remove(0);
    let is_release = !args[0].starts_with('-');
    if !is_release {
        error(&format!("Found unknown argument '{}'", args[0]));
        process::exit(1);
    }
    
    let release_name = args[0].to_owned();
    let current_version = find_option_value(&args, "-c", "--current").unwrap_or_else(|err| {
        error(&err);
        process::exit(1);
    });
    let next_version = find_option_value(&args, "-n", "--next").unwrap_or_else(|err| {
        error(&err);
        process::exit(1);
    });
    let tweet = find_option_value(&args, "-t", "--tweet").unwrap_or_else(|err| {
        error(&err);
        process::exit(1);
    });
    let pvt_line_range = find_option_value(&args, "-p", "--pvt-line-range").unwrap_or_else(|err| {
        error(&err);
        process::exit(1);
    });
    let jiras = find_option_values(&args, "-j", "--jiras").unwrap_or_else(|err| {
        error(&err);
        process::exit(1);
    });
    let wip_jiras = find_option_values(&args, "-w", "--wip-jiras").unwrap_or_else(|err| {
        error(&err);
        process::exit(1);
    });
    let release = Release {
            name: release_name
            , current_version: current_version.unwrap_or("1".to_string())
            , next_version: next_version.unwrap_or("2".to_string())
            , tweet: tweet.unwrap_or("default tweet".to_string())
            , pvt_line_range: pvt_line_range.unwrap_or("10-20".to_string())
            , jiras: jiras.unwrap_or(vec![])
            , wip_jiras: wip_jiras.unwrap_or(vec![])
    };
    let is_parse = args.iter().find(|&x| x.eq("--parse")).is_some();
    if is_parse {
        println!("{:?}", release);
    }
}

fn error(msg: &str) {
    eprintln!("error: {}", msg);
    eprintln!("For more information try --help");
}

fn find_long_option_value(args: &Vec<String>, long: &str) -> Result<Option<String>, String> {
    return find_option_value(args, "", long);
}

fn find_option_value(args: &Vec<String>, short:  &str, long: &str) -> Result<Option<String>, String> {
    let mut iter = args.iter();
    if let Some(option) = iter.find(|&x| (short != "" && x.eq(short)) || x.eq(long)) {
        let value_token = iter.next().filter(|v| !v.starts_with('-'));
        return match value_token { 
            Some(v) => Ok(Some(v.to_string())),
            None => Err(format!("Missing option value for: {}", option)),
        };
    } else{
        return Ok(None)
    }
}

fn find_option_values(args: &Vec<String>, short:  &str, long: &str) -> Result<Option<Vec<String>>, String> {
    let mut iter = args.iter();
    if let Some(option) = iter.find(|&x| x.eq(short) || x.eq(long)) {
        let mut values: Vec<String> = vec![];
        while let Some(value) = iter.next() {
            if !value.starts_with("-") {
                values.push(value.to_string());
            }
        }
        return match values[..] { 
            [] => Err(format!("Missing option value(s) for: {}", option)),
            _ => Ok(Some(values)),
        };
    } else {
        return Ok(None)
    }
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
