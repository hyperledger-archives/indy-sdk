using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.PoolTests
{
    [TestClass]
    public class DeletePoolTest : IndyIntegrationTestBase
    {
        [TestMethod]
        public void TestDeletePoolWorks()
        {
            var poolName = PoolUtils.CreatePoolLedgerConfig();
            Pool.DeletePoolLedgerConfigAsync(poolName).Wait();
        }

        [TestMethod]
        public async Task TestDeletePoolWorksForOpened()
        {
            var poolName = PoolUtils.CreatePoolLedgerConfig();
            var pool = Pool.OpenPoolLedgerAsync(poolName, null).Result;

            Assert.IsNotNull(pool);
            _openedPools.Add(pool);

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Pool.DeletePoolLedgerConfigAsync(poolName)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidState, ex.ErrorCode);
        }        
    }
}
