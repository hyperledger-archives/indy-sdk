using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.PoolTests
{
    [TestClass]
    public class ClosePoolTest : IndyIntegrationTest
    {
        [TestMethod]
        public void TestClosePoolWorks()
        {
            var pool = PoolUtils.CreateAndOpenPoolLedger();
            Assert.IsNotNull(pool);
            _openedPools.Add(pool);

            pool.CloseAsync().Wait();
            _openedPools.Remove(pool);
        }

        [TestMethod]
        public async Task TestClosePoolWorksForTwice()
        {
            var pool = PoolUtils.CreateAndOpenPoolLedger();
            Assert.IsNotNull(pool);
            _openedPools.Add(pool);

            pool.CloseAsync().Wait();
            _openedPools.Remove(pool);

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                pool.CloseAsync()
            );

            Assert.AreEqual(ErrorCode.PoolLedgerInvalidPoolHandle, ex.ErrorCode);
        }

        [TestMethod]
        public void TestClosePoolWorksForReopenAfterClose()
        {
            var poolName = PoolUtils.CreatePoolLedgerConfig();
            var pool = Pool.OpenPoolLedgerAsync(poolName, null).Result;

            Assert.IsNotNull(pool);

            pool.CloseAsync().Wait();

            pool = Pool.OpenPoolLedgerAsync(poolName, null).Result;
            _openedPools.Add(pool);
        }
    }
}
