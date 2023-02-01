use crate::strategies::Strategy;
use crate::utils::{exec_command_output, write_to_file};

pub struct PythonStrategy;
impl Strategy for PythonStrategy {
    fn build(&self, code: &str) -> Result<String, String> {
        write_to_file(code, "implementation.py");
        Ok(String::new())
    }

    fn setup_tests(&self, tests: &str) -> String {
        write_to_file(tests, "tests.py");
        String::new()
    }

    fn run(&self) -> (String, bool) {
        let output = exec_command_output("python3", Vec::from(["tests.py"]));
        let data = String::from_utf8(output.stderr).expect("Stdout was not a string");
        let split: Vec<&str> = data.split('\n').collect();
        let element = split[split.len() - 2].trim();
        let success = element == "OK";
        (data, success)
    }

    fn get_command(&self) -> &'static str { "python3" }

    fn get_queue_name(&self) -> &'static str { "PY" }

    fn print_greeting(&self) { println!("[.] Awaiting RPC requests on the Python queue"); }

    fn process_result(&self, data: String) -> (String, i32) {
        let split: Vec<&str> = data.split('\n').collect();
        let mut time = -1;
        for line in split.iter() {
            if line.starts_with("Ran") {
                let split2: Vec<&str> = line.split(' ').collect();
                let mut str = split2.last().unwrap().to_string();
                str.pop();
                let ftime = str.parse::<f32>().unwrap_or_else(|_| { -1 as f32 });
                time = (ftime * 1000.0f32) as i32;
            }
        }

        (data, time)
    }
}