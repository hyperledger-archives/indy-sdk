using Hyperledger.Indy.PoolApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.PoolTests
{
    [TestClass]
    public class DisposePoolTest : IndyIntegrationTestBase
    {
        [TestMethod]
        public async Task CanDisposeClosedPool()
        {
            var poolName = PoolUtils.CreatePoolLedgerConfig();

            using (var pool = await Pool.OpenPoolLedgerAsync(poolName, null))
            {
                await pool.CloseAsync();
            }
        }

        [TestMethod]
        public async Task DisposeCanBeCalledRepeatedly()
        {
            var poolName = PoolUtils.CreatePoolLedgerConfig();

            var pool = await Pool.OpenPoolLedgerAsync(poolName, null);
            pool.Dispose();
            pool.Dispose();
        }

        [TestMethod]
        public async Task PoolWithSameNameCanBeOpenedAfterDispose()
        {
            var poolName = PoolUtils.CreatePoolLedgerConfig();

            var pool = await Pool.OpenPoolLedgerAsync(poolName, null);
            pool.Dispose();

            using (var newPool = await Pool.OpenPoolLedgerAsync(poolName, null))
            {
            }
        }

        [TestMethod]
        public async Task ClosingDisposedPoolStillProvidesSDKError()
        {
            var poolName = PoolUtils.CreatePoolLedgerConfig();

            var pool = await Pool.OpenPoolLedgerAsync(poolName, null);
            pool.Dispose();

            var ex = await Assert.ThrowsExceptionAsync<InvalidPoolException>(() =>
                pool.CloseAsync()
            );
        }      
    }
}
