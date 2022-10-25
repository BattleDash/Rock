use std::env;

pub fn cmd_option(option: &str) -> Option<String> {
    let mut args = env::args().peekable();
    let prefix = '-';

    while let Some(arg) = args.next() {
        if arg.strip_prefix(prefix) == Some(option) {
            return Some(args.next_if(|v| !v.starts_with(prefix)).unwrap_or(arg));
        }
    }

    None
}

pub fn cmd_option_default(option: &str, default: &str) -> String {
    cmd_option(option).unwrap_or_else(|| default.to_string())
}
