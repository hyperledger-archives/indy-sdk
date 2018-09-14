using Hyperledger.Indy.LedgerApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.LedgerTests
{
    [TestClass]
    public class PoolRestartRequestTest : IndyIntegrationTestWithPoolAndSingleWallet
    {
        /// <summary>
        /// Marked as ignore, see commented out code.  Havent found a suitable replacement so this test might be obsolete
        /// </summary>
        /// <returns>The build pool restart request works.</returns>
        [TestMethod]
        [Ignore]    
        public async Task TestBuildPoolRestartRequestWorks()
        {
            var expectedResult = string.Format("\"identifier\":\"%s\"," +
                "\"operation\":{\"type\":\"118\"," +
                "\"action\":\"start\"," +
                "\"schedule\":{}", DID1);

            var action = "start";
            var schedule = "{}";
            var poolRestartRequest = ""; //TODO await Ledger.BuildPoolRestartRequestAsync(DID1, action, schedule);

            Assert.IsTrue(poolRestartRequest.Replace("\\", "").Contains(expectedResult));
        }
    }
}
