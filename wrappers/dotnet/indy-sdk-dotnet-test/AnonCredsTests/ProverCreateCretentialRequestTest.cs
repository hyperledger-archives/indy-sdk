using Hyperledger.Indy.AnonCredsApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AnonCredsTests
{
    [TestClass]
    public class ProverCreateCretentialRequestTest : AnonCredsIntegrationTestBase
    {
        [TestMethod]
        public async Task TestProverCreateAndStoreCredentialReqWorks()
        {
        }

        [TestMethod]
        public async Task TestProverCreateAndStoreCredentialReqWorksForCredentialDefDoesNotCorrespondToCredentialOfferDifferentIssuer()
        {
            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                AnonCreds.ProverCreateCredentialReqAsync(wallet, proverDid, issuer2GvtCredOffer, issuer1gvtCredDef, masterSecretId)
            );
        }

        [TestMethod]
        public async Task TestProverCreateAndStoreCredentialReqWorksForInvalidCredentialOffer()
        {
            var credentialOffer = string.Format("{{\"issuer_did\":\"{0}\"}}", issuerDid);

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                AnonCreds.ProverCreateCredentialReqAsync(wallet, proverDid, credentialOffer, issuer1gvtCredDef, masterSecretId)
            );
        }

        [TestMethod]
        public async Task TestProverCreateAndStoreCredentialReqWorksForInvalidMasterSecret()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletItemNotFoundException>(() =>
                AnonCreds.ProverCreateCredentialReqAsync(wallet, proverDid, issuer1GvtCredOffer, issuer1gvtCredDef, masterSecretId + "a")
            );
        }
    }
}
