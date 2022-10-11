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
    public void TwoTimesTwo_IsFour()
    {
        var result = Solution.Solve(2);

        Assert.AreEqual(4, result);
    }
}
