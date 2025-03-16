mod utils;

use crate::utils::logger::{fatal, info, warning};
use std::env;
use std::io::{stdin, stdout, Write};
use std::process::{Command, Stdio};
use std::time::SystemTime;

pub struct Config {
    runs: u32,
    command: String,
    args: Vec<String>,
    output: bool
}

pub struct Results {
    times: Vec<u128>
}

fn _print_config(config: &Config) {
    println!("runs: {}", config.runs);
    println!("command: {}", config.command);
    println!("args: {:?}", config.args);
    println!("show output? {:?}", config.output);
}

fn take_input(message: &str) -> String {
    let mut input =String::new();
    print!("{}", message);
    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("error when taking input");
    if let Some('\n')= input.chars().next_back() {
        input.pop();
    }
    if let Some('\r')= input.chars().next_back() {
        input.pop();
    }

    return input
}

fn main() {
    let mut config: Config = Config {
        runs: 5,
        command: String::new(),
        args: Vec::new(),
        output: false
    };

    let args: Vec<String> = env::args().collect();

    let runs_flag: String = String::from("-runs");
    let output_flag: String = String::from("-output");


    // check command flags
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

    // take command input and parse args
    let full_command = take_input("command: ");

    config.command = (&full_command).split(" ").collect::<Vec<_>>()[0].to_string();
    let args_string = (&full_command).split(" ").collect::<Vec<_>>()[1..].join(" ");

    let mut in_quotes: bool = false;
    let mut quotes_buffer: String = String::new();
    let mut arguments: Vec<String> = Vec::new();

    for i in args_string.split(" ") {
        if !in_quotes && i.starts_with("\"") {
            in_quotes = true;
            quotes_buffer += i;
            quotes_buffer += " ";
            continue;
        }

        if in_quotes && i.ends_with("\"") {
            in_quotes = false;
            quotes_buffer += i;
            arguments.push(quotes_buffer);
            quotes_buffer = String::from(" ");
            continue;
        }

        if in_quotes {
            quotes_buffer += i;
            quotes_buffer += " ";
            continue;
        }

        arguments.push(String::from(i));
    }

    config.args = arguments;

    _print_config(&config);
    // perform tests
    let mut results = Results {
        times: Vec::new()
    };

    for i in 0..config.runs {
        info!("Run {}...", i+1);
        let now = SystemTime::now();

        let base_command = &config.command.split(" ").collect::<Vec<_>>()[0];

        let mut command = Command::new(base_command);

        command
            .args(&config.args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let raw_output = command.output();

        let output = match raw_output {
            Ok(res) => res,
            Err(_) => {
                fatal!("The command could not be run.");
                return;
            }
        };

        let time_taken = match now.elapsed() {
            Ok(elapsed) => elapsed.as_millis(),
            Err(_) => {
                warning!("Timing failed.");
                0
            }
        };

        if config.output {
            let output_format_result = String::from_utf8(output.stdout);

            let output_str = match output_format_result {
                Ok(res) => res,
                Err(_) => {
                    fatal!("Could not parse the solution output.");
                    return;
                }
            };
            let number_length = ((if i == 0 {1} else {i}).ilog10() + 1) as usize;
            println!("---------- Run {} Output ----------", i);
            println!("{}", output_str);
            println!("---------------{}------------------", "-".repeat(number_length));
        }

        results.times.push(time_taken);
    }
}
