use crate::strategies::Strategy;
use crate::utils::{exec_command, write_to_file};

pub struct NodeJavaScriptStrategy;
impl Strategy for NodeJavaScriptStrategy {

    // We don't build here in JS land
    fn build(&self, code: &str) -> String {
        // Write the program to fs
        write_to_file(code, "index.js");
        String::new()
    }

    fn run(&self) -> String {
        exec_command("/usr/bin/node", Vec::from(["index.js"]))
    }

    fn get_command(&self) -> &'static str {
        "node"
    }

    fn get_queue_name(&self) -> &'static str { "JS" }

    fn print_greeting(&self) { println!("[.] Awaiting RPC requests on the JavaScript queue"); }
}