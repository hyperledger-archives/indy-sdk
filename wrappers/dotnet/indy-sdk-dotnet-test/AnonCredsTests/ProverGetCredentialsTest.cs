using Hyperledger.Indy.AnonCredsApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AnonCredsTests
{
    [TestClass]
    public class ProverGetCredentialsTest : AnonCredsIntegrationTestBase
    {
        [TestMethod]
        public async Task TestProverGetCredentialsWorksForEmptyFilter()
        {
            var credentials = await AnonCreds.ProverGetCredentialsAsync(wallet, "{}");

            var credentialsArray = JArray.Parse(credentials);

            Assert.AreEqual(3, credentialsArray.Count);
        }

        [TestMethod]
        public async Task TestProverGetCredentialsWorksForFilterByIssuer()
        {
            var filter = string.Format("{{\"issuer_did\":\"{0}\"}}", issuerDid);

            var credentials = await AnonCreds.ProverGetCredentialsAsync(wallet, filter);

            var credentialsArray = JArray.Parse(credentials);

            Assert.AreEqual(2, credentialsArray.Count);
        }

        [TestMethod]
        public async Task TestProverGetCredentialsWorksForFilterBySchema()
        {
            var filter = string.Format("{{\"schema_id\":\"{0}\"}}", gvtSchemaId);

            var credentials = await AnonCreds.ProverGetCredentialsAsync(wallet, filter);

            var credentialsArray = JArray.Parse(credentials);

            Assert.AreEqual(2, credentialsArray.Count);
        }

        [TestMethod]
        public async Task TestProverGetCredentialsWorksForFilterBySchemaName()
        {
            var filter = "{\"schema_name\":\"gvt\"}";

            var credentials = await AnonCreds.ProverGetCredentialsAsync(wallet, filter);

            var credentialsArray = JArray.Parse(credentials);

            Assert.AreEqual(2, credentialsArray.Count);
        }


        [TestMethod]
        public async Task TestProverGetCredentialsWorksForFilterByCredDefId()
        {
            var filter = string.Format("{{\"cred_def_id\":\"{0}\"}}", issuer1gvtCredDefId);

            var credentials = await AnonCreds.ProverGetCredentialsAsync(wallet, filter);

            var credentialsArray = JArray.Parse(credentials);

            Assert.AreEqual(1, credentialsArray.Count);
        }

        [TestMethod]
        public async Task TestProverGetCredentialsWorksForEmptyResult()
        {
            var filter = string.Format("{{\"issuer_id\":\"{0}a\"}}",  issuerDid);

            var credentials = await AnonCreds.ProverGetCredentialsAsync(wallet, filter);

            var credentialsArray = JArray.Parse(credentials);

            Assert.AreEqual(0, credentialsArray.Count);
        }


        [TestMethod]
        public async Task TestProverGetCredentialsWorksForInvalidFilterJson()
        {
            var filter = "{\"issuer_id\":1}";

            var ex = await Assert.ThrowsExceptionAsync<WalletInvalidQueryException>(() =>
                AnonCreds.ProverGetCredentialsAsync(wallet, filter)
            );
        }

    }
}
