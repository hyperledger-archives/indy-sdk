using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System;
using System.Threading.Tasks;


namespace Indy.Sdk.Dotnet.Test.Wrapper.AnonCredsTests
{
    [TestClass]
    public class ProverGetClaimsTest : AnonCredsIntegrationTestBase
    {
        [ClassCleanup]
        public static void CloseCommonWallet()
        {
            if (_commonWallet != null)
                _commonWallet.CloseAsync().Wait();
        }

        [TestMethod]
        public void TestProverGetClaimsWorksForEmptyFilter()
        {
            InitCommonWallet();

            var claims = AnonCreds.ProverGetClaimsAsync(_commonWallet, "{}").Result;

            var claimsArray = JArray.Parse(claims);

            Assert.AreEqual(1, claimsArray.Count);
        }

        [TestMethod]
        public void TestProverGetClaimsWorksForFilterByIssuer()
        {
            InitCommonWallet();

            var filter = string.Format("{{\"issuer_did\":\"{0}\"}}", _issuerDid);

            var claims = AnonCreds.ProverGetClaimsAsync(_commonWallet, filter).Result;

            var claimsArray = JArray.Parse(claims);

            Assert.AreEqual(1, claimsArray.Count);
        }

        [TestMethod]
        public void TestProverGetClaimsWorksForFilterByIssuerAndSchema()
        {
            InitCommonWallet();

            var filter = string.Format("{{\"issuer_did\":\"{0}\", \"schema_seq_no\":{1}}}", _issuerDid, 1);

            var claims = AnonCreds.ProverGetClaimsAsync(_commonWallet, filter).Result;

            var claimsArray = JArray.Parse(claims);

            Assert.AreEqual(1, claimsArray.Count);
        }

        [TestMethod]
        public void TestProverGetClaimsWorksForEmptyResult()
        {
            InitCommonWallet();

            var filter = string.Format("{{\"schema_seq_no\":{0}}}",  10);

            var claims = AnonCreds.ProverGetClaimsAsync(_commonWallet, filter).Result;

            var claimsArray = JArray.Parse(claims);

            Assert.AreEqual(0, claimsArray.Count);
        }


        [TestMethod]
        public async Task TestProverGetClaimsForProofRequestWorksForInvalidPredicateType()
        {
            InitCommonWallet();

            var filter = string.Format("{{\"schema_seq_no\":\"{0}\"}}", 1);

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                AnonCreds.ProverGetClaimsAsync(_commonWallet, filter)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

    }
}
