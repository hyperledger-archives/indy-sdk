using Hyperledger.Indy.AnonCredsApi;
using Hyperledger.Indy.WalletApi;
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

            var claimOffer = string.Format(claimOfferTemplate, issuerDid, 1);

            await AnonCreds.ProverCreateCredentialReqAsync(commonWallet, proverDid, claimOffer, claimDef, masterSecretName);
        }

        [TestMethod]
        public async Task TestProverCreateAndStoreClaimReqWorksForClaimDefDoesNotCorrespondToClaimOfferDifferentIssuer()
        {
            await InitCommonWallet();

            var claimOffer = string.Format(claimOfferTemplate, "acWziYqKpYi6ov5FcYDi1e3", 1);         

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                AnonCreds.ProverCreateCredentialReqAsync(commonWallet, proverDid, claimOffer, claimDef, masterSecretName)
            );
        }

        [TestMethod]
        public async Task TestProverCreateAndStoreClaimReqWorksForClaimDefDoesNotCorrespondToClaimOfferDifferentSchema()
        {
            await InitCommonWallet();

            var claimOffer = string.Format(claimOfferTemplate, issuerDid, 2);

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                AnonCreds.ProverCreateCredentialReqAsync(commonWallet, proverDid, claimOffer, claimDef, masterSecretName)
            );
        }

        [TestMethod]
        public async Task TestProverCreateAndStoreClaimReqWorksForInvalidClaimOffer()
        {
            await InitCommonWallet();

            var claimOffer = string.Format("{{\"issuer_did\":\"{0}\"}}", issuerDid);

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                AnonCreds.ProverCreateCredentialReqAsync(commonWallet, proverDid, claimOffer, claimDef, masterSecretName)
            );
        }

        [TestMethod]
        public async Task TestProverCreateAndStoreClaimReqWorksForInvalidMasterSecret()
        {
            await InitCommonWallet();

            var claimOffer = string.Format(claimOfferTemplate, issuerDid, 2);

            var ex = await Assert.ThrowsExceptionAsync<WalletValueNotFoundException>(() =>
                AnonCreds.ProverCreateCredentialReqAsync(commonWallet, proverDid, claimOffer, claimDef, "other_master_secret")
            );
        }
    }
}
