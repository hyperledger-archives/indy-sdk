using Hyperledger.Indy.LedgerApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.LedgerTests
{
    [TestClass]
    public class RevocRegDefRequestTest : LedgerIntegrationTestBase
    {
        [TestMethod]
        public async Task TestBuildRevocRegDefRequestWorks()
        {
            var expectedResult =
                 "\"operation\":{" +
                         "\"type\":\"113\"," +
                         "\"id\":\"RevocRegID\"," +
                         "\"revocDefType\":\"CL_ACCUM\"," +
                         "\"tag\":\"TAG1\"," +
                         "\"credDefId\":\"CredDefID\"," +
                         "\"value\":{" +
                         "\"issuanceType\":\"ISSUANCE_ON_DEMAND\"," +
                         "\"maxCredNum\":5," +
                         "\"publicKeys\":{" +
                         "\"accumKey\":{" +
                         "\"z\":\"1 0000000000000000000000000000000000000000000000000000000000001111 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000\"" +
                         "}" +
                         "}";

            var data = "{\n" +
                    "        \"ver\": \"1.0\",\n" +
                    "        \"id\": \"RevocRegID\",\n" +
                    "        \"revocDefType\": \"CL_ACCUM\",\n" +
                    "        \"tag\": \"TAG1\",\n" +
                    "        \"credDefId\": \"CredDefID\",\n" +
                    "        \"value\": {\n" +
                    "            \"issuanceType\": \"ISSUANCE_ON_DEMAND\",\n" +
                    "            \"maxCredNum\": 5,\n" +
                    "            \"tailsHash\": \"s\",\n" +
                    "            \"tailsLocation\": \"http://tails.location.com\",\n" +
                    "            \"publicKeys\": {\n" +
                    "                \"accumKey\": {\n" +
                    "                    \"z\": \"1 0000000000000000000000000000000000000000000000000000000000001111 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000\"\n" +
                    "                }\n" +
                    "            }\n" +
                    "        }\n" +
                    "    }";

            var request = await Ledger.BuildRevocRegDefRequestAsync(DID, data);
            Assert.IsTrue(request.Replace("\\s+", "").Contains(expectedResult.Replace("\\s+", "")));
        }

        [TestMethod]
        public async Task TestRevocRegRequestsWorks()
        {
            await PostEntitiesAsync();

            var getRevRegDefRequest = await Ledger.BuildGetRevocRegDefRequestAsync(DID, revRegDefId);
            var getRevRegDefResponse = await PoolUtils.EnsurePreviousRequestAppliedAsync(pool, getRevRegDefRequest, response => {
                var responseObject = JObject.Parse(response);
                return responseObject["result"]["seqNo"] != null ;
            });

            await Ledger.ParseGetRevocRegDefResponseAsync(getRevRegDefResponse);
        }
    }
}
