//Usage format based on git
pub fn main() -> String {
    let usage = r#"Templar. The template release command line tool

Usage: templar [-h | --help] [--version] [--home <path>] <command> <args>

Options:
    -h, --help      Show this screen
    --version       Show version
    --home <path>   Override user's home directory (where '.templar' configuration resides)
"#;
    usage.to_string()
}

pub fn release() -> String {
    let usage = r#"
Usage: templar release [-h | --help] <name> [-c | --current <version>] [-n | --next <version>]
               [-t | --tweet <description>] [-p | --pvt-line-range <range>] [-j | --jiras <jira...>]
               [-w | --wip-jiras <jira...>] [--parse] [<path>]

Options:
    -h, --help                      Show this screen
    --home <path>                   Override user's home directory (where '.templar' configuration resides)
    -c, --current <version>         Current release version [default: 1]
    -n, --next <version>            Next release version [default: 2]
    -t, --tweet <description>       Release short description [default: default tweet]
    -p, --pvt-line-range <range>    The PVT line range [default: 10-20]
    -j, --jiras <jira...>           The jiras released [default: ]
    -w, --wip-jiras <jira...>       The work in progress jiras in the released [default: ]
    --parse                         Parses release options and prints them without creating the release
"#;
    usage.to_string()
}