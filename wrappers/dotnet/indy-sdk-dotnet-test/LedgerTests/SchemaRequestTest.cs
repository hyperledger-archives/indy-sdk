using Hyperledger.Indy.LedgerApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.LedgerTests
{
    [TestClass]
    public class SchemaRequestTest : LedgerIntegrationTestBase
    {
        [TestMethod]
        public async Task TestBuildSchemaRequestWorks()
        {
            var expectedResult = "\"operation\":{" +
                "\"type\":\"101\"," +
                "\"data\":{\"name\":\"gvt\",\"version\":\"1.0\",\"attr_names\":[\"name\"]}" +
                "}";

            var schemaRequest = await Ledger.BuildSchemaRequestAsync(DID, SCHEMA_DATA);

            Assert.IsTrue(schemaRequest.Replace("\\s+", "").Contains(expectedResult.Replace("\\s+", "")));
        }

        [TestMethod]
        public async Task TestBuildGetSchemaRequestWorks()
        {
            var id = string.Format("{0}:1:{1}:{2}", DID, GVT_SCHEMA_NAME, SCHEMA_VERSION);

            var expectedResult = "\"operation\":{\"type\":\"107\",\"dest\":\"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW\",\"data\":{\"name\":\"gvt\",\"version\":\"1.0\"}}";

            var getSchemaRequest = await Ledger.BuildGetSchemaRequestAsync(DID, id);

            Assert.IsTrue(getSchemaRequest.Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestBuildSchemaRequestWorksWithoutSignature()
        {
            var schemaRequest = await Ledger.BuildSchemaRequestAsync(DID, SCHEMA_DATA);
            var response = await Ledger.SubmitRequestAsync(pool, schemaRequest);
            CheckResponseType(response, "REQNACK");
        }

        [TestMethod]
        public async Task TestSchemaRequestWorks()
        {
            await PostEntitiesAsync();

            var getSchemaRequest = await Ledger.BuildGetSchemaRequestAsync(DID, schemaId);
            var getSchemaResponse = await PoolUtils.EnsurePreviousRequestAppliedAsync(pool, getSchemaRequest, response => {
                var getSchemaResponseObject = JObject.Parse(response);
                return getSchemaResponseObject["result"]["seqNo"] != null;
            });

            await Ledger.ParseGetSchemaResponseAsync(getSchemaResponse);
        }

        [TestMethod]
        public async Task TestBuildSchemaRequestWorksForMissedFields()
        {
            var data = "{\"name\":\"name\",\"version\":\"1.0\"}";

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                Ledger.BuildSchemaRequestAsync(DID, data)
            );
        }

    }
}
