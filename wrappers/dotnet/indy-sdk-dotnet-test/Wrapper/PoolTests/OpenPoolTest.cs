using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.PoolTests
{
    [TestClass]
    public class OpenPoolTest : IndyIntegrationTestBase
    {
        [TestMethod]
        public async Task TestOpenPoolWorksForNullConfig()
        {
            var poolName = PoolUtils.CreatePoolLedgerConfig();
            var pool = await Pool.OpenPoolLedgerAsync(poolName, null);

            Assert.IsNotNull(pool);

            _openedPools.Add(pool);
        }

        [TestMethod]
        public async Task TestOpenPoolWorksForConfig()
        {
            var poolName = PoolUtils.CreatePoolLedgerConfig();

            var config = "{\"refreshOnOpen\":true,\"autoRefreshTime\":false,\"networkTimeout\":false}";
            var pool = await Pool.OpenPoolLedgerAsync(poolName, config);


            Assert.IsNotNull(pool);
            _openedPools.Add(pool);
        }

        [TestMethod]
        public async Task TestOpenPoolWorksForTwice()
        {
            var poolName = PoolUtils.CreatePoolLedgerConfig();
            var pool = await Pool.OpenPoolLedgerAsync(poolName, null);

            Assert.IsNotNull(pool);
            _openedPools.Add(pool);

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
               Pool.OpenPoolLedgerAsync(poolName, null)
            );

            Assert.AreEqual(ErrorCode.PoolLedgerInvalidPoolHandle, ex.ErrorCode);

        }

        [TestMethod]
        public async Task TestOpenPoolWorksForTwoNodes()
        {
            var poolName = PoolUtils.CreatePoolLedgerConfig(2);

            var pool = await Pool.OpenPoolLedgerAsync(poolName, null);

            Assert.IsNotNull(pool);
            _openedPools.Add(pool);
        }

        [TestMethod]
        public async Task TestOpenPoolWorksForThreeNodes()
        {
            var poolName = PoolUtils.CreatePoolLedgerConfig(3);

            var pool = await Pool.OpenPoolLedgerAsync(poolName, null);

            Assert.IsNotNull(pool);
            _openedPools.Add(pool);
        }
    }
}
