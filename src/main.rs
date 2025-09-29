mod outcome;
mod process;
mod usage;

use process::process;
use usage::{is_help, print_usage};

fn get_cli_args() -> Vec<String> {
    std::env::args().collect::<Vec<String>>().split_off(1)
}

fn main() {
    let cli_args = get_cli_args();
    act(cli_args);
}

fn act(args: Vec<String>) {
    if is_help(args.to_owned()) {
        print_usage();
    } else if process(args) == 0 {
        eprintln!("no input. use -h.");
    }
}

#[cfg(test)]
mod test_dummy {
    #[test]
    fn test_main() {
        crate::main();
    }

    #[test]
    fn test_act_usage() {
        crate::act(vec!["u2u".to_string(), "-h".to_string()]);
    }
}
