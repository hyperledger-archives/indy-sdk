using Hyperledger.Indy.LedgerApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.LedgerTests
{
    [TestClass]
    public class RevocRegEntryRequestTest : LedgerIntegrationTestBase
    {
        [TestMethod]
        public async Task TestBuildRevocRegEntryRequestWorks()
        {
            var expectedResult =
                "\"operation\":{" +
                        "\"type\":\"114\"," +
                        "\"revocRegDefId\":\"RevocRegID\"," +
                        "\"revocDefType\":\"CL_ACCUM\"," +
                        "\"value\":{\"accum\":\"false 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\"}" +
                        "}";

           var value = "{\"ver\":\"1.0\"," +
                "        \"value\": {\n" +
                "            \"accum\": \"false 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\"\n" +
                "        }}";

            var request = await Ledger.BuildRevocRegEntryRequestAsync(DID, "RevocRegID", "CL_ACCUM", value);
            Assert.IsTrue(request.Replace("\\s+", "").Contains(expectedResult.Replace("\\s+", "")));
        }
    }
}
