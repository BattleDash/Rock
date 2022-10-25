use std::env;

pub fn cmd_option(option: &str) -> Option<String> {
    let args: Vec<String> = env::args().collect();
    let prefix = '-';

    for (i, arg) in args.iter().enumerate() {
        if arg.as_bytes()[0] as char == prefix && arg.strip_prefix(prefix).unwrap() == option {
            if i == args.len() - 1 {
                return Some(arg.to_string());
            }

            let next = &args[i + 1];
            if next.as_bytes()[0] as char == prefix {
                return Some(arg.to_string());
            }

            return Some(next.to_string());
        }
    }

    None
}

pub fn cmd_option_default(option: &str, default: &str) -> String {
    let result = cmd_option(option);

    if result.is_some() {
        return result.unwrap();
    }

    default.to_string()
}