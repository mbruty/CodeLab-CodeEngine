use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use crate::{get_strategy_for, Languages};
use crate::utils::{exec_command, get_stats, process_stats};
use crate::strategies::Strategy;
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
struct Instruction {
    code: String,
    test: String
}

#[derive(Serialize)]
struct Stat {
    cpu: String,
    mem: String
}

#[derive(Serialize)]
struct CodeResult {
    output: String,
    stats: Vec<Stat>,
    execution_time_ms: u32,
    error_text: String,
    is_successful: bool
}
pub fn handle(instruction: &str, language: Languages) -> String {
    // ToDo: Read this with commandline args
    let ctx: Box<dyn Strategy> = get_strategy_for(language);

    // Temp: Just build without validation
    let now = Instant::now();
    let deserialized: Instruction = serde_json::from_str(&instruction).unwrap();
    ctx.setup_tests(&*deserialized.test);
    if let Err(e) = ctx.build(&*deserialized.code) {
        let code_result = CodeResult{
            output: "".parse().unwrap(),
            stats: Vec::new(),
            execution_time_ms: 0,
            error_text: e,
            is_successful: false
        };
        return serde_json::to_string(&code_result).unwrap();
    }
    let elapsed = now.elapsed();
    println!("[.{}] build took: {:.2?}", ctx.get_queue_name(), elapsed);

    let command = ctx.get_command();
    let shared_command = Arc::new(Mutex::new(command));
    let shared_output = Arc::new(Mutex::new(String::new()));
    let shared_stats = Arc::new(Mutex::new(String::new()));
    let shared_time = Arc::new(Mutex::new(String::new()));
    let shared_sucess = Arc::new(Mutex::new(false));
    // Clone the output and stats here so shared_output is not moved into the thread closure
    // This allows us to access it again in the current closure
    let output = Arc::clone(&shared_output);
    let stats = Arc::clone(&shared_stats);
    let time = Arc::clone(&shared_time);
    let sucess = Arc::clone(&shared_sucess);

    let stats_thread = thread::spawn(move || {

        // Clone the objects we need
        let mut running = true;
        let command = Arc::clone(&shared_command);

        let command_mutex = command.lock().unwrap();
        let mut results = Vec::new();

        // Number of retries to try and find the process stats
        let mut retries = 0;
        const MAX_RETRIES: i32 = 50;

        // Get the pid
        let mut pid = String::new();
        while pid.is_empty() {
            pid = exec_command("pgrep", Vec::from([*command_mutex]));
            retries += 1;
            if retries == MAX_RETRIES {
                panic!("Could not find child process")
            }
        }

        pid.pop(); // Remove trailing \n

        while running {
            let res = get_stats(pid.as_str());
            let processed = process_stats(res.as_str());
            if processed.is_empty() {
                running = false;
            }
            results.push(processed.join(","));
        }

        let mut stats_mutex = stats.lock().unwrap();
        *stats_mutex = results.join("\n")
    });

    let code_thread = thread::spawn(move || {
        let ctx_1: Box<dyn Strategy> = get_strategy_for(language);

        // Clone the objects we need
        // Lock the items we need
        let mut output_mutex = output.lock().unwrap();
        let mut sucess_mutex = sucess.lock().unwrap();
        let mut time_mutex = time.lock().unwrap();

        let now = Instant::now();
        let output = ctx_1.run();
        *output_mutex = output.0;
        *sucess_mutex = output.1;
        let elapsed = now.elapsed();

        *time_mutex = format!("{:.2?}", elapsed.as_millis());
    });

    // Wait for both of the threads to finish
    code_thread.join().expect("TODO: panic message");
    stats_thread.join().expect("TODO: panic message");

    // Copy the results
    let output_copy = Arc::clone(&shared_output);
    let stats_copy = Arc::clone(&shared_stats);
    let time_copy = Arc::clone(&shared_time);
    let sucess_copy = Arc::clone(&shared_sucess);
    let final_output = output_copy.lock().unwrap();
    let final_stats = stats_copy.lock().unwrap();
    let final_time = time_copy.lock().unwrap();
    let final_sucess = sucess_copy.lock().unwrap();

    let stat_lines = (*final_stats.as_str().split("\n").collect::<Vec<&str>>()).to_owned();
    let mut stats: Vec<Stat> = Vec::new();
    for &line in &stat_lines {
        let split = line.split(",").collect::<Vec<&str>>();
        if split.len() == 2 {
            stats.push(Stat { cpu: split[0].parse().unwrap(), mem: split[1].parse().unwrap() });
        }
    }

    let processed = ctx.process_result(final_output.to_string());

    let code_result = CodeResult{
        output: processed.0,
        stats,
        execution_time_ms: if processed.1 == -1 { final_time.parse().unwrap() } else { processed.1 } as u32,
        error_text: "".parse().unwrap(),
        is_successful: *final_sucess
    };
    return serde_json::to_string(&code_result).unwrap();
}