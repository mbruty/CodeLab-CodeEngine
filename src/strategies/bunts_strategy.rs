use crate::strategies::Strategy;
use crate::utils::{exec_command, write_to_file};

pub struct BunTypeScriptStrategy;
impl Strategy for BunTypeScriptStrategy {

    // We don't build here in JS land
    fn build(&self, code: &str) -> Result<String, String> {
        // Write the program to fs
        write_to_file(code, "index.ts");
        Ok(String::new())
    }

    fn run(&self) -> String {
        exec_command("/usr/bin/bun/bun", Vec::from(["wiptest"]))
    }

    fn setup_tests(&self, tests: &str) -> String {
        write_to_file(tests, "index.test.ts");
        String::new()
    }

    fn get_command(&self) -> &'static str {
        "bun"
    }

    fn get_queue_name(&self) -> &'static str { "TS" }

    fn print_greeting(&self) { println!("[.] Awaiting RPC requests on the TypeScript queue"); }
}