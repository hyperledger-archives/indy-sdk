using Hyperledger.Indy.PoolApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.PoolTests
{
    [TestClass]
    public class ClosePoolTest : IndyIntegrationTestBase
    {

        [TestInitialize]
        public async Task SetProtocolVersion()
        {
            Pool.SetProtocolVersionAsync(PoolUtils.PROTOCOL_VERSION).Wait();
        }

        [TestMethod]
        public async Task TestClosePoolWorks()
        {
            var pool = await PoolUtils.CreateAndOpenPoolLedgerAsync();
            Assert.IsNotNull(pool);
            openedPools.Add(pool);

            await pool.CloseAsync();
            openedPools.Remove(pool);
        }

        [TestMethod]
        public async Task TestClosePoolWorksForTwice()
        {
            var pool = await PoolUtils.CreateAndOpenPoolLedgerAsync();
            Assert.IsNotNull(pool);
            openedPools.Add(pool);

            await pool.CloseAsync();
            openedPools.Remove(pool);

            var ex = await Assert.ThrowsExceptionAsync<InvalidPoolException>(() =>
                pool.CloseAsync()
            );
        }

        [TestMethod]
        public async Task TestClosePoolWorksForReopenAfterClose()
        {
            var poolName = PoolUtils.CreatePoolLedgerConfig();
            var pool = await Pool.OpenPoolLedgerAsync(poolName, null);

            Assert.IsNotNull(pool);

            await pool.CloseAsync();

            pool = await Pool.OpenPoolLedgerAsync(poolName, null);
            openedPools.Add(pool);
        }
    }
}
