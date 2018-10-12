using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.DidApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.LedgerTests
{
    [TestClass]
    public class GetRevocRegDefRequestTest : IndyIntegrationTestWithPoolAndSingleWallet
    {
        [TestMethod]
        public async Task TestBuildGetRevocRegDefRequestWorks()
        {
            var expectedResult = "\"operation\":{\"type\":\"115\",\"id\":\"RevocRegID\"}";

            var request = await Ledger.BuildGetRevocRegDefRequestAsync(DID, "RevocRegID");

            Assert.IsTrue(request.Replace("\\s+", "").Contains(expectedResult.Replace("\\s+", "")));
        }
    }
}
