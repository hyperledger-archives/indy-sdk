using Hyperledger.Indy.AnonCredsApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AnonCredsTests
{
    [TestClass]
    public class ProverSearchCredentialsTest : AnonCredsIntegrationTestBase
    {
        [TestMethod]
        public async Task TestProverSearchCredentialsWorksForEmptyFilter()
        {           
            var credentials = await AnonCreds.ProverSearchCredentialsAsync(wallet, "{}");
            Assert.AreEqual(3, credentials.TotalCount);

            var credentialsArray = await credentials.NextAsync(100);
            var jsonArray = JArray.Parse(credentialsArray);

            Assert.AreEqual(3, jsonArray.Count);

            //TODO: Shouldn't there be an explicit close of the credential search here?
        }

        [TestMethod]
        public async Task TestProverSearchCredentialsWorksForInvalidFilterJson()
        {
            var filter = "{\"issuer_id\":1}";
            var ex = await Assert.ThrowsExceptionAsync<WalletInvalidQueryException>(() =>
                AnonCreds.ProverSearchCredentialsAsync(wallet, filter)
            );
        }
    }
}
