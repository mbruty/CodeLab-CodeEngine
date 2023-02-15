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
}