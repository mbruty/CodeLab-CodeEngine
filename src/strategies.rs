use crate::Languages;

pub mod dotnet_strategy;
pub mod bunjs_strategy;
pub mod bunts_strategy;

pub trait Strategy {
    fn warm_up(&self);
    fn build(&self, code: &str) -> Result<String, String>;
    fn setup_tests(&self, tests: &str) -> String;
    fn run<'a>(&self) -> (String, bool);
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
        Languages::BunJavaScript => Box::new(bunjs_strategy::BunJavaScriptStrategy)
    }
}