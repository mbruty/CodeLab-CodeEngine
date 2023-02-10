using Newtonsoft.Json.Linq;
using NUnit.Common;
using NUnitLite;
using System.Reflection;
using System.Reflection.Emit;
using System.Text;

namespace program;

class Writer : ExtendedTextWriter
{
    public override Encoding Encoding => throw new NotImplementedException();

    public override void Write(ColorStyle style, string value)
    {
        Console.Error.Write(value);
    }

    public override void WriteLabel(string label, object option)
    {
        Console.Error.Write(label + '\t' + option);
    }

    public override void WriteLabel(string label, object option, ColorStyle valueStyle)
    {
        Console.Error.Write(label + '\t' + option);
    }

    public override void WriteLabelLine(string label, object option)
    {
        Console.Error.WriteLine(label + '\t' + option);
    }

    public override void WriteLabelLine(string label, object option, ColorStyle valueStyle)
    {
        Console.Error.WriteLine(label + '\t' + option);
    }

    public override void WriteLine(ColorStyle style, string value)
    {
        Console.Error.WriteLine(value);
    }
}
public class Program
{
    public static int Main()
    {
        return new AutoRun(Assembly.GetExecutingAssembly())
                       .Execute(new string[] { "/test:program" }, new Writer(), null);
    }
}