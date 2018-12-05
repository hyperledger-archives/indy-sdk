using Hyperledger.Indy.LedgerApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.LedgerTests
{
    [TestClass]
    public class PoolRestartRequestTest : IndyIntegrationTestWithPoolAndSingleWallet
    {
        [TestMethod]
        public async Task TestBuildPoolRestartRequestWorksForStartAction()
        {
            var expectedResult = string.Format("\"identifier\":\"{0}\"," +
                "\"operation\":{{\"type\":\"118\"," +
                "\"action\":\"start\"," +
                "\"datetime\":\"0\"", DID);

            var request = await Ledger.BuildPoolRestartRequestAsync(DID, "start", "0");

            Assert.IsTrue(request.Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestBuildPoolRestartRequestWorksForCancelAction()
        {
            var expectedResult = string.Format("\"identifier\":\"{0}\"," +
                "\"operation\":{{\"type\":\"118\"," +
                "\"action\":\"cancel\"", DID);

            var request = await Ledger.BuildPoolRestartRequestAsync(DID, "cancel", null);

            Assert.IsTrue(request.Contains(expectedResult));
        }
    }
}
