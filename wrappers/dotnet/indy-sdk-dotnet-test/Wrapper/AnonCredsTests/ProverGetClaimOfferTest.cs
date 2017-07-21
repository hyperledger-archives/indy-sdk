using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.Wrapper.Agent;
using static Indy.Sdk.Dotnet.Wrapper.AgentObservers;

namespace Indy.Sdk.Dotnet.Test.Wrapper.AnonCredsTests
{
    [TestClass]
    public class ProverGetClaimOfferTest : AnonCredsIntegrationTestBase
    {
        [ClassCleanup]
        public static void CloseCommonWallet()
        {
            try
            {
                _commonWallet.CloseAsync().Wait();
            }
            catch (Exception)
            { }

        }

        [TestMethod]
        public void TestsProverGetClaimOffersWorksForEmptyFilter()
        {
            InitCommonWallet();

            var claimOffers = AnonCreds.ProverGetClaimOffersAsync(_commonWallet, "{}").Result;
            var claimOffersArray = JArray.Parse(claimOffers);

            Assert.AreEqual(3, claimOffersArray.Count);
        }

        [TestMethod] //Returned data formatting does not match asserts.
        public void TestsProverGetClaimOffersWorksForFilterByIssuer()
        {
            InitCommonWallet();

            var filter = string.Format("{{\"issuer_did\":\"{0}\"}}", _issuerDid);

            var claimOffers = AnonCreds.ProverGetClaimOffersAsync(_commonWallet, filter).Result;
            var claimOffersArray = JArray.Parse(claimOffers);

            Assert.AreEqual(2, claimOffersArray.Count);

            Assert.IsTrue(claimOffersArray.ToString().Contains(string.Format(_claimOfferTemplate, _issuerDid, 1)));
            Assert.IsTrue(claimOffersArray.ToString().Contains(string.Format(_claimOfferTemplate, _issuerDid, 2)));
        }

        [TestMethod] //Returned data formatting does not match asserts.
        public void TestsProverGetClaimOffersWorksForFilterBySchema()
        {
            InitCommonWallet();

            var filter = string.Format("{{\"schema_seq_no\":\"{0}\"}}", _issuerDid);

            var claimOffers = AnonCreds.ProverGetClaimOffersAsync(_commonWallet, filter).Result;
            var claimOffersArray = JArray.Parse(claimOffers);

            Assert.AreEqual(2, claimOffersArray.Count);

            Assert.IsTrue(claimOffersArray.ToString().Contains(string.Format(_claimOfferTemplate, _issuerDid, 1)));
            Assert.IsTrue(claimOffersArray.ToString().Contains(string.Format(_claimOfferTemplate, _issuerDid2, 2)));
        }

        [TestMethod] //Returned data formatting does not match asserts.
        public void TestsProverGetClaimOffersWorksForFilterByIssuerAndSchema()
        {
            InitCommonWallet();

            var filter = string.Format("{{\"issuer_did\":\"{0}\",\"schema_seq_no\":{1}}}", _issuerDid, 1);

            var claimOffers = AnonCreds.ProverGetClaimOffersAsync(_commonWallet, filter).Result;
            var claimOffersArray = JArray.Parse(claimOffers);

            Assert.AreEqual(1, claimOffersArray.Count);

            Assert.IsTrue(claimOffersArray.ToString().Contains(string.Format(_claimOfferTemplate, _issuerDid, 1)));
        }

        [TestMethod]
        public void TestsProverGetClaimOffersWorksForNoResult()
        {
            InitCommonWallet();

            var filter = string.Format("{{\"schema_seq_no\":{0}}}", 3);

            var claimOffers = AnonCreds.ProverGetClaimOffersAsync(_commonWallet, filter).Result;
            var claimOffersArray = JArray.Parse(claimOffers);

            Assert.AreEqual(0, claimOffersArray.Count);
        }

        [TestMethod]
        public async Task TestProverCreateProofWorksForInvalidRequestedClaimsJson()
        {
            InitCommonWallet();

            var filter = string.Format("{{\"schema_seq_no\":\"{0}\"}}", 1);

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                AnonCreds.ProverGetClaimOffersAsync(_commonWallet, filter)

            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

    }
}
