use crate::strategies::Strategy;
use crate::utils::{exec_command, write_to_file};

pub struct DotnetStrategy;
impl Strategy for DotnetStrategy {
    fn build(&self, code: &str) -> String {
        const CSPROJ: &str = "
            <Project Sdk=\"Microsoft.NET.Sdk\">
                <PropertyGroup>
                    <OutputType>Exe</OutputType>
                    <TargetFramework>net6.0</TargetFramework>
                    <ImplicitUsings>enable</ImplicitUsings>
                    <Nullable>enable</Nullable>
                </PropertyGroup>
            </Project>";

        // Write the csproj to fs
        write_to_file(CSPROJ, "Application.csproj");
        // Write the program to fs
        write_to_file(code, "Program.cs");
        exec_command("dotnet", Vec::from(["build", "--configuration", "Release"]))
    }

    fn run(&self) -> String {
        exec_command("./bin/Release/net6.0/Application", Vec::from([]))
    }

    fn get_command(&self) -> &'static str {
        "Application"
    }

    fn get_queue_name(&self) -> &'static str { "DOTNET" }

    fn print_greeting(&self) { println!("[.] Awaiting RPC requests on the Dotnet queue"); }
}