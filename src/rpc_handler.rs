use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use crate::{get_strategy_for, Languages};
use crate::utils::{exec_command, get_stats, process_stats};
use crate::strategies::Strategy;

pub fn handle(code: &str, language: Languages) -> String {
    // ToDo: Read this with commandline args
    let ctx: Box<dyn Strategy> = get_strategy_for(language);

    // Temp: Just build without validation
    ctx.build(code);

    // ToDo: Validate build
    //let build = ctx.build(code);

    let command = ctx.get_command();
    let shared_command = Arc::new(Mutex::new(command));
    let shared_output = Arc::new(Mutex::new(String::new()));
    let shared_stats = Arc::new(Mutex::new(String::new()));
    let shared_time = Arc::new(Mutex::new(String::new()));

    // Clone the output and stats here so shared_output is not moved into the thread closure
    // This allows us to access it again in the current closure
    let output = Arc::clone(&shared_output);
    let stats = Arc::clone(&shared_stats);
    let time = Arc::clone(&shared_time);

    let code_thread = thread::spawn(move || {
        let ctx_1: Box<dyn Strategy> = get_strategy_for(language);

        // Clone the objects we need
        // Lock the items we need
        let mut output_mutex = output.lock().unwrap();
        let mut time_mutex = time.lock().unwrap();

        let now = Instant::now();
        *output_mutex = ctx_1.run();
        let elapsed = now.elapsed();

        *time_mutex = format!("{:.2?}", elapsed.as_millis());
    });


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

    // Wait for both of the threads to finish
    code_thread.join().expect("TODO: panic message");
    stats_thread.join().expect("TODO: panic message");

    // Copy the results
    let output_copy = Arc::clone(&shared_output);
    let stats_copy = Arc::clone(&shared_stats);
    let time_copy = Arc::clone(&shared_time);
    let mut final_output = output_copy.lock().unwrap();
    let final_stats = stats_copy.lock().unwrap();
    let final_time = time_copy.lock().unwrap();

    // Sanitise the final output
    *final_output = (*final_output.replace("\"","\\\"")).parse().unwrap();

    // Create the json response
    let lines = (*final_output.as_str().split("\n").collect::<Vec<&str>>()).to_owned();
    let mut output_obj = String::new();

    // Add { "output": [
    output_obj.push_str("{\"output\": [");

    // Add each line to the output
    for &line in &lines {
        output_obj.push_str(&*format!("\"{}\",", line));
    }

    output_obj.pop();

    // Add closing ]
    output_obj.push_str("],");

    // Add "stats": [
    output_obj.push_str("\"stats\": [");

    let stat_lines = (*final_stats.as_str().split("\n").collect::<Vec<&str>>()).to_owned();

    // Add each stat to the output { "cpu": %, "mem": mb }
    for &line in &stat_lines {
        let split = line.split(",").collect::<Vec<&str>>();
        if split.len() == 2 {
            output_obj.push_str(&*format!("{{\"cpu\": \"{}\", \"mem\": \"{}\"}},", split[0], split[1]));
        }
    }
    output_obj.pop();

    // Add closing ] to the string and opening "execution_time" :
    output_obj.push_str("], \"execution_time_ms\" : ");

    // Add execution time to the string
    output_obj.push_str(&*final_time);

    // Close the final child object, and parent object
    output_obj.push_str("}");
    return output_obj;
}