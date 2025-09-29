const USAGE: &str = "\x1b[1mu2u\x1b[22m [0.1.0]

A tool for handling ulid/uuid conversions.

\x1b[1mUSAGE\x1b[22m: u2u [OPTIONS] [ulids]...

\x1b[1mOPTIONS\x1b[22m:
  -h, --help        Print help

\x1b[1mARGS\x1b[22m:
  <ulids>...        ULIDs to convert";

pub fn print_usage() {
    println!("{USAGE}");
}

pub fn is_help(args: Vec<String>) -> bool {
    for v in ["-h", "--help"]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
    {
        if args.iter().any(|s| s == &v) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use crate::usage::is_help;

    #[test]
    fn test_is_help_true_short() {
        let some_args: Vec<String> = vec!["u2u", "anyvalue", "--help"]
            .iter()
            .map(|a| a.to_string())
            .collect();
        let h = is_help(some_args);
        assert_eq!(h, true);
    }

    #[test]
    fn test_is_help_true_long() {
        let some_args = vec!["u2u", "anyvalueA", "-h"]
            .iter()
            .map(|a| a.to_string())
            .collect();
        let h = is_help(some_args);
        assert_eq!(h, true);
    }

    #[test]
    fn test_is_help_false() {
        let some_args = vec!["u2u", "anyvalueA"]
            .iter()
            .map(|a| a.to_string())
            .collect();
        let h = is_help(some_args);
        assert_eq!(h, false);
    }
}

#[cfg(test)]
mod test_dummy {
    #[test]
    fn test_usage() {
        crate::usage::print_usage();
    }
}
