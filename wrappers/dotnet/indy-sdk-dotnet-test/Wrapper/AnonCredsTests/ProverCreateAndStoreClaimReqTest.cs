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
    public class ProverCreateAndStoreClaimReqTest : AnonCredsIntegrationTestBase
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
        public void TestProverCreateAndStoreClaimReqWorks()
        {
            InitCommonWallet();

            var claimOffer = string.Format(_claimOfferTemplate, _issuerDid, 1);

            AnonCreds.ProverCreateAndStoreClaimReqAsync(_commonWallet, _proverDid, claimOffer, _claimDef, _masterSecretName).Wait();
        }

        [TestMethod]
        public async Task TestProverCreateAndStoreClaimReqWorksForClaimDefDoesNotCorrespondToClaimOfferDifferentIssuer()
        {
            InitCommonWallet();

            var claimOffer = string.Format(_claimOfferTemplate, "acWziYqKpYi6ov5FcYDi1e3", 1);         

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                AnonCreds.ProverCreateAndStoreClaimReqAsync(_commonWallet, _proverDid, claimOffer, _claimDef, _masterSecretName)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestProverCreateAndStoreClaimReqWorksForClaimDefDoesNotCorrespondToClaimOfferDifferentSchema()
        {
            InitCommonWallet();

            var claimOffer = string.Format(_claimOfferTemplate, _issuerDid, 2);

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                AnonCreds.ProverCreateAndStoreClaimReqAsync(_commonWallet, _proverDid, claimOffer, _claimDef, _masterSecretName)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestProverCreateAndStoreClaimReqWorksForInvalidClaimOffer()
        {
            InitCommonWallet();

            var claimOffer = string.Format("{{\"issuer_did\":\"{0}\"}}", _issuerDid);

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                AnonCreds.ProverCreateAndStoreClaimReqAsync(_commonWallet, _proverDid, claimOffer, _claimDef, _masterSecretName)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestProverCreateAndStoreClaimReqWorksForInvalidMasterSecret()
        {
            InitCommonWallet();

            var claimOffer = string.Format(_claimOfferTemplate, _issuerDid, 2);

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                AnonCreds.ProverCreateAndStoreClaimReqAsync(_commonWallet, _proverDid, claimOffer, _claimDef, "other_master_secret")
            );

            Assert.AreEqual(ErrorCode.WalletNotFoundError, ex.ErrorCode);
        }
    }
}
