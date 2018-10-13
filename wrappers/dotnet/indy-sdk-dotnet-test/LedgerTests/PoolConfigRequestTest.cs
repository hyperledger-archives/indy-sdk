using Hyperledger.Indy.DidApi;
using Hyperledger.Indy.LedgerApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.LedgerTests
{
    [TestClass]
    public class PoolConfigRequestTest : IndyIntegrationTestWithPoolAndSingleWallet
    {
        [TestMethod]
        public async Task TestBuildPoolConfigRequestWorks()
        {
            var expectedResult = string.Format("\"identifier\":\"{0}\"," +
                "\"operation\":{{" +
                "\"type\":\"111\"," +
                "\"writes\":false," +
                "\"force\":false" +
                "}}", DID);

            var request = await Ledger.BuildPoolConfigRequestAsync(DID, false, false);

            Assert.IsTrue(request.Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestPoolConfigRequestWorks()
        {
            var didResult = await Did.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var did = didResult.Did;

            var request = await Ledger.BuildPoolConfigRequestAsync(did, false, false);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, did, request);

            request = await Ledger.BuildPoolConfigRequestAsync(did, true, false);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, did, request);
        }
    }
}
