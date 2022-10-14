using NUnitLite;
using System.Reflection;
namespace program;
public class Program
{
    public static int Main()
    {
        return new AutoRun(Assembly.GetExecutingAssembly())
                       .Execute(new String[] { "/test:program" });
    }
}