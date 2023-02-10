use crate::strategies::Strategy;
use crate::utils::{exec_command_output, write_to_file};

pub struct BunJavaScriptStrategy;
impl Strategy for BunJavaScriptStrategy {
    // We don't build here in JS land
    fn build(&self, code: &str) -> Result<String, String> {
        // Write the program to fs
        // Append an export for the code so that the tests can read it
        let export_code = code.to_owned() + "\nexport default solve;";
        write_to_file(export_code.as_str(), "index.ts");
        Ok(String::new())
    }

    fn setup_tests(&self, tests: &str) -> String {
        write_to_file(tests, "index.test.js");
        String::new()
    }

    fn run(&self) -> (String, String, bool) {
        // For some reason bun's test outputs to stderr
        let output = exec_command_output("/root/.bun/bin/bun", Vec::from(["wiptest"]));
        let test_output = String::from_utf8(output.stderr).expect("Stderr was not a string");
        let console_output = String::from_utf8(output.stdout).expect("Stdout was not a string");
        (console_output, test_output, output.status.success())
    }

    fn get_command(&self) -> &'static str {
        "bun"
    }

    fn get_queue_name(&self) -> &'static str { "JS" }

    fn print_greeting(&self) { println!("[.] Awaiting RPC requests on the JavaScript queue"); }

    fn process_result(&self, data: String) -> (String, i32) {
        let mut split: Vec<&str> = data.split('\n').collect();
        split.remove(0);
        split.pop();
        let mut last: String = split.last().unwrap().parse().unwrap();
        let mut exec_time_ms = -1;
        if last != "" {
            let split2: Vec<&str> = last.split("[").collect();
            last = split2[1].replace("ms]", "");
            let float_time = last.parse::<f32>().unwrap_or_else(|_| {
                return -1 as f32;
            });
            exec_time_ms = float_time as i32;
        }
        return (split.join("\n"), exec_time_ms);
    }
}