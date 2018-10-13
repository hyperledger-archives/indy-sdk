using Hyperledger.Indy.DidApi;
using Hyperledger.Indy.LedgerApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.LedgerTests
{
    [TestClass]
    public class GetValidatorInfoRequestTest : IndyIntegrationTestWithPoolAndSingleWallet
    {        
        [TestMethod]
        public async Task TestBuildGetValidatorInfoRequestWorks()
        {
            var expectedResult = "" +
                 "\"operation\":{" +
                 "\"type\":\"119\"" +
                 "}";

            var getValidatorInfoRequest = await Ledger.BuildGetValidatorInfoRequestAsync(DID);

            Assert.IsTrue(getValidatorInfoRequest.Replace("\\", "").Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestGetValidatorInfoRequestWorks()
        {
            var didJson = JsonConvert.SerializeObject(new { seed = TRUSTEE_SEED });
            var result = await Did.CreateAndStoreMyDidAsync(wallet, didJson);
            var did = result.Did;

            var getValidatorInfoRequest = await Ledger.BuildGetValidatorInfoRequestAsync(did);
            var getValidatorInfoResponse = await Ledger.SignAndSubmitRequestAsync(pool, wallet, did, getValidatorInfoRequest);

            var getValidatorInfoObj = JObject.Parse(getValidatorInfoResponse);

            for (int i = 1; i <= 4; i++)
            {
                var nodeName = string.Format("Node{0}", i);
                var nodeObject = JObject.Parse(getValidatorInfoObj[nodeName].ToString());
                Assert.IsFalse(nodeObject["result"]["data"] == null);
            }
        }
    }
}
