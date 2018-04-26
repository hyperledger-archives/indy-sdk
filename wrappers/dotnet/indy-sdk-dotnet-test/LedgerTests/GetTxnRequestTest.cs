using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.DidApi;
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
            var data = 1;

            var expectedResult = string.Format("\"identifier\":\"{0}\"," +
                    "\"operation\":{{" +
                    "\"type\":\"3\"," +
                    "\"data\":{1}" +
                    "}}", DID1, data);

            var getTxnRequest = await Ledger.BuildGetTxnRequestAsync(DID1, data);

            Assert.IsTrue(getTxnRequest.Replace("\\", "").Contains(expectedResult));
        }

        [TestMethod] //This test fails here and in the Java version.
        public async Task TestGetTxnRequestWorks()
        {
            var didResult = await Did.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var did = didResult.Did;

            var schemaRequest = await Ledger.BuildSchemaRequestAsync(did, SCHEMA_DATA);
            var schemaResponse = await Ledger.SignAndSubmitRequestAsync(pool, wallet, did, schemaRequest);

            var schemaResponseObj = JObject.Parse(schemaResponse);

            var seqNo = schemaResponseObj["result"].Value<int>("seqNo");

            var getTxnRequest = await Ledger.BuildGetTxnRequestAsync(did, seqNo);
            var getTxnResponse = await Ledger.SubmitRequestAsync(pool, getTxnRequest);

            var getTxnResponseObj = JObject.Parse(getTxnResponse);

            var returnedSchemaData = getTxnResponseObj["result"]["data"]["data"];            
            var expectedSchemaData = JToken.Parse(SCHEMA_DATA);

            Assert.IsTrue(JToken.DeepEquals(expectedSchemaData, returnedSchemaData));
        }

        [TestMethod] 
        public async Task TestGetTxnRequestWorksForInvalidSeqNo()
        {
            var didResult = await Did.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var did = didResult.Did;

            var schemaRequest = await Ledger.BuildSchemaRequestAsync(did, SCHEMA_DATA);
            var schemaResponse = await Ledger.SignAndSubmitRequestAsync(pool, wallet, did, schemaRequest);

            var schemaResponseObj = JObject.Parse(schemaResponse);

            var seqNo = (int)schemaResponseObj["result"]["seqNo"] + 1;

            var getTxnRequest = await Ledger.BuildGetTxnRequestAsync(did, seqNo);
            var getTxnResponse = await Ledger.SubmitRequestAsync(pool, getTxnRequest);

            var getTxnResponseObj = JObject.Parse(getTxnResponse);

            Assert.IsFalse(getTxnResponseObj["result"]["data"].HasValues);
        }
    }
}
