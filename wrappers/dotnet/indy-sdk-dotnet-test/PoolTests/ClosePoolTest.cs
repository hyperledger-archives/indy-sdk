using Hyperledger.Indy.PoolApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.PoolTests
{
    [TestClass]
    public class ClosePoolTest : IndyIntegrationTestBase
    {
        [TestMethod]
        public async Task TestClosePoolWorks()
        {
            var pool = await PoolUtils.CreateAndOpenPoolLedgerAsync();
            Assert.IsNotNull(pool);
            _openedPools.Add(pool);

            await pool.CloseAsync();
            _openedPools.Remove(pool);
        }

        [TestMethod]
        public async Task TestClosePoolWorksForTwice()
        {
            var pool = await PoolUtils.CreateAndOpenPoolLedgerAsync();
            Assert.IsNotNull(pool);
            _openedPools.Add(pool);

            await pool.CloseAsync();
            _openedPools.Remove(pool);

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                pool.CloseAsync()
            );

            Assert.AreEqual(ErrorCode.PoolLedgerInvalidPoolHandle, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestClosePoolWorksForReopenAfterClose()
        {
            var poolName = PoolUtils.CreatePoolLedgerConfig();
            var pool = await Pool.OpenPoolLedgerAsync(poolName, null);

            Assert.IsNotNull(pool);

            await pool.CloseAsync();

            pool = await Pool.OpenPoolLedgerAsync(poolName, null);
            _openedPools.Add(pool);
        }
    }
}
