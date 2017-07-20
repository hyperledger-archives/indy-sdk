using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace Indy.Sdk.Dotnet.Test.Wrapper.PoolTests
{
    [TestClass]
    public class RefreshPoolTest : IndyIntegrationTestBase
    {
        [TestMethod]
        public void TestRefreshPoolWorks()
        {
            var pool = PoolUtils.CreateAndOpenPoolLedger();

            Assert.IsNotNull(pool);
            _openedPools.Add(pool);

            pool.RefreshAsync().Wait();
        }       
    }
}
