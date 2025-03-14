mod utils;

use crate::utils::logger::{fatal, warning};
use std::env;

pub struct Config {
    runs: u32,
    command: String
}

fn main() {
    let mut config: Config = Config {
        runs: 5,
        command: String::new()
    };

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        warning!("You must provide a command to run between quotes. E.g. runtime \"<command>\"")
    }

    let runs_flag: String = String::from("-runs");

    // get command from args
    let command_res = args.get(1);

    config.command = match command_res {
        Some(command) => command.clone(),
        None => {
            warning!("No command supplied");
            return;
        }
    };

    // get run count from args
    if args.contains(&runs_flag) {
        let pos: usize = args.iter().position(|x| x == "-runs").unwrap();

        let item_result: Option<&String> = args.get(pos + 1);

        let res: &String = match item_result {
            Some(item) => item,
            None => {
                fatal!("The -runs flags should be used as follows: ... -run <run count> ...");
                return;
            }
        };

        let parse_result = match res.parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                fatal!("The -runs flags should be used as follows: ... -run <number> ...");
                return;
            }
        };

        config.runs = parse_result as u32;
    }
}
