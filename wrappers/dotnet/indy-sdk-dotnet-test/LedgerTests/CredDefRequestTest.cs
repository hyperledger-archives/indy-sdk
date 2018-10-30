using Hyperledger.Indy.LedgerApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.LedgerTests
{
    [TestClass]
    public class CredDefRequestTest : LedgerIntegrationTestBase
    {
        [TestMethod]
        public async Task TestBuildCredDefRequestWorks()
        {
            var data = "{\n" +
                "        \"ver\": \"1.0\",\n" +
                "        \"id\": \"cred_def_id\",\n" +
                "        \"schemaId\": \"1\",\n" +
                "        \"type\": \"CL\",\n" +
                "        \"tag\": \"TAG_1\",\n" +
                "        \"value\": {\n" +
                "            \"primary\": {\n" +
                "                \"n\": \"1\",\n" +
                "                \"s\": \"2\",\n" +
                "                \"r\": {\"name\": \"1\",\"master_secret\": \"3\"},\n" +
                "                \"rctxt\": \"1\",\n" +
                "                \"z\": \"1\"\n" +
                "            }\n" +
                "        }\n" +
                "    }";

            var expectedResult = "{\n" +
                    "            \"ref\": 1,\n" +
                    "            \"data\": {\n" +
                    "                \"primary\": {\"n\": \"1\", \"s\": \"2\", \"r\": {\"name\": \"1\",\"master_secret\": \"3\"}, \"rctxt\": \"1\", \"z\": \"1\"}\n" +
                    "            },\n" +
                    "            \"type\": \"102\",\n" +
                    "            \"signature_type\": \"CL\",\n" +
                    "            \"tag\": \"TAG_1\"\n" +
                    "        }";

            var credDefRequest = await Ledger.BuildCredDefRequestAsync(DID, data);

            Assert.IsTrue(JToken.DeepEquals(JObject.Parse(credDefRequest)["operation"], JObject.Parse(expectedResult)));
        }

        [TestMethod]
        public async Task TestBuildGetCredDefRequestWorks()
        {
            var seqNo = 1;
            var id = DID + ":3:" + SIGNATURE_TYPE + ":" + seqNo + ":" + TAG;
            var expectedResult = string.Format("\"identifier\":\"{0}\"," +
                    "\"operation\":{{" +
                    "\"type\":\"108\"," +
                    "\"ref\":{1}," +
                    "\"signature_type\":\"{2}\"," +
                    "\"origin\":\"{3}\"," +
                    "\"tag\":\"{4}\"" +
                    "}}", DID, seqNo, SIGNATURE_TYPE, DID, TAG);

            var getCredDefRequest = await Ledger.BuildGetCredDefRequestAsync(DID, id);

            Assert.IsTrue(getCredDefRequest.Replace("\\", "").Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestBuildCredDefRequestWorksForInvalidJson()
        {
            var data = "{\"primary\":{\"n\":\"1\",\"s\":\"2\",\"rms\":\"3\",\"r\":{\"name\":\"1\"}}}";

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                Ledger.BuildCredDefRequestAsync(DID, data)
            );
        }

        [TestMethod]
        public async Task TestCredDefRequestWorks()
        {
            await PostEntitiesAsync();

            var getCredDefRequest = await Ledger.BuildGetCredDefRequestAsync(DID, credDefId);
            var getCredDefResponse = await PoolUtils.EnsurePreviousRequestAppliedAsync(pool, getCredDefRequest, response => {
                var responseObject = JObject.Parse(response);
                return responseObject["result"]["seqNo"] != null;
            });

            await Ledger.ParseGetCredDefResponseAsync(getCredDefResponse);
        }

    }
}
