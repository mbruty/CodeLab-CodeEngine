use crate::strategies::Strategy;
use crate::utils::{exec_command, write_to_file};

pub struct BunJavaScriptStrategy;
impl Strategy for BunJavaScriptStrategy {

    // We don't build here in JS land
    fn build(&self, code: &str) -> String {
        // Write the program to fs
        write_to_file(code, "index.js");
        String::new()
    }

    fn run(&self) -> String {
        exec_command("/root/.bun/bin/bun", Vec::from(["run", "index.js"]))
    }

    fn get_command(&self) -> &'static str {
        "bun"
    }

    fn get_queue_name(&self) -> &'static str { "JS" }

    fn print_greeting(&self) { println!("[.] Awaiting RPC requests on the JavaScript queue"); }
}