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
        /// <summary>
        /// format of schema request is  {did}:2:{name}:{version}
        /// </summary>
        /// <returns>The build get schema request object.</returns>
        /// <param name="did">Did.</param>
        /// <param name="schemaName">Schema name.</param>
        private string CreateBuildGetSchemaRequestObject(string did,  string schemaName)
        {
            return string.Format("{0}:2:{1}:1.0", did, schemaName);
        }

        [TestMethod]
        public async Task TestBuildSchemaRequestWorks()
        {

            var data = "{\"id\":\"id\",\"attrNames\":[\"user_name\",\"sex\"],\"name\":\"test_schema_works_1\",\"version\":\"1.0\",\"ver\":\"1.0\"}";

            var schemaResponse = await Ledger.BuildSchemaRequestAsync(DID1, data);

            /*
             *  Here's the response back.  We can test 
                {
                    "reqId":1536942407004916000,
                    "identifier":"8wZcEriaNLNKtteJvx7f8i",
                    "operation":
                    {
                        "type":"101",
                        "data":
                        {
                             "name":"test_schema_works_1",
                             "version":"1.0",
                             "attr_names":["user_name","sex"]
                        }
                    },
                    "protocolVersion":2
                }
             */

            var schemaObject = JObject.Parse(schemaResponse);

            Assert.AreEqual(DID1, schemaObject["identifier"]);
            Assert.AreEqual("101", schemaObject["operation"]["type"], "failed to traverse {0}", schemaObject["operation"]);
            Assert.AreEqual("test_schema_works_1", schemaObject["operation"]["data"]["name"], "failed to traverse {0}", schemaObject["operation"]);
        }

        [TestMethod]
        public async Task TestBuildGetSchemaRequestWorks()
        {
            var data = CreateBuildGetSchemaRequestObject(DID1, "test_schema_works_1");

            var getSchemaRequestResponse = await Ledger.BuildGetSchemaRequestAsync(DID1, data);

            /*
             *  Response from BuildGetSchemaRequestAsync looks like this
                {
                    "reqId":1536944004614470000,
                    "identifier":"8wZcEriaNLNKtteJvx7f8i",
                    "operation":
                    {
                        "type":"107",
                        "dest":"8wZcEriaNLNKtteJvx7f8i",
                        "data":
                        {
                            "name":"test_schema_works_1",
                            "version":"1.0"
                        }
                    },
                    "protocolVersion":2
                }
             */

            var schemaObject = JObject.Parse(getSchemaRequestResponse);
            Assert.AreEqual(DID1, schemaObject["identifier"]);
            Assert.AreEqual("107", schemaObject["operation"]["type"], "failed to traverse {0}", schemaObject["operation"]);
            Assert.AreEqual("test_schema_works_1", schemaObject["operation"]["data"]["name"], "failed to traverse {0}", schemaObject["operation"]);
        }

        [TestMethod]
        public async Task TestBuildSchemaRequestWorksWithoutSignature()
        {
            var didResult = await Did.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var did = didResult.Did;
            var schemaRequest = await Ledger.BuildSchemaRequestAsync(did, SCHEMA_DATA);

            var submitRequestResponse = await Ledger.SubmitRequestAsync(pool, schemaRequest);

            /*  SubmitRequestAsync will return error response as follows
                {
                    "reason":"client request invalid: MissingSignature()",
                    "op":"REQNACK",
                    "reqId":1536945730478714000,
                    "identifier":"V4SGRU86Z58d6TV7PBUe6f"
                }
             */

            Assert.IsTrue(submitRequestResponse.Contains("\"op\":\"REQNACK\""));
            Assert.IsTrue(submitRequestResponse.Contains("MissingSignature()"));
        }

        /// <summary>
        /// Not really sure the value of the test since other test methods in this test class do the same thing
        /// </summary>
        /// <returns>The schema request works.</returns>
        [TestMethod] 
        public async Task TestSchemaRequestWorks()
        {
            string schemaName = "gvt2";
            var didResult = await Did.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var did = didResult.Did;

            var schemaData = "{\"id\":\"id\",\"attrNames\": [\"name\", \"male\"],\"name\":\"gvt2\",\"version\":\"2.0\",\"ver\":\"1.0\"}";

            var schemaRequest = await Ledger.BuildSchemaRequestAsync(did, schemaData);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, did, schemaRequest);

            var getSchemaData = CreateBuildGetSchemaRequestObject(did, schemaName);
            var getSchemaRequest = await Ledger.BuildGetSchemaRequestAsync(did, getSchemaData);
            var getSchemaResponse = await Ledger.SubmitRequestAsync(pool, getSchemaRequest);

            var getSchemaResponseObject = JObject.Parse(getSchemaResponse);


            Assert.AreEqual(schemaName, (string)getSchemaResponseObject["result"]["data"]["name"]);
            Assert.AreEqual("1.0", (string)getSchemaResponseObject["result"]["data"]["version"]);           
        }

        [TestMethod]
        public async Task TestGetSchemaRequestsWorksForUnknownSchema()
        {
            var didResult = await Did.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var did = didResult.Did;

            var getSchemaData = CreateBuildGetSchemaRequestObject(did, "schema_name");
            var getSchemaRequest = await Ledger.BuildGetSchemaRequestAsync(did, getSchemaData);
            var getSchemaResponse = await Ledger.SubmitRequestAsync(pool, getSchemaRequest);

            var getSchemaResponseObject = JObject.Parse(getSchemaResponse);

            Assert.IsNotNull(getSchemaResponseObject["result"]["data"]);
        }

        [TestMethod]
        public async Task TestBuildSchemaRequestInvalidStructureExceptionForMissedFields()
        {
            var data = "{\"name\":\"name\",\"version\":\"1.0\"}";

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                Ledger.BuildSchemaRequestAsync(DID1, data)
            );
        }

    }
}
