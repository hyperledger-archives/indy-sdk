using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.DidApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.LedgerTests
{
    [TestClass]
    public class SchemaRequestTest : IndyIntegrationTestWithPoolAndSingleWallet
    {       
        [TestMethod]
        public async Task TestBuildSchemaRequestWorks()
        {
            var data = "{\"name\":\"name\",\"version\":\"1.0\",\"attr_names\":[\"name\",\"male\"]}";

            var expectedResult = string.Format("\"identifier\":\"{0}\"," +
                    "\"operation\":{{" +
                    "\"type\":\"101\"," +
                    "\"data\":{1}" +
                    "}}", DID1, data);

            var schemaRequest = await Ledger.BuildSchemaRequestAsync(DID1, data);

            Assert.IsTrue(schemaRequest.Replace("\\", "").Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestBuildGetSchemaRequestWorks()
        {
            var data = "{\"name\":\"name\",\"version\":\"1.0\"}";

            var expectedResult = string.Format("\"identifier\":\"{0}\"," +
                    "\"operation\":{{" +
                    "\"type\":\"107\"," +
                    "\"dest\":\"{1}\"," +
                    "\"data\":{2}" +
                    "}}", DID1, DID1, data);

            var getSchemaRequest = await Ledger.BuildGetSchemaRequestAsync(DID1, DID1, data);

            Assert.IsTrue(getSchemaRequest.Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestBuildSchemaRequestWorksWithoutSignature()
        {
            var didResult = await Did.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var did = didResult.Did;

            var schemaRequest = await Ledger.BuildSchemaRequestAsync(did, SCHEMA_DATA);

            var ex = await Assert.ThrowsExceptionAsync<InvalidLedgerTransactionException>(() =>
                Ledger.SubmitRequestAsync(pool, schemaRequest)
            );
        }

        [TestMethod] //Name of this test is bad.
        public async Task TestSchemaRequestWorks()
        {
            var didResult = await Did.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var did = didResult.Did;

            var schemaData = "{\"name\":\"gvt2\",\"version\":\"2.0\",\"attr_names\": [\"name\", \"male\"]}";

            var schemaRequest = await Ledger.BuildSchemaRequestAsync(did, schemaData);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, did, schemaRequest);

            var getSchemaData = "{\"name\":\"gvt2\",\"version\":\"2.0\"}";
            var getSchemaRequest = await Ledger.BuildGetSchemaRequestAsync(did, did, getSchemaData);
            var getSchemaResponse = await Ledger.SubmitRequestAsync(pool, getSchemaRequest);

            var getSchemaResponseObject = JObject.Parse(getSchemaResponse);

            Assert.AreEqual("gvt2", (string)getSchemaResponseObject["result"]["data"]["name"]);
            Assert.AreEqual("2.0", (string)getSchemaResponseObject["result"]["data"]["version"]);           
        }

        [TestMethod]
        public async Task TestGetSchemaRequestsWorksForUnknownSchema()
        {
            var didResult = await Did.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var did = didResult.Did;

            var getSchemaData = "{\"name\":\"schema_name\",\"version\":\"2.0\"}";
            var getSchemaRequest = await Ledger.BuildGetSchemaRequestAsync(did, did, getSchemaData);
            var getSchemaResponse = await Ledger.SubmitRequestAsync(pool, getSchemaRequest);

            var getSchemaResponseObject = JObject.Parse(getSchemaResponse);

            Assert.IsNotNull(getSchemaResponseObject["result"]["data"]);
        }

        [TestMethod]
        public async Task TestBuildSchemaRequestWorksForMissedFields()
        {
            var data = "{\"name\":\"name\",\"version\":\"1.0\"}";

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                Ledger.BuildSchemaRequestAsync(DID1, data)
            );
        }

    }
}
