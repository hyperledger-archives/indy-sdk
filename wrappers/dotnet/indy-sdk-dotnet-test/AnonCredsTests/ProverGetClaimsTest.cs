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

            var claims = await AnonCreds.ProverGetClaimsAsync(_commonWallet, "{}");

            var claimsArray = JArray.Parse(claims);

            Assert.AreEqual(1, claimsArray.Count);
        }

        [TestMethod]
        public async Task TestProverGetClaimsWorksForFilterByIssuer()
        {
            await InitCommonWallet();

            var filter = string.Format("{{\"issuer_did\":\"{0}\"}}", _issuerDid);

            var claims = await AnonCreds.ProverGetClaimsAsync(_commonWallet, filter);

            var claimsArray = JArray.Parse(claims);

            Assert.AreEqual(1, claimsArray.Count);
        }

        [TestMethod]
        public async Task TestProverGetClaimsWorksForFilterByIssuerAndSchema()
        {
            await InitCommonWallet();

            var filter = string.Format("{{\"issuer_did\":\"{0}\", \"schema_seq_no\":{1}}}", _issuerDid, 1);

            var claims = await AnonCreds.ProverGetClaimsAsync(_commonWallet, filter);

            var claimsArray = JArray.Parse(claims);

            Assert.AreEqual(1, claimsArray.Count);
        }

        [TestMethod]
        public async Task TestProverGetClaimsWorksForEmptyResult()
        {
            await InitCommonWallet();

            var filter = string.Format("{{\"schema_seq_no\":{0}}}",  10);

            var claims = await AnonCreds.ProverGetClaimsAsync(_commonWallet, filter);

            var claimsArray = JArray.Parse(claims);

            Assert.AreEqual(0, claimsArray.Count);
        }


        [TestMethod]
        public async Task TestProverGetClaimsForProofRequestWorksForInvalidPredicateType()
        {
            await InitCommonWallet();

            var filter = string.Format("{{\"schema_seq_no\":\"{0}\"}}", 1);

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                AnonCreds.ProverGetClaimsAsync(_commonWallet, filter)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

    }
}
