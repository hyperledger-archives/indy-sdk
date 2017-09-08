using Hyperledger.Indy.AnonCredsApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;


namespace Hyperledger.Indy.Test.AnonCredsTests
{
    [TestClass]
    public class ProverCreateAndStoreClaimReqTest : AnonCredsIntegrationTestBase
    {
        [TestMethod]
        public async Task TestProverCreateAndStoreClaimReqWorks()
        {
            await InitCommonWallet();

            var claimOffer = string.Format(_claimOfferTemplate, _issuerDid, 1);

            await AnonCreds.ProverCreateAndStoreClaimReqAsync(_commonWallet, _proverDid, claimOffer, _claimDef, _masterSecretName);
        }

        [TestMethod]
        public async Task TestProverCreateAndStoreClaimReqWorksForClaimDefDoesNotCorrespondToClaimOfferDifferentIssuer()
        {
            await InitCommonWallet();

            var claimOffer = string.Format(_claimOfferTemplate, "acWziYqKpYi6ov5FcYDi1e3", 1);         

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                AnonCreds.ProverCreateAndStoreClaimReqAsync(_commonWallet, _proverDid, claimOffer, _claimDef, _masterSecretName)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestProverCreateAndStoreClaimReqWorksForClaimDefDoesNotCorrespondToClaimOfferDifferentSchema()
        {
            await InitCommonWallet();

            var claimOffer = string.Format(_claimOfferTemplate, _issuerDid, 2);

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                AnonCreds.ProverCreateAndStoreClaimReqAsync(_commonWallet, _proverDid, claimOffer, _claimDef, _masterSecretName)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestProverCreateAndStoreClaimReqWorksForInvalidClaimOffer()
        {
            await InitCommonWallet();

            var claimOffer = string.Format("{{\"issuer_did\":\"{0}\"}}", _issuerDid);

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                AnonCreds.ProverCreateAndStoreClaimReqAsync(_commonWallet, _proverDid, claimOffer, _claimDef, _masterSecretName)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestProverCreateAndStoreClaimReqWorksForInvalidMasterSecret()
        {
            await InitCommonWallet();

            var claimOffer = string.Format(_claimOfferTemplate, _issuerDid, 2);

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                AnonCreds.ProverCreateAndStoreClaimReqAsync(_commonWallet, _proverDid, claimOffer, _claimDef, "other_master_secret")
            );

            Assert.AreEqual(ErrorCode.WalletNotFoundError, ex.ErrorCode);
        }
    }
}
