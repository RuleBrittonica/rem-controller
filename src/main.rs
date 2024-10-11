mod non_local_controller;
use clap::{Parser, Subcommand};
use colored::Colorize;
use rem_utils::compile_file;
use std::process::exit;
use std::time::SystemTime;
use std::{env, fs};
use std::time::Duration;

use non_local_controller::ControllerInput;
mod error;
use crate::error::ControllerError;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the repairs
    Run {
        file_name: String,
        new_file_name: String,
        caller_fn_name: String,
        callee_fn_name: String,
    },
    Test {},
}

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let args = Cli::parse();
    match &args.command {
        Commands::Test {} => test(),
        Commands::Run {
            file_name,
            new_file_name,
            caller_fn_name,
            callee_fn_name,
        } => {
            let input_code: String = fs::read_to_string( file_name ).unwrap();
            let input: ControllerInput = ControllerInput {
                input_code,
                caller_fn_name: caller_fn_name.to_string(),
                callee_fn_name: callee_fn_name.to_string(),
            };
            let result:Result<String, ControllerError>  = non_local_controller::make_controls( input );
            match result {
                Ok(_) => {
                    fs::write(
                        new_file_name.clone(),
                        result.unwrap()
                    ).unwrap();
                    exit(0)
                },
                Err(e) => {
                    eprintln!("Error: {}", e);
                    exit(1)
                }
            }
        }
    }
}

fn test() {
    let mut test_durations: Vec<Duration> = Vec::new();
    let mut successful_tests: i32 = 0;
    let mut num_tests: i32 = 0;
    let mut total_time: Duration = Duration::new(0, 0);
    let mut min_time: Duration = Duration::new(std::u64::MAX, 999_999_999);
    let mut max_time: Duration = Duration::new(0, 0);

    for file in fs::read_dir("./input").unwrap() {
        num_tests += 1;
        
        let test_name = file.unwrap().file_name().to_owned();
        if test_name.to_str().unwrap() == "if_return_unit_controller.rs" {
            continue;
        }
        // if !test_name.to_str().unwrap().contains("qmark_test") {
        //     continue;
        // }
        let file_name: String = format!("./input/{}", test_name.to_str().unwrap());
        let new_file_name: String = format!("./output/{}", test_name.to_str().unwrap());
        let callee_fn_name: &str = "bar";
        let caller_fn_name: &str = "new_foo";
        let input_code: String = fs::read_to_string(file_name.clone()).unwrap();
        let now: SystemTime = SystemTime::now();

        let input: ControllerInput = ControllerInput {
            input_code,
            caller_fn_name: caller_fn_name.to_string(),
            callee_fn_name: callee_fn_name.to_string(),
        };
        let result = non_local_controller::make_controls(input);

        let time_elapsed = now.elapsed().unwrap();

        test_durations.push(time_elapsed);
        total_time += time_elapsed;
        if time_elapsed < min_time {
            min_time = time_elapsed;
        }
        if time_elapsed > max_time {
            max_time = time_elapsed;
        }

        if let Ok(ref success) = result {
            successful_tests += 1;
            fs::write(new_file_name.clone(), success).unwrap();
            let args = vec![];
            let mut compile_cmd = compile_file(new_file_name.as_str(), &args);
            let out = compile_cmd.output().unwrap();
            println!(
                "{}: {} in {:#?}",
                if out.status.success() && result.is_ok() {
                    format!("PASSED").green()
                } else {
                    format!("FAILED").red()
                },
                test_name.to_str().unwrap(),
                time_elapsed
            );
            println!("------------------------------------------------------------------\n");
        } else {
            println!("FAILED: {} in {:#?}", test_name.to_str().unwrap(), time_elapsed);
            println!("------------------------------------------------------------------\n");
        }
    }

    // Calculate statistics
    let average_time: Duration = if !test_durations.is_empty() {
        total_time / test_durations.len() as u32
    } else {
        Duration::new(0, 0)
    };

    println!("Test Statistics:");
    println!("Number of successful tests: {} out of {}", format!("{}", successful_tests).green(), format!("{}", num_tests).blue());
    println!("Total time elapsed: {:#?}", total_time);
    println!("Average time per test: {:#?}", average_time);
    println!("Minimum time for a test: {:#?}", min_time);
    println!("Maximum time for a test: {:#?}", max_time);
}
