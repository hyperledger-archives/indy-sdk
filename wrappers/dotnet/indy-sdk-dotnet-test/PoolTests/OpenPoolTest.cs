using Hyperledger.Indy.PoolApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.PoolTests
{
    [TestClass]
    public class OpenPoolTest : IndyIntegrationTestBase
    {
        [TestMethod]
        public async Task TestOpenPoolWorksForNullConfig()
        {
            Pool.SetProtocolVersionAsync(PoolUtils.PROTOCOL_VERSION).Wait();

            var poolName = PoolUtils.CreatePoolLedgerConfig();
            var pool = await Pool.OpenPoolLedgerAsync(poolName, null);

            Assert.IsNotNull(pool);

            openedPools.Add(pool);
        }

        [TestMethod]
        public async Task TestOpenPoolWorksForConfig()
        {
            Pool.SetProtocolVersionAsync(PoolUtils.PROTOCOL_VERSION).Wait();

            var poolName = PoolUtils.CreatePoolLedgerConfig();

            var config = "{\"refresh_on_open\":true,\"auto_refresh_time\":false,\"network_timeout\":false}";
            var pool = await Pool.OpenPoolLedgerAsync(poolName, config);


            Assert.IsNotNull(pool);
            openedPools.Add(pool);
        }

        [TestMethod]
        public async Task TestOpenPoolWorksForTwice()
        {
            Pool.SetProtocolVersionAsync(PoolUtils.PROTOCOL_VERSION).Wait();

            var poolName = PoolUtils.CreatePoolLedgerConfig();
            var pool = await Pool.OpenPoolLedgerAsync(poolName, null);

            Assert.IsNotNull(pool);
            openedPools.Add(pool);

            var ex = await Assert.ThrowsExceptionAsync<InvalidPoolException>(() =>
               Pool.OpenPoolLedgerAsync(poolName, null)
            );
        }

        [TestMethod]
        public async Task TestOpenPoolWorksForTwoNodes()
        {
            Pool.SetProtocolVersionAsync(PoolUtils.PROTOCOL_VERSION).Wait();

            var poolName = PoolUtils.CreatePoolLedgerConfig(2);

            var pool = await Pool.OpenPoolLedgerAsync(poolName, null);

            Assert.IsNotNull(pool);
            openedPools.Add(pool);
        }

        [TestMethod]
        public async Task TestOpenPoolWorksForThreeNodes()
        {
            Pool.SetProtocolVersionAsync(PoolUtils.PROTOCOL_VERSION).Wait();

            var poolName = PoolUtils.CreatePoolLedgerConfig(3);

            var pool = await Pool.OpenPoolLedgerAsync(poolName, null);

            Assert.IsNotNull(pool);
            openedPools.Add(pool);
        }
    }
}
