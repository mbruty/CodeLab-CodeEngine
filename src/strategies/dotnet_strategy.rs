use crate::strategies::Strategy;
use crate::utils::{exec_command_output, write_to_file};

pub struct DotnetStrategy;
impl Strategy for DotnetStrategy {
    fn build(&self, code: &str) -> Result<String, String> {
        const CSPROJ: &str = "
<Project Sdk=\"Microsoft.NET.Sdk\">
  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net6.0</TargetFramework>
    <ImplicitUsings>enable</ImplicitUsings>
    <Nullable>enable</Nullable>
    <IsPackable>false</IsPackable>
    <GenerateProgramFile>false</GenerateProgramFile>
  </PropertyGroup>
  <ItemGroup>
    <PackageReference Include=\"nunit\" Version=\"3.13.3\" />
    <PackageReference Include=\"Microsoft.NET.Test.Sdk\" Version=\"17.3.2\" />
    <PackageReference Include=\"NUnitLite\" Version=\"3.13.3\" />
    <PackageReference Include=\"NUnitTestAdapter.WithFramework\" Version=\"2.0.0\" />
  </ItemGroup>

</Project>
";

        const TEST_RUNNER: &str = "
using NUnitLite;
using System.Reflection;

namespace program;
public class Program
{
    public static int Main()
    {
        return new AutoRun(Assembly.GetExecutingAssembly())
                       .Execute(new String[] { \"/test:program\" });
    }
}
";
        // Write the csproj to fs
        write_to_file(CSPROJ, "Application.csproj");
        // Write the program to fs
        write_to_file(TEST_RUNNER, "Program.cs");
        write_to_file(code, "Solution.cs");
        let output = exec_command_output("dotnet", Vec::from(["build", "--configuration", "Release"]));
        let stdout = String::from_utf8(output.stdout).expect("");

        if output.status.code().expect("No status code for program") != 0 {
            return Err(stdout);
        }
        return Ok(stdout);
    }

    fn setup_tests(&self, tests: &str) -> String {
        write_to_file(tests, "UnitTests.cs");
        String::new()
    }

    fn run(&self) -> (String, bool) {
        let output = exec_command_output("./bin/Release/net6.0/Application", Vec::from([]));
        let stdout = String::from_utf8(output.stdout).expect("");
        println!("[dotnet] Run status: {}", output.status);
        println!("[dotnet] Run stdout: {}", stdout);
        println!("[dotnet] Run stderr: {}", String::from_utf8(output.stderr).expect(""));
        (stdout, output.status.success())

    }

    fn get_command(&self) -> &'static str {
        "Application"
    }

    fn get_queue_name(&self) -> &'static str { "DOTNET" }

    fn print_greeting(&self) { println!("[.] Awaiting RPC requests on the Dotnet queue"); }
}