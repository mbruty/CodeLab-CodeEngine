use crate::strategies::Strategy;
use crate::utils::{exec_command_output, write_to_file};

pub struct DotnetStrategy;
impl Strategy for DotnetStrategy {
    fn build(&self, code: &str) -> Result<String, String> {
        // Write the program to fs
        write_to_file(code, "Solution.cs");
        let output = exec_command_output("dotnet", Vec::from(["build", "--configuration", "Release"]));
        let stdout = String::from_utf8(output.stdout).expect("");

        if output.status.code().expect("No status code for program") != 0 {
            let mut split: Vec<&str> = stdout.split('\n').collect();
            while split[0] != "Build FAILED." {
                split.remove(0);
            }

            return Err(split.join("\n"));
        }
        return Ok(stdout);
    }

    fn setup_tests(&self, tests: &str) -> String {
        write_to_file(tests, "UnitTests.cs");
        String::new()
    }

    fn run(&self) -> (String, String, bool) {
        let output = exec_command_output("./bin/Release/net6.0/Application", Vec::from([]));
        let stdout = String::from_utf8(output.stdout).expect("");
        let stderr = String::from_utf8(output.stderr).expect("");

        (stdout, stderr, output.status.success())

    }

    fn get_command(&self) -> &'static str {
        "Application"
    }

    fn get_queue_name(&self) -> &'static str { "DOTNET" }

    fn print_greeting(&self) { println!("[.] Awaiting RPC requests on the Dotnet queue"); }

    fn process_result(&self, data: String) -> (String, i32) {
        let mut split: Vec<&str> = data.split('\n').collect();
        let mut output: Vec<&str> = Vec::new();
        let mut gather_errors = false;
        let mut exec_time_ms: i32 = -1;
        // Remove un-necessary lines
        while split[0] != "Test Run Summary" {
            if split[0] == "Errors, Failures and Warnings" {
                gather_errors = true;
            }
            if split[0] == "Run Settings" {
                gather_errors = false;
            }
            if gather_errors && split[0] != "" {
                output.push(split[0]);
            }
            split.remove(0);
        }

        for line in &split {
            if line.starts_with("    Duration: ") {
                let copy = line.clone();
                let mut seconds = copy.replace("    Duration: ", "");
                seconds = seconds.replace(" seconds", "");

                let ms: f32 = match seconds.parse::<f32>() {
                    Ok(v) => v * 1000.0,
                    Err(_) => -1.0
                };

                exec_time_ms = ms as i32;
            }
        }
        split.pop();
        split.pop();
        output.append(&mut split);
        return (output.join("\n"), exec_time_ms)
    }
}
