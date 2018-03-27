using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.SignusApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.LedgerTests
{
    [TestClass]
    class PoolRestartRequestTest : IndyIntegrationTestWithPoolAndSingleWallet
    {
        [TestMethod]
        public async Task TestBuildPoolRestartRequestWorks()
        {                   
            var expectedResult = string.Format("\"identifier\":\"%s\"," +
                "\"operation\":{\"type\":\"118\"," +
                "\"action\":\"start\"," +
                "\"schedule\":{}", DID1);

            var action = "start";
            var schedule = "{}";
            var poolRestartRequest = await Ledger.BuildPoolRestartRequestAsync(DID1, action, scedule);

            Assert.IsTrue(poolRestartRequest.Replace("\\", "").Contains(expectedResult));
        }
    }
}
