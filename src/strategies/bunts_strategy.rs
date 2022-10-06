use crate::strategies::Strategy;
use crate::utils::{exec_command_output, write_to_file};

pub struct BunTypeScriptStrategy;
impl Strategy for BunTypeScriptStrategy {

    // We don't build here in JS land
    fn build(&self, code: &str) -> Result<String, String> {
        // Write the program to fs
        // Append an export for the code so that the tests can read it
        let export_code = code.to_owned() + "\nexport default solve;";
        write_to_file(export_code.as_str(), "index.ts");
        Ok(String::new())
    }

    fn setup_tests(&self, tests: &str) -> String {
        write_to_file(tests, "index.test.ts");
        String::new()
    }

    fn run(&self) -> (String, bool) {
        let output = exec_command_output("/root/.bun/bin/bun", Vec::from(["wiptest"]));
        println!("Sucess: {}", output.status.success());
        (String::from_utf8(output.stderr).expect("Stdout was not a string"), output.status.success())
    }

    fn get_command(&self) -> &'static str {
        "bun"
    }

    fn get_queue_name(&self) -> &'static str { "TS" }

    fn print_greeting(&self) { println!("[.] Awaiting RPC requests on the TypeScript queue"); }
}