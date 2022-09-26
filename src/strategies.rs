use crate::Languages;

pub mod dotnet_strategy;
pub mod bunjs_strategy;
pub mod nodejs_strategy;
pub mod bunts_strategy;

pub trait Strategy {
    fn build(&self, code: &str) -> String;
    fn run<'a>(&self) -> String;
    fn get_command(&self) -> &'static str;
    fn get_queue_name(&self) -> &'static str;
    fn print_greeting(&self);
}

pub fn get_strategy_for(lang: Languages) -> Box<dyn Strategy> {
    return match lang {
        Languages::Dotnet => Box::new(dotnet_strategy::DotnetStrategy),
        Languages::JavaScript => Box::new(bunjs_strategy::BunJavaScriptStrategy),
        Languages::TypeScript => Box::new(bunts_strategy::BunTypeScriptStrategy),
        Languages::NodeJavaScript => Box::new(nodejs_strategy::NodeJavaScriptStrategy),
        Languages::BunJavaScript => Box::new(bunjs_strategy::BunJavaScriptStrategy)
    }
}