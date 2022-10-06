
using NUnit.Framework;

namespace program;

[TestFixture]
public class UnitTests
{
    private Solution _solution;

    [SetUp]
    public void SetUp()
    {
        _solution = new Solution();
    }

    [Test]
    public void TwoTimesTwo_IsFour()
    {
        var result = Solution.Solve(2);
        
        Assert.AreEqual(5, result);
    }

    [Test]
    public void FourTimesTwo_IsEight() 
    {
        var result = Solution.Solve(4);
        
        Assert.AreEqual(8, result);
    }
}

