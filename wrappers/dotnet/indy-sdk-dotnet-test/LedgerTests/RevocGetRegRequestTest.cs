using Hyperledger.Indy.LedgerApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.LedgerTests
{
    [TestClass]
    public class RevocGetRegRequestTest : LedgerIntegrationTestBase
    {
        [TestMethod]
        public async Task TestBuildGetRevocRegRequestWorks()
        {
            var expectedResult =
                "\"operation\":{" +
                "\"type\":\"116\"," +
                "\"revocRegDefId\":\"RevocRegID\"," +
                "\"timestamp\":100" +
                "}";

            var request = await Ledger.BuildGetRevocRegRequestAsync(DID, "RevocRegID", 100);

            Assert.IsTrue(request.Replace("\\s+", "").Contains(expectedResult.Replace("\\s+", "")));
        }

        [TestMethod]
        public async Task TestRevocRegRequestsWorks()
        {
            await PostEntitiesAsync();

            var timespan = (DateTime.UtcNow - new DateTime(1970, 1, 1, 0, 0, 0, DateTimeKind.Utc));
            var timestamp = (int)timespan.TotalSeconds + 100;

            var getRevRegRequest = await Ledger.BuildGetRevocRegRequestAsync(DID, revRegDefId, timestamp);
            var getRevRegResponse = await PoolUtils.EnsurePreviousRequestAppliedAsync(pool, getRevRegRequest, response => {
                var responseObject = JObject.Parse(response);
                return responseObject["result"]["seqNo"] != null;
            });

            await Ledger.ParseGetRevocRegResponseAsync(getRevRegResponse);
        }
    }
}
