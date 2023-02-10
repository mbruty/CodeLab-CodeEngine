use crate::Languages;
use crate::utils::{write_to_file};

pub mod dotnet_strategy;
pub mod bunjs_strategy;
pub mod bunts_strategy;
pub mod py_strategy;

pub trait Strategy {
    fn build(&self, code: &str) -> Result<String, String>;

    fn write_files(&self, file: &str, file_name: &str) {
        write_to_file(file, file_name);
    }

    fn setup_tests(&self, tests: &str) -> String;
    fn run<'a>(&self) -> (String, String, bool);
    fn get_command(&self) -> &'static str;
    fn get_queue_name(&self) -> &'static str;
    fn print_greeting(&self);
    fn process_result(&self, data: String) -> (String, i32);
}

pub fn get_strategy_for(lang: Languages) -> Box<dyn Strategy> {
    return match lang {
        Languages::Dotnet => Box::new(dotnet_strategy::DotnetStrategy),
        Languages::JavaScript => Box::new(bunjs_strategy::BunJavaScriptStrategy),
        Languages::TypeScript => Box::new(bunts_strategy::BunTypeScriptStrategy),
        Languages::BunJavaScript => Box::new(bunjs_strategy::BunJavaScriptStrategy),
        Languages::Python => Box::new(py_strategy::PythonStrategy)
    }
}