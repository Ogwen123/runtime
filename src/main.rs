mod utils;

use crate::utils::logger::{fatal, info, warning};
use std::env;
use std::process::{Command, Stdio};
use std::time::SystemTime;

pub struct Config {
    runs: u32,
    command: String,
    output: bool
}

pub struct Results {
    times: Vec<u128>
}

fn main() {
    let mut config: Config = Config {
        runs: 5,
        command: String::new(),
        output: false
    };

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        warning!("You must provide a command to run between quotes. E.g. runtime \"<command>\"")
    }

    let runs_flag: String = String::from("-runs");
    let output_flag: String = String::from("-output");

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

    if args.contains(&output_flag) {
        config.output = true;
    }

    let mut results = Results {
        times: Vec::new()
    };

    for i in 0..config.runs {
        info!("Run {}...", i+1);
        println!("{}", config.command.split(" ").collect::<Vec<_>>()[0]);
        let now = SystemTime::now();
        let raw_output = Command::new(&config.command.split(" ").collect::<Vec<_>>()[0])
            .args(&config.command.split(" ").collect::<Vec<_>>()[1..])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .expect("Failed to run command.");

        let time_taken = match now.elapsed() {
            Ok(elapsed) => elapsed.as_millis(),
            Err(_) => {
                warning!("Timing failed.");
                0
            }
        };

        if config.output {
            let output_format_result = String::from_utf8(raw_output.stdout);

            let output_str = match output_format_result {
                Ok(res) => res,
                Err(_) => {
                    fatal!("Could not parse the solution output.");
                    return;
                }
            };

            println!("---------- Run {} Output ----------", i);
            println!("{}", output_str);
            println!("---------------{}------------------", "-".repeat((i.ilog10() + 1) as usize));
        }

        results.times.push(time_taken);
    }
}
