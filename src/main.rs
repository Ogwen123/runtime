mod utils;

use crate::utils::logger::{fatal, info, warning};
use std::env;

pub struct Config {
    runs: u32
}

fn main() {
    let mut config: Config = Config {
        runs: 5
    };

    let args: Vec<String> = env::args().collect();

    let runs_flag: String = "-runs".to_string();

    if args.contains(&runs_flag) {
        let pos: usize = args.iter().position(|x| x == "-runs").unwrap();

        let item_result: Option<&String> = args.get(pos + 1);

        let res = match item_result {
            Ok(str) => str,
            Err(_) => {
                fatal!("The -runs flags should be used as follows: ... -run <run count> ...");
                return;
            }
        };

        let parse_result = match res.parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                fatal!("The -runs flags should be used as follows: ... -run <number> ...");
            }
        };

        config.runs = parse_result;
    }
}
