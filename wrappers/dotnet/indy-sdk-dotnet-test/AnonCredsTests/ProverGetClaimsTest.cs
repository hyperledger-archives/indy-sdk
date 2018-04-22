using Hyperledger.Indy.AnonCredsApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AnonCredsTests
{
    [TestClass]
    public class ProverGetClaimsTest : AnonCredsIntegrationTestBase
    {
        [TestMethod]
        public async Task TestProverGetClaimsWorksForEmptyFilter()
        {
            await InitCommonWallet();

            var claims = await AnonCreds.ProverGetCredentialsAsync(commonWallet, "{}");

            var claimsArray = JArray.Parse(claims);

            Assert.AreEqual(1, claimsArray.Count);
        }

        [TestMethod]
        public async Task TestProverGetClaimsWorksForFilterByIssuer()
        {
            await InitCommonWallet();

            var filter = string.Format("{{\"issuer_did\":\"{0}\"}}", issuerDid);

            var claims = await AnonCreds.ProverGetCredentialsAsync(commonWallet, filter);

            var claimsArray = JArray.Parse(claims);

            Assert.AreEqual(1, claimsArray.Count);
        }

        [TestMethod]
        public async Task TestProverGetClaimsWorksForFilterByIssuerAndSchema()
        {
            await InitCommonWallet();

            var filter = string.Format("{{\"issuer_did\":\"{0}\", \"schema_seq_no\":{1}}}", issuerDid, 1);

            var claims = await AnonCreds.ProverGetCredentialsAsync(commonWallet, filter);

            var claimsArray = JArray.Parse(claims);

            Assert.AreEqual(1, claimsArray.Count);
        }

        [TestMethod]
        public async Task TestProverGetClaimsWorksForEmptyResult()
        {
            await InitCommonWallet();

            var filter = string.Format("{{\"schema_seq_no\":{0}}}",  10);

            var claims = await AnonCreds.ProverGetCredentialsAsync(commonWallet, filter);

            var claimsArray = JArray.Parse(claims);

            Assert.AreEqual(0, claimsArray.Count);
        }


        [TestMethod]
        public async Task TestProverGetClaimsForProofRequestWorksForInvalidPredicateType()
        {
            await InitCommonWallet();

            var filter = string.Format("{{\"schema_seq_no\":\"{0}\"}}", 1);

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                AnonCreds.ProverGetCredentialsAsync(commonWallet, filter)
            );
        }

    }
}
