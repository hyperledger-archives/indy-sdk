using Hyperledger.Indy.Test.Util;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.PoolTests
{
    [TestClass]
    public class RefreshPoolTest : IndyIntegrationTestBase
    {
        [TestMethod]
        public async Task TestRefreshPoolWorks()
        {
            var pool = await PoolUtils.CreateAndOpenPoolLedgerAsync();

            Assert.IsNotNull(pool);
            _openedPools.Add(pool);

            await pool.RefreshAsync();
        }       
    }
}
