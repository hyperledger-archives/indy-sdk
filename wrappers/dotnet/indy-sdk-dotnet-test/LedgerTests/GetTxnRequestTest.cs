using Hyperledger.Indy.LedgerApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.LedgerTests
{
    [TestClass]
    public class GetTxnRequestTest : IndyIntegrationTestWithPoolAndSingleWallet
    {        
        [TestMethod]
        public async Task TestBuildGetTxnRequestWorks()
        {
            var seq_no = 1;

            var expectedResult = string.Format("\"identifier\":\"{0}\"," +
                    "\"operation\":{{" +
                    "\"type\":\"3\"," +
                    "\"data\":{1}," +
                    "\"ledgerId\":1" +
                    "}}", DID, seq_no);

            var getTxnRequest = await Ledger.BuildGetTxnRequestAsync(DID, null, seq_no);

            Assert.IsTrue(getTxnRequest.Replace("\\", "").Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestBuildGetTxnRequestWorksForLedgerType()
        {
            var seq_no = 1;

            var expectedResult = string.Format("\"identifier\":\"{0}\"," +
                    "\"operation\":{{" +
                    "\"type\":\"3\"," +
                    "\"data\":{1}," +
                    "\"ledgerId\":0" +
                    "}}", DID, seq_no);

            var getTxnRequest = await Ledger.BuildGetTxnRequestAsync(DID, "POOL", seq_no);

            Assert.IsTrue(getTxnRequest.Replace("\\", "").Contains(expectedResult));
        }

        [TestMethod] //This test fails here and in the Java version.
        public async Task TestGetTxnRequestWorks()
        {
            var did = await CreateStoreAndPublishDidFromTrusteeAsync();

            var schemaRequest = await Ledger.BuildSchemaRequestAsync(did, SCHEMA_DATA);
            var schemaResponse = await Ledger.SignAndSubmitRequestAsync(pool, wallet, did, schemaRequest);

            var schemaResponseObj = JObject.Parse(schemaResponse);

            var seqNo = schemaResponseObj["result"]["txnMetadata"].Value<int>("seqNo");

            var getTxnRequest = await Ledger.BuildGetTxnRequestAsync(did, null, seqNo);
            var expectedData = "{\"name\":\"gvt\",\"version\":\"1.0\",\"attr_names\": [\"name\"]}";

            var getTxnResponse = await PoolUtils.EnsurePreviousRequestAppliedAsync(pool, getTxnRequest, response => {
                var getTxnResponseObj = JObject.Parse(response);
                var schemaTransactionObj = getTxnResponseObj["result"]["data"]["txn"]["data"]["data"];

                return JValue.DeepEquals(JObject.Parse(expectedData), schemaTransactionObj);
            });

            Assert.IsNotNull(getTxnResponse);
        }

        [TestMethod] 
        public async Task TestGetTxnRequestWorksForInvalidSeqNo()
        {
            var did = await CreateStoreAndPublishDidFromTrusteeAsync();

            var schemaRequest = await Ledger.BuildSchemaRequestAsync(did, SCHEMA_DATA);
            var schemaResponse = await Ledger.SignAndSubmitRequestAsync(pool, wallet, did, schemaRequest);

            var schemaResponseObj = JObject.Parse(schemaResponse);

            var seqNo = (int)schemaResponseObj["result"]["txnMetadata"]["seqNo"] + 1;

            var getTxnRequest = await Ledger.BuildGetTxnRequestAsync(did, null, seqNo);
            var getTxnResponse = await Ledger.SubmitRequestAsync(pool, getTxnRequest);

            var getTxnResponseObj = JObject.Parse(getTxnResponse);

            Assert.IsTrue(getTxnResponseObj["result"]["data"] != null);
        }
    }
}
