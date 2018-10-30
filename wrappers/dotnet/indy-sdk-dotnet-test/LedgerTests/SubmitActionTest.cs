using Hyperledger.Indy.DidApi;
using Hyperledger.Indy.LedgerApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.LedgerTests
{
    [TestClass]
    public class SubmitActionTest : IndyIntegrationTestWithPoolAndSingleWallet
    {
        private string did;

        [TestInitialize]
        public async Task CreateDid()
        {
            var result = await Did.CreateAndStoreMyDidAsync(wallet, "{}");
            did = result.Did;
        }
        
        [TestMethod]
        public async Task TestSubmitActionWorksForGetValidatorInfo()
        {
            var getValidatorInfoRequest = await Ledger.BuildGetValidatorInfoRequestAsync(did);
            getValidatorInfoRequest = await Ledger.SignRequestAsync(wallet, did, getValidatorInfoRequest);
            await Ledger.SubmitActionAsync(pool, getValidatorInfoRequest, null, -1);
        }

        [TestMethod]
        public async Task TestSubmitActionWorksForPoolRestart()
        {
            var poolRestartRequest = await Ledger.BuildPoolRestartRequestAsync(did, "cancel", null);
            poolRestartRequest = await Ledger.SignRequestAsync(wallet, did, poolRestartRequest);
            await Ledger.SubmitActionAsync(pool, poolRestartRequest, null, -1);

        }

        [TestMethod]
        public async Task TestSubmitActionWorksForNodes()
        {
            var nodes = "[\"Node1\",\"Node2\"]";
            var getValidatorInfoRequest = await Ledger.BuildGetValidatorInfoRequestAsync(did);
            getValidatorInfoRequest = await Ledger.SignRequestAsync(wallet, did, getValidatorInfoRequest);
            var responseJson = await Ledger.SubmitActionAsync(pool, getValidatorInfoRequest, nodes, -1);
            var response = JObject.Parse(responseJson);
            Assert.AreEqual(2, response.Count);
            Assert.IsNotNull(response["Node1"]);
            Assert.IsNotNull(response["Node2"]);
        }
    }
}
