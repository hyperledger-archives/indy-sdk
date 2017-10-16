using Hyperledger.Indy.PoolApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.PoolTests
{
    [TestClass]
    public class DeletePoolTest : IndyIntegrationTestBase
    {
        [TestMethod]
        public async Task TestDeletePoolWorks()
        {
            var poolName = PoolUtils.CreatePoolLedgerConfig();
            await Pool.DeletePoolLedgerConfigAsync(poolName);
        }

        [TestMethod]
        public async Task TestDeletePoolWorksForOpened()
        {
            var poolName = PoolUtils.CreatePoolLedgerConfig();
            var pool = await Pool.OpenPoolLedgerAsync(poolName, null);

            Assert.IsNotNull(pool);
            openedPools.Add(pool);

            var ex = await Assert.ThrowsExceptionAsync<InvalidStateException>(() =>
                Pool.DeletePoolLedgerConfigAsync(poolName)
            );
        }        
    }
}
