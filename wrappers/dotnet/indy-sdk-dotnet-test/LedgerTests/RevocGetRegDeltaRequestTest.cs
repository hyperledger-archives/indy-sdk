using Hyperledger.Indy.LedgerApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.LedgerTests
{
    [TestClass]
    public class RevocGetRegDeltaRequestTest : LedgerIntegrationTestBase
    {
        [TestMethod]
        public async Task TestSubmitRequestWorks()
        {
            var expectedResult =
                "\"operation\":{" +
                        "\"type\":\"117\"," +
                        "\"revocRegDefId\":\"RevocRegID\"," +
                        "\"to\":100" +
                        "}";

            var request = await Ledger.BuildGetRevocRegDeltaRequestAsync(DID, "RevocRegID", -1, 100);

            Assert.IsTrue(request.Replace("\\s+", "").Contains(expectedResult.Replace("\\s+", "")));
        }

        [TestMethod]
        public async Task TestRevocRegRequestsDeltaWorks()
        {
            await PostEntitiesAsync();

            var timespan = (DateTime.UtcNow - new DateTime(1970, 1, 1, 0, 0, 0, DateTimeKind.Utc));
            var to = (long)timespan.TotalSeconds + 100;

            var getRevRegRequest = await Ledger.BuildGetRevocRegDeltaRequestAsync(DID, revRegDefId, -1, to);
            var getRevRegResponse = await PoolUtils.EnsurePreviousRequestAppliedAsync(pool, getRevRegRequest, response => {
                var responseObject = JObject.Parse(response);
                return responseObject["result"]["seqNo"] != null;
            });

            await Ledger.ParseGetRevocRegDeltaResponseAsync(getRevRegResponse);
        }
    }
}
