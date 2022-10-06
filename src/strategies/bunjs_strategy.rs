use crate::strategies::Strategy;
use crate::utils::{exec_command_output, write_to_file};

pub struct BunJavaScriptStrategy;
impl Strategy for BunJavaScriptStrategy {

    // We don't build here in JS land
    fn build(&self, code: &str) -> Result<String, String> {
        // Write the program to fs
        write_to_file(code, "index.js");
        Ok(String::new())
    }

    fn setup_tests(&self, tests: &str) -> String {
        write_to_file(tests, "index.test.js");
        String::new()
    }

    fn run(&self) -> String {
        // For some reason bun's test outputs to stderr
        let output = exec_command_output("/usr/bin/bun/bun", Vec::from(["wiptest"]));
        String::from_utf8(output.stderr).expect("Stdout was not a string")
    }

    fn get_command(&self) -> &'static str {
        "bun"
    }

    fn get_queue_name(&self) -> &'static str { "JS" }

    fn print_greeting(&self) { println!("[.] Awaiting RPC requests on the JavaScript queue"); }
}