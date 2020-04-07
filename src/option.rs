use crate::option::OptionType::{List, Value, Flag};

pub fn split_args(args: Vec<String>, commands: Vec<String>) -> (Vec<String>, Vec<String>) {
    if let Some(index) = args.iter().position(|item| commands.contains(item)) {
        let (general_args, command_args) = args.split_at(index);
        (general_args.to_vec(), command_args.to_vec())
    } else {
        (args, vec![])
    }
}

#[derive(PartialEq, Eq)]
enum OptionType {
    Flag,
    Value,
    List,
}


pub fn find_long(args: &mut Vec<String>, long: &str) -> Result<bool, String> {
    return find(args, "", long);
}

pub fn find(args: &mut Vec<String>, short: &str, long: &str) -> Result<bool, String> {
    let result = find_option(args, short, long, Flag)?;
    if let Some(values) = result {
        return match values.as_slice() {
            [] => Ok(true),
            [value] => Err(format!("Found value for flag: {}", value)),
            _ => Err(format!("Found values for flag: {:?}", values)),
        };
    }
    return Ok(false);
}

pub fn find_long_value(args: &mut Vec<String>, long: &str) -> Result<Option<String>, String> {
    return find_value(args, "", long);
}

pub fn find_value(args: &mut Vec<String>, short: &str, long: &str) -> Result<Option<String>, String> {
    let result = find_option(args, short, long, Value)?;
    if let Some(values) = result {
        return match values.as_slice() {
            [value] => Ok(Some(value.to_string())),
            [] => Err(format!("Missing value")),
            _ => Err(format!("Found more than one values: {:?}", values)),
        };
    }
    return Ok(None);
}

pub fn find_values(args: &mut Vec<String>, short: &str, long: &str) -> Result<Option<Vec<String>>, String> {
    return find_option(args, short, long, List);
}

fn find_option(args: &mut Vec<String>, short: &str, long: &str, op_type: OptionType) -> Result<Option<Vec<String>>, String> {
    let mut iter = args.iter();
    let mut value_indices: Vec<usize> = vec![];
    return if let Some(option_index) = iter.position(
        |x| (!short.is_empty() && x.eq(short)) || x.eq(long)) {
        if op_type == Flag { return Ok(Some(vec![])); }
        let option = args.get(option_index).unwrap();
        let mut values: Vec<String> = vec![];
        let mut index = option_index + 1;
        while let Some(value) = args.get(index) {
            if value.starts_with("-") { break; }
            values.push(value.to_owned());
            value_indices.push(index);
            if op_type == Value { break; };
            index += 1;
        }
        if values.is_empty() {
            return Err(format!("Missing option value(s) for: {}", option));
        }
        let original_len = args.len();
        args.remove(option_index);
        for i in value_indices { args.remove(i - (original_len - args.len())); }
        Ok(Some(values))
    } else {
        Ok(None)
    };
}