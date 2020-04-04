REFERENCES

# https://doc.rust-lang.org/stable/book/
# https://doc.rust-lang.org/std/
# https://rust-cli.github.io/book/

TODO

# validate release name from configuration file (https://rust-lang-nursery.github.io/rust-cookbook/encoding/complex.html)
# read default release template from configuration file and print it as json
# apply string interpolation taken from Release struct to release template
# ...
# populate jira from git commits
# populate tweet from jira description
# find current and next version from ssm uat and latest git tag
# find Main class and pvt line range from git repo
# from pvt line range check code, see if the jira is mentioned in the description and warn
# error when unknown flag is encountered

COMPLETED

# test default configuration file: tempgile?
# create default toml configuration file if it doesn't exist
# parse cli option: -w or --wip-jiras
# parse cli option: -j or --jiras
# parse cli option: -p or --pvt-line-range
# parse cli option: -t or --tweet
# parse cli option: -n or --next
# parse cli option: -c or --current
# implement template_name command and print default options
# print usage
# implement help option
# implement version option