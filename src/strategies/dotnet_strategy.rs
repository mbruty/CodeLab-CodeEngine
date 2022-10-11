use crate::strategies::Strategy;
use crate::utils::{delete_file, exec_command_output, write_to_file};

pub struct DotnetStrategy;
impl Strategy for DotnetStrategy {
    fn warm_up(&self) {
        println!("[.DOTNET] Warming up");
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
    <PackageReference Include=\"NUnit3TestAdapter\" Version=\"4.2.1\" />
  </ItemGroup>
</Project>
";
        // Write the csproj to fs
        write_to_file(CSPROJ, "Application.csproj");

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
        // Write the test runner code to the file system
        write_to_file(TEST_RUNNER, "Program.cs");

        const DUMMY_TESTS: &str = "
using NUnit.Framework;

namespace program;

[TestFixture]
public class TestRunner
{
    private Solution? _solution;

    [SetUp]
    public void SetUp()
    {
        _solution = new Solution();
    }

    [Test]
    public void DummyTest()
    {
        Assert.AreEqual(0, _solution?.Solve());
    }
}";

        // Write the dummy tests to the fs
        self.setup_tests(DUMMY_TESTS);

        const DUMMY_CODE: &str = "
namespace program;
public class Solution
{
    public int Solve()
    {
        // Your code here
        return 0;
    }
}";
        // Write the dummy code to the FS
        self.build(DUMMY_CODE).expect("TODO: panic message");

        // Clean up the files that don't need to still be there
        delete_file("Solution.cs");
        delete_file("UnitTests.cs");
    }

    fn build(&self, code: &str) -> Result<String, String> {

        // Write the program to fs
        write_to_file(code, "Solution.cs");
        let output = exec_command_output("dotnet", Vec::from(["build", "--configuration", "Release"]));
        let stdout = String::from_utf8(output.stdout).expect("");

        if output.status.code().expect("No status code for program") != 0 {
            let mut split: Vec<&str> = stdout.split('\n').collect();
            while split[0] != "Build FAILED." {
                split.remove(0);
            }

            return Err(split.join("\n"));
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
        (stdout, output.status.success())

    }

    fn get_command(&self) -> &'static str {
        "Application"
    }

    fn get_queue_name(&self) -> &'static str { "DOTNET" }

    fn print_greeting(&self) { println!("[.] Awaiting RPC requests on the Dotnet queue"); }

    fn process_result(&self, data: String) -> (String, i32) {
        let mut split: Vec<&str> = data.split('\n').collect();
        let mut output: Vec<&str> = Vec::new();
        let mut gather_errors = false;
        let mut exec_time_ms: i32 = -1;
        // Remove un-necessary lines
        while split[0] != "Test Run Summary" {
            if split[0] == "Errors, Failures and Warnings" {
                gather_errors = true;
            }
            if split[0] == "Run Settings" {
                gather_errors = false;
            }
            if gather_errors && split[0] != "" {
                output.push(split[0]);
            }
            split.remove(0);
        }

        for line in &split {
            if line.starts_with("    Duration: ") {
                let copy = line.clone();
                let mut ms_str = copy.replace("    Duration: ", "");
                ms_str = ms_str.replace(" seconds", "");
                let seconds: f32 = ms_str.parse().unwrap_or_else(|_| -1) as f32;
                exec_time_ms = (seconds * 1000 as f32) as i32;
            }
        }
        split.pop();
        split.pop();
        output.append(&mut split);
        return (output.join("\n"), exec_time_ms)
    }
}