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
            var poolName = PoolUtils.CreatePoolLedgerConfig();
            var pool = await Pool.OpenPoolLedgerAsync(poolName, null);

            Assert.IsNotNull(pool);

            openedPools.Add(pool);
        }

        [TestMethod]
        public async Task TestOpenPoolWorksForConfig()
        {
            var poolName = PoolUtils.CreatePoolLedgerConfig();

            var config = "{\"timeout\":20,\"extended_timeout\":80}";
            var pool = await Pool.OpenPoolLedgerAsync(poolName, config);


            Assert.IsNotNull(pool);
            openedPools.Add(pool);
        }

        [TestMethod]
        public async Task TestOpenPoolWorksForTwice()
        {
            var poolName = PoolUtils.CreatePoolLedgerConfig();
            var pool = await Pool.OpenPoolLedgerAsync(poolName, null);

            Assert.IsNotNull(pool);
            openedPools.Add(pool);

            var ex = await Assert.ThrowsExceptionAsync<InvalidPoolException>(() =>
               Pool.OpenPoolLedgerAsync(poolName, null)
            );
        }

        [TestMethod]
        public async Task TestOpenPoolWorksForIncompatibleProtocolVersion()
        {
            await Pool.SetProtocolVersionAsync(PROTOCOL_VERSION);
            var poolName = PoolUtils.CreatePoolLedgerConfig();
            var pool = await Pool.OpenPoolLedgerAsync(poolName, null);

            Assert.IsNotNull(pool);
            openedPools.Add(pool);

            var ex = await Assert.ThrowsExceptionAsync<PoolIncompatibleProtocolVersionException>(() =>
               Pool.SetProtocolVersionAsync(10)
            );
        }
    }
}
